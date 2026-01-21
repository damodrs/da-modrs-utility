use serde::Serialize;

/// Payload for config append requests
#[derive(Serialize, Debug)]
pub struct ConfigPayload {
    pub service: String,
    pub instance: String,
    pub key: String,
}
