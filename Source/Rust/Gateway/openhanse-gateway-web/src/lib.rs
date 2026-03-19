use axum::{
    Json, Router,
    extract::{Query, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use openhanse_gateway_core::{
    ErrorResponseModel, GatewayRuntimeConfig, GatewayRuntimeHandle, InboxEntryModel,
    SendMessageRequestModel, SendMessageResponseModel, UiEventModel,
};
use openhanse_protocol::model::{
    connect_model::ConnectDecisionModel, peer_model::PeerLookupResponseModel,
};
use serde::{Deserialize, Serialize};
use std::{
    ffi::{CStr, CString, c_char},
    path::PathBuf,
    sync::{Arc, Mutex as StdMutex, OnceLock},
};
use tokio::{
    net::TcpListener,
    sync::{Mutex, watch},
    task::JoinHandle,
};
use tower_http::cors::{Any, CorsLayer};

const INDEX_HTML: &str = include_str!("../assets/WebUI/index.html");
const APP_JS: &str = include_str!("../assets/WebUI/app.js");
const APP_CSS: &str = include_str!("../assets/WebUI/app.css");
const OH_LOG_JS: &str = include_str!("../assets/WebUI/components/oh-log.js");
const OH_PROMPT_JS: &str = include_str!("../assets/WebUI/components/oh-prompt.js");
const OH_SHELL_JS: &str = include_str!("../assets/WebUI/components/oh-shell.js");
const OH_STATUS_JS: &str = include_str!("../assets/WebUI/components/oh-status.js");

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GatewayWebRuntimeConfig {
    pub peer_id: String,
    pub device_key: String,
    pub display_name: Option<String>,
    pub target_peer_id: String,
    pub server_base_url: String,
    pub direct_bind_host: String,
    pub direct_bind_port: u16,
    #[serde(default = "default_supports_direct")]
    pub supports_direct: bool,
    pub ui_bind_port: u16,
    pub heartbeat_interval_secs: u64,
    pub storage_dir: PathBuf,
}

impl GatewayWebRuntimeConfig {
    pub fn core_config(&self) -> GatewayRuntimeConfig {
        GatewayRuntimeConfig {
            peer_id: self.peer_id.clone(),
            device_key: self.device_key.clone(),
            display_name: self.display_name.clone(),
            target_peer_id: self.target_peer_id.clone(),
            server_base_url: self.server_base_url.clone(),
            direct_bind_host: self.direct_bind_host.clone(),
            direct_bind_port: self.direct_bind_port,
            supports_direct: self.supports_direct,
            heartbeat_interval_secs: self.heartbeat_interval_secs,
            storage_dir: self.storage_dir.clone(),
        }
    }
}

fn default_supports_direct() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GatewayWebRuntimeInfoModel {
    pub peer_id: String,
    pub target_peer_id: String,
    pub server_base_url: String,
    pub direct_base_url: String,
    pub message_endpoint: String,
    pub ui_base_url: String,
    pub storage_dir: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GatewayWebRuntimeStatusModel {
    pub peer_id: String,
    pub display_name: Option<String>,
    pub target_peer_id: String,
    pub server_base_url: String,
    pub direct_base_url: String,
    pub message_endpoint: String,
    pub ui_base_url: String,
    pub heartbeat_interval_secs: u64,
    pub heartbeat_state: String,
    pub last_registered_at_unix_ms: Option<u64>,
    pub last_heartbeat_at_unix_ms: Option<u64>,
    pub last_error: Option<String>,
    pub inbox_count: usize,
    pub event_count: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct EventQueryModel {
    pub since_event_id: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InboxListResponseModel {
    pub inbox: Vec<InboxEntryModel>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventsResponseModel {
    pub events: Vec<UiEventModel>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollResponseModel {
    pub status: GatewayWebRuntimeStatusModel,
    pub inbox: Vec<InboxEntryModel>,
    pub events: Vec<UiEventModel>,
}

#[derive(Clone)]
pub struct GatewayWebRuntimeHandle {
    shared: Arc<GatewayWebRuntimeShared>,
}

struct GatewayWebRuntimeShared {
    gateway_runtime: GatewayRuntimeHandle,
    ui_base_url: String,
    shutdown_tx: watch::Sender<bool>,
    tasks: Mutex<Vec<JoinHandle<()>>>,
}

#[derive(Clone)]
struct UiApiAppState {
    runtime: GatewayWebRuntimeHandle,
}

impl GatewayWebRuntimeHandle {
    pub async fn start(config: GatewayWebRuntimeConfig) -> Result<Self, String> {
        let ui_listener = TcpListener::bind(("127.0.0.1", config.ui_bind_port))
            .await
            .map_err(|error| {
                format!(
                    "failed to bind UI API on 127.0.0.1:{}: {error}",
                    config.ui_bind_port
                )
            })?;
        let ui_address = ui_listener
            .local_addr()
            .map_err(|error| format!("failed to inspect UI listener: {error}"))?;

        let gateway_runtime = GatewayRuntimeHandle::start(config.core_config()).await?;
        let (shutdown_tx, _) = watch::channel(false);
        let handle = Self {
            shared: Arc::new(GatewayWebRuntimeShared {
                gateway_runtime,
                ui_base_url: format!("http://127.0.0.1:{}", ui_address.port()),
                shutdown_tx,
                tasks: Mutex::new(Vec::new()),
            }),
        };

        handle.spawn_ui_api(ui_listener).await;
        Ok(handle)
    }

    pub async fn stop(&self) -> Result<(), String> {
        let _ = self.shared.shutdown_tx.send(true);
        let mut tasks = self.shared.tasks.lock().await;
        for task in tasks.drain(..) {
            let _ = task.await;
        }
        drop(tasks);
        self.shared.gateway_runtime.stop().await
    }

    pub fn info(&self) -> GatewayWebRuntimeInfoModel {
        let core_info = self.shared.gateway_runtime.info();
        GatewayWebRuntimeInfoModel {
            peer_id: core_info.peer_id,
            target_peer_id: core_info.target_peer_id,
            server_base_url: core_info.server_base_url,
            direct_base_url: core_info.direct_base_url,
            message_endpoint: core_info.message_endpoint,
            ui_base_url: self.shared.ui_base_url.clone(),
            storage_dir: core_info.storage_dir,
        }
    }

    pub async fn status(&self) -> GatewayWebRuntimeStatusModel {
        let core_status = self.shared.gateway_runtime.status().await;
        GatewayWebRuntimeStatusModel {
            peer_id: core_status.peer_id,
            display_name: core_status.display_name,
            target_peer_id: core_status.target_peer_id,
            server_base_url: core_status.server_base_url,
            direct_base_url: core_status.direct_base_url,
            message_endpoint: core_status.message_endpoint,
            ui_base_url: self.shared.ui_base_url.clone(),
            heartbeat_interval_secs: core_status.heartbeat_interval_secs,
            heartbeat_state: core_status.heartbeat_state,
            last_registered_at_unix_ms: core_status.last_registered_at_unix_ms,
            last_heartbeat_at_unix_ms: core_status.last_heartbeat_at_unix_ms,
            last_error: core_status.last_error,
            inbox_count: core_status.inbox_count,
            event_count: core_status.event_count,
        }
    }

    pub async fn list_inbox(&self) -> Vec<InboxEntryModel> {
        self.shared.gateway_runtime.list_inbox().await
    }

    pub async fn events_since(&self, since_event_id: Option<u64>) -> Vec<UiEventModel> {
        self.shared.gateway_runtime.events_since(since_event_id).await
    }

    pub async fn poll(&self, since_event_id: Option<u64>) -> PollResponseModel {
        PollResponseModel {
            status: self.status().await,
            inbox: self.list_inbox().await,
            events: self.events_since(since_event_id).await,
        }
    }

    pub async fn lookup_target(&self) -> Result<PeerLookupResponseModel, String> {
        self.shared.gateway_runtime.lookup_target().await
    }

    pub async fn connect_target(&self) -> Result<ConnectDecisionModel, String> {
        self.shared.gateway_runtime.connect_target().await
    }

    pub async fn send_message(
        &self,
        message: impl Into<String>,
    ) -> Result<SendMessageResponseModel, String> {
        self.shared.gateway_runtime.send_message(message).await
    }

    async fn spawn_ui_api(&self, listener: TcpListener) {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
        let app = Router::new()
            .route("/", get(index_page))
            .route("/index.html", get(index_page))
            .route("/app.js", get(app_js))
            .route("/app.css", get(app_css))
            .route("/components/oh-log.js", get(oh_log_js))
            .route("/components/oh-prompt.js", get(oh_prompt_js))
            .route("/components/oh-shell.js", get(oh_shell_js))
            .route("/components/oh-status.js", get(oh_status_js))
            .route("/api/status", get(status_endpoint))
            .route("/api/inbox", get(inbox_endpoint))
            .route("/api/events", get(events_endpoint))
            .route("/api/poll", get(poll_endpoint))
            .route("/api/lookup", post(lookup_endpoint))
            .route("/api/connect", post(connect_endpoint))
            .route("/api/messages", post(send_message_endpoint))
            .layer(cors)
            .with_state(UiApiAppState {
                runtime: self.clone(),
            });
        let mut shutdown_rx = self.shared.shutdown_tx.subscribe();
        let task = tokio::spawn(async move {
            let server = axum::serve(listener, app).with_graceful_shutdown(async move {
                let _ = shutdown_rx.changed().await;
            });
            let _ = server.await;
        });
        self.shared.tasks.lock().await.push(task);
    }
}

async fn status_endpoint(
    State(state): State<UiApiAppState>,
) -> Json<GatewayWebRuntimeStatusModel> {
    Json(state.runtime.status().await)
}

async fn inbox_endpoint(State(state): State<UiApiAppState>) -> Json<InboxListResponseModel> {
    Json(InboxListResponseModel {
        inbox: state.runtime.list_inbox().await,
    })
}

async fn events_endpoint(
    State(state): State<UiApiAppState>,
    Query(query): Query<EventQueryModel>,
) -> Json<EventsResponseModel> {
    Json(EventsResponseModel {
        events: state.runtime.events_since(query.since_event_id).await,
    })
}

async fn poll_endpoint(
    State(state): State<UiApiAppState>,
    Query(query): Query<EventQueryModel>,
) -> Json<PollResponseModel> {
    Json(state.runtime.poll(query.since_event_id).await)
}

async fn lookup_endpoint(
    State(state): State<UiApiAppState>,
) -> Result<Json<PeerLookupResponseModel>, (StatusCode, Json<ErrorResponseModel>)> {
    state.runtime.lookup_target().await.map(Json).map_err(error_response)
}

async fn connect_endpoint(
    State(state): State<UiApiAppState>,
) -> Result<Json<ConnectDecisionModel>, (StatusCode, Json<ErrorResponseModel>)> {
    state
        .runtime
        .connect_target()
        .await
        .map(Json)
        .map_err(error_response)
}

async fn send_message_endpoint(
    State(state): State<UiApiAppState>,
    Json(request): Json<SendMessageRequestModel>,
) -> Result<Json<SendMessageResponseModel>, (StatusCode, Json<ErrorResponseModel>)> {
    state
        .runtime
        .send_message(request.message)
        .await
        .map(Json)
        .map_err(error_response)
}

fn text_response(content_type: &'static str, body: &'static str) -> Response {
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static(content_type))],
        body,
    )
        .into_response()
}

async fn index_page() -> Response {
    text_response("text/html; charset=utf-8", INDEX_HTML)
}

async fn app_js() -> Response {
    text_response("text/javascript; charset=utf-8", APP_JS)
}

async fn app_css() -> Response {
    text_response("text/css; charset=utf-8", APP_CSS)
}

async fn oh_log_js() -> Response {
    text_response("text/javascript; charset=utf-8", OH_LOG_JS)
}

async fn oh_prompt_js() -> Response {
    text_response("text/javascript; charset=utf-8", OH_PROMPT_JS)
}

async fn oh_shell_js() -> Response {
    text_response("text/javascript; charset=utf-8", OH_SHELL_JS)
}

async fn oh_status_js() -> Response {
    text_response("text/javascript; charset=utf-8", OH_STATUS_JS)
}

fn error_response(message: String) -> (StatusCode, Json<ErrorResponseModel>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponseModel { error: message }),
    )
}

struct HostRuntime {
    tokio_runtime: tokio::runtime::Runtime,
    gateway_runtime: GatewayWebRuntimeHandle,
}

#[derive(Serialize)]
struct BridgeResponse<T: Serialize> {
    ok: bool,
    data: Option<T>,
    error: Option<String>,
}

static HOST_RUNTIME: OnceLock<StdMutex<Option<HostRuntime>>> = OnceLock::new();

fn runtime_slot() -> &'static StdMutex<Option<HostRuntime>> {
    HOST_RUNTIME.get_or_init(|| StdMutex::new(None))
}

#[unsafe(no_mangle)]
pub extern "C" fn openhanse_start(config_json: *const c_char) -> *mut c_char {
    let response = match parse_config(config_json) {
        Ok(config) => match tokio::runtime::Runtime::new() {
            Ok(tokio_runtime) => {
                let gateway_runtime =
                    match tokio_runtime.block_on(GatewayWebRuntimeHandle::start(config)) {
                        Ok(gateway_runtime) => gateway_runtime,
                        Err(error) => {
                            return response_to_c_string(BridgeResponse::<GatewayWebRuntimeInfoModel> {
                                ok: false,
                                data: None,
                                error: Some(error),
                            });
                        }
                    };
                let info = gateway_runtime.info();
                match runtime_slot().lock() {
                    Ok(mut slot) => {
                        if let Some(existing) = slot.take() {
                            let _ = existing
                                .tokio_runtime
                                .block_on(existing.gateway_runtime.stop());
                        }
                        *slot = Some(HostRuntime {
                            tokio_runtime,
                            gateway_runtime,
                        });
                        response_to_c_string(BridgeResponse {
                            ok: true,
                            data: Some(info),
                            error: None,
                        })
                    }
                    Err(error) => response_to_c_string(BridgeResponse::<GatewayWebRuntimeInfoModel> {
                        ok: false,
                        data: None,
                        error: Some(format!("failed to lock runtime: {error}")),
                    }),
                }
            }
            Err(error) => response_to_c_string(BridgeResponse::<GatewayWebRuntimeInfoModel> {
                ok: false,
                data: None,
                error: Some(format!("failed to create tokio runtime: {error}")),
            }),
        },
        Err(error) => response_to_c_string(BridgeResponse::<GatewayWebRuntimeInfoModel> {
            ok: false,
            data: None,
            error: Some(error),
        }),
    };

    response
}

#[unsafe(no_mangle)]
pub extern "C" fn openhanse_runtime_status() -> *mut c_char {
    match runtime_slot().lock() {
        Ok(slot) => {
            let Some(runtime) = slot.as_ref() else {
                return response_to_c_string(BridgeResponse::<serde_json::Value> {
                    ok: false,
                    data: None,
                    error: Some("runtime is not running".to_string()),
                });
            };
            let status = runtime.tokio_runtime.block_on(runtime.gateway_runtime.status());
            response_to_c_string(BridgeResponse {
                ok: true,
                data: Some(status),
                error: None,
            })
        }
        Err(error) => response_to_c_string(BridgeResponse::<serde_json::Value> {
            ok: false,
            data: None,
            error: Some(format!("failed to lock runtime: {error}")),
        }),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn openhanse_stop() {
    if let Ok(mut slot) = runtime_slot().lock()
        && let Some(runtime) = slot.take()
    {
        let _ = runtime
            .tokio_runtime
            .block_on(runtime.gateway_runtime.stop());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn openhanse_string_free(value: *mut c_char) {
    if !value.is_null() {
        unsafe {
            drop(CString::from_raw(value));
        }
    }
}

fn parse_config(config_json: *const c_char) -> Result<GatewayWebRuntimeConfig, String> {
    if config_json.is_null() {
        return Err("config_json must not be null".to_string());
    }

    let raw = unsafe { CStr::from_ptr(config_json) }
        .to_str()
        .map_err(|error| format!("invalid config string: {error}"))?;
    serde_json::from_str(raw).map_err(|error| format!("invalid config JSON: {error}"))
}

fn response_to_c_string<T: Serialize>(response: BridgeResponse<T>) -> *mut c_char {
    let json = serde_json::to_string(&response).unwrap_or_else(|error| {
        format!(
            "{{\"ok\":false,\"data\":null,\"error\":\"failed to serialize response: {error}\"}}"
        )
    });
    CString::new(json)
        .expect("bridge response must not contain interior nul bytes")
        .into_raw()
}
