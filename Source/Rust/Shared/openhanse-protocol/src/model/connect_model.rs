use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConnectRequestModel {
    pub source_peer_id: String,
    pub target_peer_id: String,
    #[serde(default = "default_true")]
    pub prefer_direct: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum ConnectDecisionModel {
    Direct {
        direct: DirectConnectionInfoModel,
    },
    Relay {
        relay: RelayConnectionInfoModel,
    },
}

impl ConnectDecisionModel {
    pub fn direct(direct: DirectConnectionInfoModel) -> Self {
        Self::Direct { direct }
    }

    pub fn relay(relay: RelayConnectionInfoModel) -> Self {
        Self::Relay { relay }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirectConnectionInfoModel {
    pub peer_id: String,
    pub device_key: String,
    pub display_name: Option<String>,
    pub direct_addresses: Vec<String>,
    pub message_endpoint: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelayConnectionInfoModel {
    pub relay_session_id: Uuid,
    pub source_peer_id: String,
    pub target_peer_id: String,
    pub expires_at_unix_ms: u64,
}

fn default_true() -> bool {
    true
}
