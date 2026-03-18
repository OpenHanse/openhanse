use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterPeerRequestModel {
    pub peer_id: String,
    pub device_key: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub direct_addresses: Vec<String>,
    #[serde(default)]
    pub message_endpoint: Option<String>,
    #[serde(default)]
    pub supports_direct: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HeartbeatRequestModel {
    pub peer_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PeerRecordModel {
    pub peer_id: String,
    pub device_key: String,
    pub display_name: Option<String>,
    pub direct_addresses: Vec<String>,
    pub message_endpoint: Option<String>,
    pub supports_direct: bool,
    pub registered_at_unix_ms: u64,
    pub expires_at_unix_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterPeerResponseModel {
    pub lease_seconds: u64,
    pub peer: PeerRecordModel,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PeerLookupResponseModel {
    pub peer: PeerRecordModel,
}
