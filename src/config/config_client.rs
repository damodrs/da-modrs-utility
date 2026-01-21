use da_modrs_cloudserviceapi::{CloudRequest, CloudResponse, CloudApiResult, Transport};
use serde_json::json;
use std::sync::Arc;
use once_cell::sync::OnceCell;
use crate::config::payload::ConfigPayload;
use crate::config::request_id::generate_request_id;

/// Generic singleton for the config client
static CONFIG_CLIENT: OnceCell<Arc<ConfigClient<da_modrs_cloudserviceapi::transport::CloudTransport>>> =
    OnceCell::new();

/// Config client using any Transport
pub struct ConfigClient<T: Transport> {
    transport: T,
    pub transport_service: String,
    pub service: String,
    pub instance: String,
    pub version: String,
}

impl<T: Transport> ConfigClient<T> {
    pub fn new(transport: T, transport_service: impl Into<String>, service: impl Into<String>, instance: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            transport,
            transport_service: transport_service.into(),
            service: service.into(),
            instance: instance.into(),
            version: version.into(),
        }
    }

    /// Query a config entry
    pub async fn query(
        &self,
        request_id: &str,
        payload: ConfigPayload,
    ) -> CloudApiResult<CloudResponse> {
        let request = CloudRequest::new(
            self.version.clone(),
            request_id.to_string(),
            "getConfig",
            self.transport_service.clone(), 
            json!(payload),
        );

        self.transport.send::<CloudResponse>(request).await
    }
}

/// Initialize the global config client singleton
pub fn daconfig_client_init(
    transport: da_modrs_cloudserviceapi::transport::CloudTransport,
    transport_service: impl Into<String>,
    service: impl Into<String>,
    instance: impl Into<String>,
    version: impl Into<String>,
) -> Arc<ConfigClient<da_modrs_cloudserviceapi::transport::CloudTransport>> {
    let client = ConfigClient::new(transport, transport_service, service, instance, version);
    let arc_client = Arc::new(client);
    if CONFIG_CLIENT.set(arc_client.clone()).is_err() {
        println!("ConfigClient is already initialized");
    }
    arc_client
}

/// Retrieve the initialized config client
pub fn get_config_client() -> Arc<ConfigClient<da_modrs_cloudserviceapi::transport::CloudTransport>> {
    CONFIG_CLIENT.get().expect("ConfigClient is not initialized").clone()
}

pub async fn daconfig_query_config(
    key: &str,
) -> CloudApiResult<CloudResponse> {
    let request_id = generate_request_id();
    println!("{}", request_id);
    daconfig_query_config_with_id(&request_id, key).await
}

/// Query a config via the global singleton with a specific request ID
pub async fn daconfig_query_config_with_id(
    request_id: &str,
    key: &str,
) -> CloudApiResult<CloudResponse> {
    let client = get_config_client();

    let payload = ConfigPayload {
        service: client.service.clone(),
        instance: client.instance.clone(),
        key: key.to_string(),
    };

    client.query(request_id, payload).await
}
