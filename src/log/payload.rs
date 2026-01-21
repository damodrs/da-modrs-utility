use serde::Serialize;

/// Payload for log append requests
#[derive(Serialize, Debug)]
pub struct LogPayload {
    pub service: String,
    pub instance: String,
    pub level: u8,
    pub message: String,
}
