use openhanse_protocol::model::{
    connect_model::ConnectRequestModel,
    message_model::ChatMessageEnvelopeModel,
    peer_model::{HeartbeatRequestModel, RegisterPeerRequestModel},
};
use serde::{Deserialize, Serialize};

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
    ) -> ConnectRequestModel {
        ConnectRequestModel {
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

#[cfg(test)]
mod tests {
    use super::{DeliveryPreference, GatewayProfile};

    fn gateway_profile() -> GatewayProfile {
        GatewayProfile {
            peer_id: "gateway-a".to_string(),
            device_key: "device-key-a".to_string(),
            display_name: Some("Gateway A".to_string()),
            direct_addresses: vec!["http://127.0.0.1:7443".to_string()],
            message_endpoint: Some("/message".to_string()),
            supports_direct: true,
        }
    }

    #[test]
    fn builds_register_request_from_profile() {
        let profile = gateway_profile();
        let request = profile.register_request();

        assert_eq!(request.peer_id, "gateway-a");
        assert_eq!(request.message_endpoint.as_deref(), Some("/message"));
    }

    #[test]
    fn relay_only_disables_direct_preference() {
        assert!(DeliveryPreference::DirectFirst.prefer_direct());
        assert!(!DeliveryPreference::RelayOnly.prefer_direct());
    }
}
