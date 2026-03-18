use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::post,
};
use openhanse_protocol::model::{
    connect_model::ConnectDecisionModel,
    message_model::ChatMessageEnvelopeModel,
    peer_model::{
        HeartbeatRequestModel, PeerLookupResponseModel, RegisterPeerRequestModel,
        RegisterPeerResponseModel,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    fs::{self, OpenOptions},
    hash::{Hash, Hasher},
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    net::TcpListener,
    sync::{Mutex, RwLock, broadcast, watch},
    task::JoinHandle,
};

const DEFAULT_MESSAGE_ENDPOINT: &str = "/message";
const DEFAULT_HEARTBEAT_INTERVAL_SECS: u64 = 10;
const MAX_EVENTS: usize = 256;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GatewayProfile {
    pub peer_id: String,
    pub device_key: String,
    pub display_name: Option<String>,
    pub direct_addresses: Vec<String>,
    pub message_endpoint: Option<String>,
    pub supports_direct: bool,
}

impl GatewayProfile {
    pub fn register_request(&self) -> RegisterPeerRequestModel {
        RegisterPeerRequestModel {
            peer_id: self.peer_id.clone(),
            device_key: self.device_key.clone(),
            display_name: self.display_name.clone(),
            direct_addresses: self.direct_addresses.clone(),
            message_endpoint: self.message_endpoint.clone(),
            supports_direct: self.supports_direct,
        }
    }

    pub fn heartbeat_request(&self) -> HeartbeatRequestModel {
        HeartbeatRequestModel {
            peer_id: self.peer_id.clone(),
        }
    }

    pub fn connect_request(
        &self,
        target_peer_id: impl Into<String>,
        prefer_direct: bool,
    ) -> openhanse_protocol::model::connect_model::ConnectRequestModel {
        openhanse_protocol::model::connect_model::ConnectRequestModel {
            source_peer_id: self.peer_id.clone(),
            target_peer_id: target_peer_id.into(),
            prefer_direct,
        }
    }

    pub fn outbound_message(
        &self,
        target_peer_id: impl Into<String>,
        message: impl Into<String>,
        sent_at_unix_ms: u64,
    ) -> ChatMessageEnvelopeModel {
        ChatMessageEnvelopeModel {
            from_peer_id: self.peer_id.clone(),
            to_peer_id: target_peer_id.into(),
            message: message.into(),
            sent_at_unix_ms,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum DeliveryPreference {
    DirectFirst,
    RelayOnly,
}

impl DeliveryPreference {
    pub fn prefer_direct(self) -> bool {
        matches!(self, Self::DirectFirst)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GatewayRuntimeConfig {
    pub peer_id: String,
    pub device_key: String,
    pub display_name: Option<String>,
    pub target_peer_id: String,
    pub server_base_url: String,
    pub direct_bind_host: String,
    pub direct_bind_port: u16,
    pub heartbeat_interval_secs: u64,
    pub storage_dir: PathBuf,
}

impl GatewayRuntimeConfig {
    pub fn from_profile(
        profile: GatewayProfile,
        target_peer_id: String,
        server_base_url: String,
        direct_bind_host: String,
        direct_bind_port: u16,
        heartbeat_interval_secs: u64,
        storage_dir: PathBuf,
    ) -> Self {
        Self {
            peer_id: profile.peer_id,
            device_key: profile.device_key,
            display_name: profile.display_name,
            target_peer_id,
            server_base_url,
            direct_bind_host,
            direct_bind_port,
            heartbeat_interval_secs,
            storage_dir,
        }
    }

    pub fn normalized(mut self) -> Self {
        self.server_base_url = normalize_server_base_url(&self.server_base_url);
        if self.heartbeat_interval_secs == 0 {
            self.heartbeat_interval_secs = DEFAULT_HEARTBEAT_INTERVAL_SECS;
        }
        self
    }

    pub fn profile(&self) -> GatewayProfile {
        GatewayProfile {
            peer_id: self.peer_id.clone(),
            device_key: self.device_key.clone(),
            display_name: self.display_name.clone(),
            direct_addresses: vec![format!(
                "http://{}:{}",
                self.direct_bind_host, self.direct_bind_port
            )],
            message_endpoint: Some(DEFAULT_MESSAGE_ENDPOINT.to_string()),
            supports_direct: true,
        }
    }

    pub fn inbox_file(&self) -> PathBuf {
        self.storage_dir.join("inbox.jsonl")
    }

    pub fn events_file(&self) -> PathBuf {
        self.storage_dir.join("events.jsonl")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GatewayRuntimeInfoModel {
    pub peer_id: String,
    pub target_peer_id: String,
    pub server_base_url: String,
    pub direct_base_url: String,
    pub message_endpoint: String,
    pub storage_dir: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GatewayRuntimeStatusModel {
    pub peer_id: String,
    pub display_name: Option<String>,
    pub target_peer_id: String,
    pub server_base_url: String,
    pub direct_base_url: String,
    pub message_endpoint: String,
    pub heartbeat_interval_secs: u64,
    pub heartbeat_state: String,
    pub last_registered_at_unix_ms: Option<u64>,
    pub last_heartbeat_at_unix_ms: Option<u64>,
    pub last_error: Option<String>,
    pub inbox_count: usize,
    pub event_count: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InboxEntryModel {
    pub received_at_unix_ms: u64,
    pub peer_id: String,
    pub payload: ChatMessageEnvelopeModel,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiEventModel {
    pub id: u64,
    pub kind: String,
    pub message: String,
    pub created_at_unix_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMessageResponseModel {
    pub accepted: bool,
    pub delivery_mode: String,
    pub target_url: String,
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
pub struct ErrorResponseModel {
    pub error: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMessageRequestModel {
    pub message: String,
}

#[derive(Clone)]
pub struct GatewayRuntimeHandle {
    shared: Arc<GatewayRuntimeShared>,
}

struct GatewayRuntimeShared {
    config: GatewayRuntimeConfig,
    profile: GatewayProfile,
    direct_base_url: String,
    state: RwLock<GatewayRuntimeState>,
    hub_client: HubClient,
    shutdown_tx: watch::Sender<bool>,
    event_tx: broadcast::Sender<UiEventModel>,
    tasks: Mutex<Vec<JoinHandle<()>>>,
}

struct GatewayRuntimeState {
    last_registered_at_unix_ms: Option<u64>,
    last_heartbeat_at_unix_ms: Option<u64>,
    lease_seconds: Option<u64>,
    last_error: Option<String>,
    inbox: Vec<InboxEntryModel>,
    events: Vec<UiEventModel>,
    next_event_id: u64,
}

#[derive(Clone)]
struct DirectMessageAppState {
    runtime: GatewayRuntimeHandle,
}

impl GatewayRuntimeHandle {
    pub async fn start(config: GatewayRuntimeConfig) -> Result<Self, String> {
        let config = config.normalized();
        fs::create_dir_all(&config.storage_dir).map_err(|error| error.to_string())?;

        let direct_listener =
            bind_direct_listener(&config.direct_bind_host, config.direct_bind_port).await?;
        let direct_address = direct_listener
            .local_addr()
            .map_err(|error| format!("failed to inspect direct listener: {error}"))?;

        let profile = GatewayProfile {
            direct_addresses: vec![format!("http://{}:{}", config.direct_bind_host, direct_address.port())],
            ..config.profile()
        };
        let inbox = load_json_lines::<InboxEntryModel>(&config.inbox_file())?;
        let events = load_json_lines::<UiEventModel>(&config.events_file())?;
        let next_event_id = events.last().map(|event| event.id + 1).unwrap_or(1);
        let hub_client = HubClient::new(config.server_base_url.clone())?;
        let (shutdown_tx, _) = watch::channel(false);
        let (event_tx, _) = broadcast::channel(128);

        let shared = Arc::new(GatewayRuntimeShared {
            config: config.clone(),
            profile: profile.clone(),
            direct_base_url: format!("http://{}:{}", config.direct_bind_host, direct_address.port()),
            state: RwLock::new(GatewayRuntimeState {
                last_registered_at_unix_ms: None,
                last_heartbeat_at_unix_ms: None,
                lease_seconds: None,
                last_error: None,
                inbox,
                events,
                next_event_id,
            }),
            hub_client,
            shutdown_tx,
            event_tx,
            tasks: Mutex::new(Vec::new()),
        });
        let handle = Self { shared };

        handle
            .record_event(
                "runtime_started",
                format!(
                    "Runtime started for {} targeting {}.",
                    handle.shared.profile.peer_id, handle.shared.config.target_peer_id
                ),
            )
            .await?;

        handle.spawn_direct_receiver(direct_listener).await;
        handle.register().await?;
        handle.spawn_heartbeat_loop().await;

        Ok(handle)
    }

    pub async fn stop(&self) -> Result<(), String> {
        let _ = self.shared.shutdown_tx.send(true);
        let mut tasks = self.shared.tasks.lock().await;
        for task in tasks.drain(..) {
            let _ = task.await;
        }
        self.record_event(
            "runtime_stopped",
            format!("Runtime stopped for {}.", self.shared.profile.peer_id),
        )
        .await?;
        Ok(())
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<UiEventModel> {
        self.shared.event_tx.subscribe()
    }

    pub async fn status(&self) -> GatewayRuntimeStatusModel {
        let state = self.shared.state.read().await;
        GatewayRuntimeStatusModel {
            peer_id: self.shared.profile.peer_id.clone(),
            display_name: self.shared.profile.display_name.clone(),
            target_peer_id: self.shared.config.target_peer_id.clone(),
            server_base_url: self.shared.config.server_base_url.clone(),
            direct_base_url: self.shared.direct_base_url.clone(),
            message_endpoint: self
                .shared
                .profile
                .message_endpoint
                .clone()
                .unwrap_or_else(|| DEFAULT_MESSAGE_ENDPOINT.to_string()),
            heartbeat_interval_secs: self.shared.config.heartbeat_interval_secs,
            heartbeat_state: heartbeat_state(&state),
            last_registered_at_unix_ms: state.last_registered_at_unix_ms,
            last_heartbeat_at_unix_ms: state.last_heartbeat_at_unix_ms,
            last_error: state.last_error.clone(),
            inbox_count: state.inbox.len(),
            event_count: state.events.len(),
        }
    }

    pub fn info(&self) -> GatewayRuntimeInfoModel {
        GatewayRuntimeInfoModel {
            peer_id: self.shared.profile.peer_id.clone(),
            target_peer_id: self.shared.config.target_peer_id.clone(),
            server_base_url: self.shared.config.server_base_url.clone(),
            direct_base_url: self.shared.direct_base_url.clone(),
            message_endpoint: self
                .shared
                .profile
                .message_endpoint
                .clone()
                .unwrap_or_else(|| DEFAULT_MESSAGE_ENDPOINT.to_string()),
            storage_dir: self.shared.config.storage_dir.to_string_lossy().to_string(),
        }
    }

    pub async fn list_inbox(&self) -> Vec<InboxEntryModel> {
        self.shared.state.read().await.inbox.clone()
    }

    pub async fn events_since(&self, since_event_id: Option<u64>) -> Vec<UiEventModel> {
        let state = self.shared.state.read().await;
        match since_event_id {
            Some(since_event_id) => state
                .events
                .iter()
                .filter(|event| event.id > since_event_id)
                .cloned()
                .collect(),
            None => state.events.clone(),
        }
    }

    pub async fn lookup_target(&self) -> Result<PeerLookupResponseModel, String> {
        self.record_event(
            "lookup_started",
            format!("Looking up {}.", self.shared.config.target_peer_id),
        )
        .await?;
        match self
            .shared
            .hub_client
            .lookup(&self.shared.config.target_peer_id)
            .await
        {
            Ok(response) => {
                self.record_event(
                    "lookup_succeeded",
                    format!("{} is online.", response.peer.peer_id),
                )
                .await?;
                Ok(response)
            }
            Err(error) => {
                self.record_error(&error).await?;
                Err(error)
            }
        }
    }

    pub async fn connect_target(&self) -> Result<ConnectDecisionModel, String> {
        self.record_event(
            "connect_started",
            format!(
                "Requesting connect decision for {}.",
                self.shared.config.target_peer_id
            ),
        )
        .await?;
        match self
            .shared
            .hub_client
            .connect(
                &self.shared.profile,
                &self.shared.config.target_peer_id,
                DeliveryPreference::DirectFirst,
            )
            .await
        {
            Ok(decision) => {
                let message = match &decision {
                    ConnectDecisionModel::Direct { direct } => format!(
                        "Connect decision: direct to {} via {}.",
                        direct.peer_id,
                        direct
                            .direct_addresses
                            .first()
                            .cloned()
                            .unwrap_or_else(|| "<missing>".to_string())
                    ),
                    ConnectDecisionModel::Relay { relay } => format!(
                        "Connect decision: relay {} for {} -> {}.",
                        relay.relay_session_id, relay.source_peer_id, relay.target_peer_id
                    ),
                };
                self.record_event("connect_succeeded", message).await?;
                Ok(decision)
            }
            Err(error) => {
                self.record_error(&error).await?;
                Err(error)
            }
        }
    }

    pub async fn send_message(&self, message: impl Into<String>) -> Result<SendMessageResponseModel, String> {
        let message = message.into();
        let decision = self.connect_target().await?;
        match decision {
            ConnectDecisionModel::Direct { direct } => {
                let direct_address = direct.direct_addresses.first().cloned().ok_or_else(|| {
                    format!("target peer '{}' has no direct address", direct.peer_id)
                })?;
                let target_url = join_url(
                    &direct_address,
                    direct
                        .message_endpoint
                        .as_deref()
                        .unwrap_or(DEFAULT_MESSAGE_ENDPOINT),
                );
                let payload = self.shared.profile.outbound_message(
                    self.shared.config.target_peer_id.clone(),
                    message.clone(),
                    current_unix_ms(),
                );
                match self
                    .shared
                    .hub_client
                    .post_direct_message(&target_url, &payload)
                    .await
                {
                    Ok(_) => {
                        self.record_event(
                            "message_sent",
                            format!("Sent message to {}.", self.shared.config.target_peer_id),
                        )
                        .await?;
                        Ok(SendMessageResponseModel {
                            accepted: true,
                            delivery_mode: "direct".to_string(),
                            target_url,
                        })
                    }
                    Err(error) => {
                        self.record_error(&error).await?;
                        Err(error)
                    }
                }
            }
            ConnectDecisionModel::Relay { .. } => {
                let error =
                    "relay transfer is not implemented yet in the Phase 1 Rust gateway".to_string();
                self.record_error(&error).await?;
                Err(error)
            }
        }
    }

    pub async fn register(&self) -> Result<RegisterPeerResponseModel, String> {
        match self.shared.hub_client.register(&self.shared.profile).await {
            Ok(response) => {
                let mut state = self.shared.state.write().await;
                state.last_registered_at_unix_ms = Some(current_unix_ms());
                state.last_heartbeat_at_unix_ms = state.last_registered_at_unix_ms;
                state.lease_seconds = Some(response.lease_seconds);
                state.last_error = None;
                drop(state);
                self.record_event(
                    "register_succeeded",
                    format!("Registered {}.", self.shared.profile.peer_id),
                )
                .await?;
                Ok(response)
            }
            Err(error) => {
                self.record_error(&error).await?;
                Err(error)
            }
        }
    }

    async fn heartbeat(&self) -> Result<RegisterPeerResponseModel, String> {
        match self.shared.hub_client.heartbeat(&self.shared.profile).await {
            Ok(response) => {
                let mut state = self.shared.state.write().await;
                state.last_heartbeat_at_unix_ms = Some(current_unix_ms());
                state.lease_seconds = Some(response.lease_seconds);
                state.last_error = None;
                Ok(response)
            }
            Err(error) => {
                self.record_error(&error).await?;
                Err(error)
            }
        }
    }

    async fn receive_message(
        &self,
        payload: ChatMessageEnvelopeModel,
    ) -> Result<AcceptedResponse, String> {
        let entry = InboxEntryModel {
            received_at_unix_ms: current_unix_ms(),
            peer_id: self.shared.profile.peer_id.clone(),
            payload: payload.clone(),
        };
        append_json_line(&self.shared.config.inbox_file(), &entry).map_err(|error| error.to_string())?;
        {
            let mut state = self.shared.state.write().await;
            state.inbox.push(entry);
        }
        self.record_event(
            "message_received",
            format!(
                "Received message from {}: {}",
                payload.from_peer_id, payload.message
            ),
        )
        .await?;

        Ok(AcceptedResponse {
            status: "accepted".to_string(),
            peer_id: self.shared.profile.peer_id.clone(),
        })
    }

    async fn spawn_direct_receiver(&self, listener: TcpListener) {
        let app = Router::new()
            .route(DEFAULT_MESSAGE_ENDPOINT, post(receive_message_endpoint))
            .with_state(DirectMessageAppState {
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

    async fn spawn_heartbeat_loop(&self) {
        let runtime = self.clone();
        let mut shutdown_rx = self.shared.shutdown_tx.subscribe();
        let interval_secs = self.shared.config.heartbeat_interval_secs;
        let task = tokio::spawn(async move {
            let duration = Duration::from_secs(interval_secs);
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(duration) => {
                        if let Err(error) = runtime.heartbeat().await {
                            let _ = runtime.record_error(&error).await;
                        }
                    }
                    _ = shutdown_rx.changed() => break,
                }
            }
        });
        self.shared.tasks.lock().await.push(task);
    }

    async fn record_error(&self, error: &str) -> Result<(), String> {
        {
            let mut state = self.shared.state.write().await;
            state.last_error = Some(error.to_string());
        }
        self.record_event("error", error.to_string()).await
    }

    async fn record_event(&self, kind: impl Into<String>, message: impl Into<String>) -> Result<(), String> {
        let event = {
            let mut state = self.shared.state.write().await;
            let event = UiEventModel {
                id: state.next_event_id,
                kind: kind.into(),
                message: message.into(),
                created_at_unix_ms: current_unix_ms(),
            };
            state.next_event_id += 1;
            state.events.push(event.clone());
            if state.events.len() > MAX_EVENTS {
                let overflow = state.events.len() - MAX_EVENTS;
                state.events.drain(0..overflow);
            }
            event
        };
        append_json_line(&self.shared.config.events_file(), &event).map_err(|error| error.to_string())?;
        let _ = self.shared.event_tx.send(event);
        Ok(())
    }
}

async fn bind_direct_listener(bind_host: &str, bind_port: u16) -> Result<TcpListener, String> {
    let mut attempts = Vec::new();
    for candidate in direct_bind_candidates(bind_host, bind_port) {
        match TcpListener::bind((candidate.as_str(), bind_port)).await {
            Ok(listener) => return Ok(listener),
            Err(error) => attempts.push(format!("{candidate}:{bind_port}: {error}")),
        }
    }

    Err(format!(
        "failed to bind direct receiver on {bind_host}:{bind_port} ({})",
        attempts.join("; ")
    ))
}

fn direct_bind_candidates(bind_host: &str, bind_port: u16) -> Vec<String> {
    let mut candidates = vec![bind_host.to_string()];
    if bind_port == 0 && !is_loopback_host(bind_host) && bind_host != "0.0.0.0" {
        candidates.push("0.0.0.0".to_string());
    }
    candidates
}

fn is_loopback_host(host: &str) -> bool {
    matches!(host, "127.0.0.1" | "localhost" | "::1")
}

async fn receive_message_endpoint(
    State(state): State<DirectMessageAppState>,
    Json(payload): Json<ChatMessageEnvelopeModel>,
) -> Result<Json<AcceptedResponse>, (StatusCode, Json<ErrorResponseModel>)> {
    state
        .runtime
        .receive_message(payload)
        .await
        .map(Json)
        .map_err(internal_string_error)
}

#[derive(Deserialize, Serialize)]
struct AcceptedResponse {
    status: String,
    peer_id: String,
}

#[derive(Clone)]
struct HubClient {
    http_client: reqwest::Client,
    server_base_url: String,
}

impl HubClient {
    fn new(server_base_url: String) -> Result<Self, String> {
        let http_client = reqwest::Client::builder()
            .build()
            .map_err(|error| error.to_string())?;

        Ok(Self {
            http_client,
            server_base_url,
        })
    }

    async fn register(
        &self,
        profile: &GatewayProfile,
    ) -> Result<RegisterPeerResponseModel, String> {
        self.post_json("/v1/peers/register", &profile.register_request())
            .await
    }

    async fn heartbeat(
        &self,
        profile: &GatewayProfile,
    ) -> Result<RegisterPeerResponseModel, String> {
        self.post_json("/v1/peers/heartbeat", &profile.heartbeat_request())
            .await
    }

    async fn lookup(&self, target_peer_id: &str) -> Result<PeerLookupResponseModel, String> {
        let url = format!("{}/v1/peers/{}", self.server_base_url, target_peer_id);
        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(|error| error.to_string())?;
        decode_json_response(response).await
    }

    async fn connect(
        &self,
        profile: &GatewayProfile,
        target_peer_id: &str,
        preference: DeliveryPreference,
    ) -> Result<ConnectDecisionModel, String> {
        self.post_json(
            "/v1/connect",
            &profile.connect_request(target_peer_id.to_string(), preference.prefer_direct()),
        )
        .await
    }

    async fn post_direct_message(
        &self,
        target_url: &str,
        payload: &ChatMessageEnvelopeModel,
    ) -> Result<AcceptedResponse, String> {
        let response = self
            .http_client
            .post(target_url)
            .json(payload)
            .send()
            .await
            .map_err(|error| error.to_string())?;
        decode_json_response(response).await
    }

    async fn post_json<Request: Serialize, Response: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        payload: &Request,
    ) -> Result<Response, String> {
        let url = format!("{}{}", self.server_base_url, path);
        let response = self
            .http_client
            .post(url)
            .json(payload)
            .send()
            .await
            .map_err(|error| error.to_string())?;
        decode_json_response(response).await
    }
}

async fn decode_json_response<Response: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<Response, String> {
    let status = response.status();
    let body = response.text().await.map_err(|error| error.to_string())?;

    if !status.is_success() {
        if body.is_empty() {
            return Err(format!("HTTP {}", status));
        }
        return Err(format!("HTTP {}: {}", status, body));
    }

    serde_json::from_str(&body).map_err(|error| format!("invalid response body: {error}"))
}

fn heartbeat_state(state: &GatewayRuntimeState) -> String {
    if state.last_error.is_some() {
        return "error".to_string();
    }
    if state.last_heartbeat_at_unix_ms.is_some() {
        return "online".to_string();
    }
    if state.last_registered_at_unix_ms.is_some() {
        return "registered".to_string();
    }
    "starting".to_string()
}

fn append_json_line<T: Serialize>(path: &Path, value: &T) -> Result<(), std::io::Error> {
    ensure_parent_dir(path).map_err(std::io::Error::other)?;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    use std::io::Write;

    serde_json::to_writer(&mut file, value)?;
    file.write_all(b"\n")?;
    Ok(())
}

fn load_json_lines<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Vec<T>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).map_err(|error| error.to_string()))
        .collect()
}

fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn internal_string_error(error: String) -> (StatusCode, Json<ErrorResponseModel>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponseModel { error }),
    )
}

pub fn normalize_server_base_url(value: &str) -> String {
    value.trim_end_matches('/').to_string()
}

pub fn join_url(base: &str, suffix: &str) -> String {
    if suffix.starts_with("http://") || suffix.starts_with("https://") {
        return suffix.to_string();
    }

    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        suffix.trim_start_matches('/')
    )
}

pub fn default_port_for_peer(peer_id: &str) -> u16 {
    match peer_id {
        "gateway-a" => 17441,
        "gateway-b" => 17442,
        _ => {
            let mut hasher = DefaultHasher::new();
            peer_id.hash(&mut hasher);
            20000 + (hasher.finish() % 20000) as u16
        }
    }
}

pub fn display_name_for_peer(peer_id: &str) -> String {
    let mut chars = peer_id.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => "OpenHanse Peer".to_string(),
    }
}

pub fn current_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

pub fn default_runtime_storage_dir(root: &Path, peer_id: &str) -> PathBuf {
    root.join("peers").join(peer_id)
}

pub fn socket_addr_port(address: &str) -> Option<u16> {
    address.parse::<SocketAddr>().ok().map(|socket| socket.port())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json, Router, extract::Path, routing::get};
    use openhanse_protocol::model::{
        connect_model::{ConnectDecisionModel, DirectConnectionInfoModel},
        peer_model::{PeerLookupResponseModel, PeerRecordModel},
    };
    use std::{collections::HashMap, sync::Arc};
    use tokio::sync::RwLock as TokioRwLock;

    #[test]
    fn builds_register_request_from_profile() {
        let profile = gateway_profile("gateway-a", 7443);
        let request = profile.register_request();

        assert_eq!(request.peer_id, "gateway-a");
        assert_eq!(request.message_endpoint.as_deref(), Some("/message"));
    }

    #[test]
    fn relay_only_disables_direct_preference() {
        assert!(DeliveryPreference::DirectFirst.prefer_direct());
        assert!(!DeliveryPreference::RelayOnly.prefer_direct());
    }

    #[test]
    fn normalizes_join_url() {
        assert_eq!(join_url("http://127.0.0.1:8080/", "/message"), "http://127.0.0.1:8080/message");
    }

    #[tokio::test]
    async fn direct_message_flow_works_between_two_runtimes() {
        let hub = TestHub::start().await;
        let runtime_a = GatewayRuntimeHandle::start(runtime_config(
            "gateway-a",
            "gateway-b",
            &hub.base_url,
            unique_temp_dir("gateway-a"),
        ))
        .await
        .expect("start runtime a");
        let runtime_b = GatewayRuntimeHandle::start(runtime_config(
            "gateway-b",
            "gateway-a",
            &hub.base_url,
            unique_temp_dir("gateway-b"),
        ))
        .await
        .expect("start runtime b");

        let send = runtime_a
            .send_message("hello from runtime a")
            .await
            .expect("send direct message");
        assert_eq!(send.delivery_mode, "direct");

        tokio::time::sleep(Duration::from_millis(200)).await;

        let inbox = runtime_b.list_inbox().await;
        assert_eq!(inbox.len(), 1);
        assert_eq!(inbox[0].payload.message, "hello from runtime a");

        runtime_a.stop().await.expect("stop runtime a");
        runtime_b.stop().await.expect("stop runtime b");
    }

    #[test]
    fn direct_bind_candidates_keep_loopback_hosts() {
        assert_eq!(direct_bind_candidates("127.0.0.1", 0), vec!["127.0.0.1"]);
    }

    #[test]
    fn direct_bind_candidates_fallback_to_any_for_ephemeral_lan_bind() {
        assert_eq!(
            direct_bind_candidates("192.168.1.105", 0),
            vec!["192.168.1.105", "0.0.0.0"]
        );
    }

    fn gateway_profile(peer_id: &str, port: u16) -> GatewayProfile {
        GatewayProfile {
            peer_id: peer_id.to_string(),
            device_key: format!("device-key-{peer_id}"),
            display_name: Some(display_name_for_peer(peer_id)),
            direct_addresses: vec![format!("http://127.0.0.1:{port}")],
            message_endpoint: Some("/message".to_string()),
            supports_direct: true,
        }
    }

    fn runtime_config(
        peer_id: &str,
        target_peer_id: &str,
        server_base_url: &str,
        storage_dir: PathBuf,
    ) -> GatewayRuntimeConfig {
        GatewayRuntimeConfig {
            peer_id: peer_id.to_string(),
            device_key: format!("device-key-{peer_id}"),
            display_name: Some(display_name_for_peer(peer_id)),
            target_peer_id: target_peer_id.to_string(),
            server_base_url: server_base_url.to_string(),
            direct_bind_host: "127.0.0.1".to_string(),
            direct_bind_port: 0,
            heartbeat_interval_secs: 60,
            storage_dir,
        }
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "openhanse-gateway-test-{label}-{}",
            current_unix_ms()
        ))
    }

    #[derive(Clone)]
    struct TestHubState {
        peers: Arc<TokioRwLock<HashMap<String, PeerRecordModel>>>,
    }

    struct TestHub {
        base_url: String,
    }

    impl TestHub {
        async fn start() -> Self {
            let state = TestHubState {
                peers: Arc::new(TokioRwLock::new(HashMap::new())),
            };
            let app = Router::new()
                .route("/v1/peers/register", post(test_register_peer))
                .route("/v1/peers/heartbeat", post(test_heartbeat_peer))
                .route("/v1/peers/{peer_id}", get(test_lookup_peer))
                .route("/v1/connect", post(test_connect_peer))
                .with_state(state);
            let listener = TcpListener::bind(("127.0.0.1", 0))
                .await
                .expect("bind test hub");
            let address = listener.local_addr().expect("test hub local addr");
            tokio::spawn(async move {
                let _ = axum::serve(listener, app).await;
            });
            Self {
                base_url: format!("http://127.0.0.1:{}", address.port()),
            }
        }
    }

    async fn test_register_peer(
        State(state): State<TestHubState>,
        Json(request): Json<RegisterPeerRequestModel>,
    ) -> Json<RegisterPeerResponseModel> {
        let peer = PeerRecordModel {
            peer_id: request.peer_id.clone(),
            device_key: request.device_key.clone(),
            display_name: request.display_name.clone(),
            direct_addresses: request.direct_addresses.clone(),
            message_endpoint: request.message_endpoint.clone(),
            supports_direct: request.supports_direct,
            registered_at_unix_ms: current_unix_ms(),
            expires_at_unix_ms: current_unix_ms() + 30_000,
        };
        state
            .peers
            .write()
            .await
            .insert(request.peer_id.clone(), peer.clone());

        Json(RegisterPeerResponseModel {
            lease_seconds: 30,
            peer,
        })
    }

    async fn test_heartbeat_peer(
        State(state): State<TestHubState>,
        Json(request): Json<HeartbeatRequestModel>,
    ) -> Result<Json<RegisterPeerResponseModel>, StatusCode> {
        let mut peers = state.peers.write().await;
        let Some(peer) = peers.get_mut(&request.peer_id) else {
            return Err(StatusCode::NOT_FOUND);
        };
        peer.expires_at_unix_ms = current_unix_ms() + 30_000;
        Ok(Json(RegisterPeerResponseModel {
            lease_seconds: 30,
            peer: peer.clone(),
        }))
    }

    async fn test_lookup_peer(
        State(state): State<TestHubState>,
        Path(peer_id): Path<String>,
    ) -> Result<Json<PeerLookupResponseModel>, StatusCode> {
        let peers = state.peers.read().await;
        let Some(peer) = peers.get(&peer_id) else {
            return Err(StatusCode::NOT_FOUND);
        };
        Ok(Json(PeerLookupResponseModel { peer: peer.clone() }))
    }

    async fn test_connect_peer(
        State(state): State<TestHubState>,
        Json(request): Json<openhanse_protocol::model::connect_model::ConnectRequestModel>,
    ) -> Result<Json<ConnectDecisionModel>, StatusCode> {
        let peers = state.peers.read().await;
        let Some(peer) = peers.get(&request.target_peer_id) else {
            return Err(StatusCode::NOT_FOUND);
        };
        Ok(Json(ConnectDecisionModel::Direct {
            direct: DirectConnectionInfoModel {
                peer_id: peer.peer_id.clone(),
                device_key: peer.device_key.clone(),
                display_name: peer.display_name.clone(),
                direct_addresses: peer.direct_addresses.clone(),
                message_endpoint: peer.message_endpoint.clone(),
            },
        }))
    }
}
