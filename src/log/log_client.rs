use da_modrs_cloudserviceapi::{CloudRequest, CloudResponse, CloudApiResult, Transport};
use serde_json::json;
use std::sync::Arc;
use once_cell::sync::OnceCell;
use crate::log::payload::LogPayload;
use crate::log::request_id::generate_request_id;

/// Default version for requests
//const DEFAULT_VERSION: &str = "v1";

/// Generic singleton for the log client
static LOG_CLIENT: OnceCell<Arc<LogClient<da_modrs_cloudserviceapi::transport::CloudTransport>>> =
    OnceCell::new();

/// Log client using any Transport
pub struct LogClient<T: Transport> {
    transport: T,
    pub transport_service: String,
    pub service: String,
    pub instance: String,
    pub version: String,
}

impl<T: Transport> LogClient<T> {
    /// Create a new log client
    pub fn new(transport: T, transport_service: impl Into<String>, service: impl Into<String>, instance: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            transport,
            transport_service: transport_service.into(),
            service: service.into(),
            instance: instance.into(),
            version: version.into(),
        }
    }

    /// Append a log entry
    pub async fn append(
        &self,
        request_id: &str,
        payload: LogPayload,
    ) -> CloudApiResult<CloudResponse> {
        let request = CloudRequest::new(
            self.version.clone(),
            request_id.to_string(),
            "append",
            self.transport_service.clone(),
            json!(payload),
        );

        self.transport.send::<CloudResponse>(request).await
    }
}

/// Initialize the global log client singleton
pub fn dalog_client_init(
    transport: da_modrs_cloudserviceapi::transport::CloudTransport,
    transport_service: impl Into<String>,
    service: impl Into<String>,
    instance: impl Into<String>,
    version: impl Into<String>,
) -> Arc<LogClient<da_modrs_cloudserviceapi::transport::CloudTransport>> {
    let client = LogClient::new(transport, transport_service, service, instance, version);
    let arc_client = Arc::new(client);
    if LOG_CLIENT.set(arc_client.clone()).is_err() {
        println!("LogClient is already initialized");
    }
    arc_client
}

/// Retrieve the initialized log client
pub fn get_log_client() -> Arc<LogClient<da_modrs_cloudserviceapi::transport::CloudTransport>> {
    LOG_CLIENT.get().expect("LogClient is not initialized").clone()
}

/// Append a log via the global singleton using the global request ID
pub async fn dalog_append_log(
    level: u8,
    message: &str,
) -> CloudApiResult<CloudResponse> {
    let request_id = generate_request_id(); // auto-generated
    println!("{}", request_id);
    dalog_append_log_with_id(&request_id, level, message).await
}

/// Append a log via the global singleton with a specific request ID
pub async fn dalog_append_log_with_id(
    request_id: &str,
    level: u8,
    message: &str,
) -> CloudApiResult<CloudResponse> {
    let client = get_log_client();

    let payload = LogPayload {
        service: client.service.clone(),
        instance: client.instance.clone(),
        level,
        message: message.to_string(),
    };


    client.append(request_id, payload).await
}
