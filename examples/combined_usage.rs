//! Real demo: Connect to separate config and log servers
//!
//! This example demonstrates a realistic scenario:
//! - Configure separate config and log servers (can be different servers)
//! - Each with their own host and authentication token
//! - Append logs to log server
//! - Query configuration from config server
//!
//! Usage with environment variables:
//!   CONFIG_HOST=http://config-server:port CONFIG_TOKEN=config-token \
//!   LOG_HOST=http://log-server:port LOG_TOKEN=log-token \
//!   cargo run --example combined_usage
//!
//! Or simply: cargo run --example combined_usage (you'll be prompted for values)

use da_modrs_utility::config;
use da_modrs_utility::log;
use da_modrs_cloudserviceapi::transport::CloudTransport;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("\n=== Real Demo: Config & Log with Separate Servers ===\n");

    // Get config server details
    let config_host = get_config("CONFIG_HOST", "Config server host (e.g., http://config-server:8080)");
    let config_token = get_config("CONFIG_TOKEN", "Config server token (Bearer)");

    // Get log server details
    let log_host = get_config("LOG_HOST", "Log server host (e.g., http://log-server:8081)");
    let log_token = get_config("LOG_TOKEN", "Log server token (Bearer)");

    println!("\n📋 Configuration:");
    println!("  Config Server: {}", config_host);
    println!("  Config Token:  {}***", &config_token[..config_token.len().min(10)]);
    println!("  Log Server:    {}", log_host);
    println!("  Log Token:     {}***\n", &log_token[..log_token.len().min(10)]);

    // Set custom request ID prefixes for clarity
    config::daconfig_set_request_id_prefix("cfg");
    log::dalog_set_request_id_prefix("log");

    // Initialize separate transports for config and log servers
    let config_transport = CloudTransport::new(&config_host, &config_token);
    let log_transport = CloudTransport::new(&log_host, &log_token);

    let config_transport_service = "config";
    let config_service_id = "dademo";
    let config_app_id = "test-1";
    let config_version = "v1";
    let config_client = config::ConfigClient::new(config_transport, config_transport_service, config_service_id, config_app_id, config_version);

    let config_key = "key1";
    println!("🔍 Querying config key: '{}'\n", config_key);

    let payload = config::ConfigPayload {
        service: config_service_id.to_string(),
        instance: config_app_id.to_string(),
        key: config_key.to_string(),
    };


    match config_client.query("req-001", payload).await {
        Ok(response) => {
            println!("✅ Success!");
            println!("   Type: {:?}", response.r#type);
            println!("   Request ID: {:?}", response.request_id);
            println!("   Message: {:?}", response.message);
            println!("   Payload: {:?}", response.payload);
        }
        Err(e) => {
            println!("❌ Error querying config: {:?}", e);
            println!("\n💡 Make sure to:");
            println!("   1. Set CONFIG_HOST environment variable to your cloud service endpoint");
            println!("   2. Set CONFIG_TOKEN environment variable to your bearer token");
            println!("   3. Ensure the service is running and accessible");
        }
    }



    let log_transport_service = "log";
    let log_service_id = "dademo";
    let log_instance_id = "test-1";
    let log_version = "v1";
    let log_client = log::LogClient::new(log_transport, log_transport_service, log_service_id, log_instance_id, log_version);


    let log_level = 6;
    let log_message = "This is a test log message from the combined usage example.";
    // println!("\n📝 Appending log: '{}'\n", log_message);
    let log_payload = log::LogPayload {
        service: log_service_id.to_string(),
        instance: log_instance_id.to_string(),
        level: log_level,
        message: log_message.to_string(),
    };

    // match log::dalog_append_log_with_id("req-002", log_level, log_message).await {
    //     Ok(response) => {
    //         println!("✅ Log appended successfully!");
    //         println!("   Type: {:?}", response.r#type);
    //         println!("   Request ID: {:?}", response.request_id);
    //         println!("   Message: {:?}", response.message);
    //         println!("   Payload: {:?}", response.payload);
    //     }
    //     Err(e) => {
    //         println!("❌ Error appending log: {:?}", e);
    //         println!("\n💡 Make sure to:");
    //         println!("   1. Set LOG_HOST environment variable to your cloud service endpoint");
    //         println!("   2. Set LOG_TOKEN environment variable to your bearer token");
    //         println!("   3. Ensure the service is running and accessible");
    //     }
    // }


    match log_client.append("req-002", log_payload).await {
        Ok(response) => {
            println!("✅ Log appended successfully!");
            println!("   Type: {:?}", response.r#type);
            println!("   Request ID: {:?}", response.request_id);
            println!("   Message: {:?}", response.message);
            println!("   Payload: {:?}", response.payload);
        }
        Err(e) => {
            println!("❌ Error appending log: {:?}", e);
            println!("\n💡 Make sure to:");
            println!("   1. Set LOG_HOST environment variable to your cloud service endpoint");
            println!("   2. Set LOG_TOKEN environment variable to your bearer token");
            println!("   3. Ensure the service is running and accessible");
        }
    }

}

/// Get configuration value from environment or prompt user
fn get_config(env_var: &str, prompt: &str) -> String {
    if let Ok(value) = std::env::var(env_var) {
        if !value.is_empty() {
            return value;
        }
    }

    print!("\n🔧 {}: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
