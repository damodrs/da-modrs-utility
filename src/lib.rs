//! da-modrs-utility
//!
//! Unified utility crate combining config and log clients using `da-modrs-cloudserviceapi`.

pub mod config {
    pub mod config_client;
    pub mod payload;
    pub mod request_id;

    pub use config_client::{ConfigClient, daconfig_client_init, daconfig_query_config, daconfig_query_config_with_id};
    pub use payload::ConfigPayload;
    pub use request_id::daconfig_set_request_id_prefix;
}

pub mod log {
    pub mod log_client;
    pub mod payload;
    pub mod request_id;

    pub use log_client::{LogClient, dalog_client_init, dalog_append_log, dalog_append_log_with_id};
    pub use payload::LogPayload;
    pub use request_id::dalog_set_request_id_prefix;
}

// Re-export commonly used types at the top level for convenience
pub use config::{ConfigClient, ConfigPayload};
pub use log::{LogClient, LogPayload};
