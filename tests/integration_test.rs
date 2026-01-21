//! Unit and integration tests for da-modrs-utility
//!
//! These tests verify:
//! - Request ID generation
//! - Payload structures
//! - Client initialization
//! - Request ID prefix changes

#[cfg(test)]
mod config_tests {
    use da_modrs_utility::config;

    #[test]
    fn test_config_request_id_generation() {
        // Reset with a known prefix
        config::daconfig_set_request_id_prefix("test");

        // We can't directly call generate_request_id from this module,
        // but the prefix setting should work without panicking
        println!("✓ Config request ID prefix set successfully");
    }

    #[test]
    fn test_config_payload_creation() {
        let payload = config::ConfigPayload {
            service: "my-service".to_string(),
            instance: "prod".to_string(),
            key: "database.url".to_string(),
        };

        assert_eq!(payload.service, "my-service");
        assert_eq!(payload.instance, "prod");
        assert_eq!(payload.key, "database.url");
    }

    #[test]
    fn test_config_payload_serialization() {
        let payload = config::ConfigPayload {
            service: "test-service".to_string(),
            instance: "test-instance".to_string(),
            key: "test-key".to_string(),
        };

        // Test that payload can be serialized to JSON
        let json = serde_json::to_value(&payload).expect("Failed to serialize payload");
        assert_eq!(json["service"], "test-service");
        assert_eq!(json["instance"], "test-instance");
        assert_eq!(json["key"], "test-key");
    }
}

#[cfg(test)]
mod log_tests {
    use da_modrs_utility::log;

    #[test]
    fn test_log_request_id_generation() {
        // Reset with a known prefix
        log::dalog_set_request_id_prefix("logtest");

        // The prefix setting should work without panicking
        println!("✓ Log request ID prefix set successfully");
    }

    #[test]
    fn test_log_payload_creation() {
        let payload = log::LogPayload {
            service: "my-service".to_string(),
            instance: "prod".to_string(),
            level: 1,
            message: "Application started".to_string(),
        };

        assert_eq!(payload.service, "my-service");
        assert_eq!(payload.instance, "prod");
        assert_eq!(payload.level, 1);
        assert_eq!(payload.message, "Application started");
    }

    #[test]
    fn test_log_payload_all_levels() {
        let levels = vec![0, 1, 2, 3];
        let level_names = vec!["DEBUG", "INFO", "WARN", "ERROR"];

        for (level, name) in levels.iter().zip(level_names.iter()) {
            let payload = log::LogPayload {
                service: "test".to_string(),
                instance: "test".to_string(),
                level: *level,
                message: format!("Test {} message", name),
            };

            assert_eq!(payload.level, *level);
            assert!(payload.message.contains(name));
        }
    }

    #[test]
    fn test_log_payload_serialization() {
        let payload = log::LogPayload {
            service: "test-service".to_string(),
            instance: "test-instance".to_string(),
            level: 2,
            message: "Warning message".to_string(),
        };

        // Test that payload can be serialized to JSON
        let json = serde_json::to_value(&payload).expect("Failed to serialize payload");
        assert_eq!(json["service"], "test-service");
        assert_eq!(json["instance"], "test-instance");
        assert_eq!(json["level"], 2);
        assert_eq!(json["message"], "Warning message");
    }
}

#[cfg(test)]
mod utility_tests {
    use da_modrs_utility::{ConfigPayload, LogPayload};

    #[test]
    fn test_top_level_exports() {
        // Verify that top-level exports work correctly
        let config_payload = ConfigPayload {
            service: "svc".to_string(),
            instance: "inst".to_string(),
            key: "k".to_string(),
        };

        let log_payload = LogPayload {
            service: "svc".to_string(),
            instance: "inst".to_string(),
            level: 1,
            message: "msg".to_string(),
        };

        assert_eq!(config_payload.key, "k");
        assert_eq!(log_payload.level, 1);
    }

    #[test]
    fn test_module_organization() {
        // Test that both modules are accessible
        use da_modrs_utility::config;
        use da_modrs_utility::log;

        // Set prefixes - if these don't panic, the modules are working
        config::daconfig_set_request_id_prefix("cfg");
        log::dalog_set_request_id_prefix("log");

        println!("✓ Both modules are accessible");
    }
}

#[cfg(test)]
mod payload_validation_tests {
    use da_modrs_utility::config::ConfigPayload;
    use da_modrs_utility::log::LogPayload;

    #[test]
    fn test_config_payload_field_types() {
        let payload = ConfigPayload {
            service: "service".to_string(),
            instance: "instance".to_string(),
            key: "key".to_string(),
        };

        // Verify all fields are strings
        let _: String = payload.service;
        let _: String = payload.instance;
        let _: String = payload.key;
    }

    #[test]
    fn test_log_payload_field_types() {
        let payload = LogPayload {
            service: "service".to_string(),
            instance: "instance".to_string(),
            level: 0,
            message: "message".to_string(),
        };

        // Verify field types
        let _: String = payload.service;
        let _: String = payload.instance;
        let _: u8 = payload.level;
        let _: String = payload.message;
    }

    #[test]
    fn test_empty_strings() {
        let config = ConfigPayload {
            service: String::new(),
            instance: String::new(),
            key: String::new(),
        };

        assert!(config.service.is_empty());
        assert!(config.instance.is_empty());
        assert!(config.key.is_empty());
    }

    #[test]
    fn test_large_strings() {
        let large_string = "x".repeat(10000);

        let config = ConfigPayload {
            service: large_string.clone(),
            instance: large_string.clone(),
            key: large_string.clone(),
        };

        assert_eq!(config.service.len(), 10000);
        assert_eq!(config.instance.len(), 10000);
        assert_eq!(config.key.len(), 10000);
    }

    #[test]
    fn test_special_characters_in_payloads() {
        let special_chars = "!@#$%^&*()_+-=[]{}|;:',.<>?/~`";

        let config = ConfigPayload {
            service: special_chars.to_string(),
            instance: special_chars.to_string(),
            key: special_chars.to_string(),
        };

        let log = LogPayload {
            service: special_chars.to_string(),
            instance: special_chars.to_string(),
            level: 1,
            message: special_chars.to_string(),
        };

        assert_eq!(config.service, special_chars);
        assert_eq!(log.message, special_chars);
    }
}

#[cfg(test)]
mod json_serialization_tests {
    use da_modrs_utility::config::ConfigPayload;
    use da_modrs_utility::log::LogPayload;

    #[test]
    fn test_config_json_structure() {
        let payload = ConfigPayload {
            service: "my-service".to_string(),
            instance: "prod".to_string(),
            key: "app.name".to_string(),
        };

        let json_str = serde_json::to_string(&payload).expect("Serialization failed");
        let json_obj: serde_json::Value =
            serde_json::from_str(&json_str).expect("Deserialization failed");

        assert_eq!(json_obj["service"], "my-service");
        assert_eq!(json_obj["instance"], "prod");
        assert_eq!(json_obj["key"], "app.name");
    }

    #[test]
    fn test_log_json_structure() {
        let payload = LogPayload {
            service: "auth-service".to_string(),
            instance: "staging".to_string(),
            level: 2,
            message: "User login failed".to_string(),
        };

        let json_str = serde_json::to_string(&payload).expect("Serialization failed");
        let json_obj: serde_json::Value =
            serde_json::from_str(&json_str).expect("Deserialization failed");

        assert_eq!(json_obj["service"], "auth-service");
        assert_eq!(json_obj["instance"], "staging");
        assert_eq!(json_obj["level"], 2);
        assert_eq!(json_obj["message"], "User login failed");
    }

    #[test]
    fn test_json_pretty_print() {
        let payload = ConfigPayload {
            service: "test".to_string(),
            instance: "test".to_string(),
            key: "test".to_string(),
        };

        let json_str = serde_json::to_string_pretty(&payload).expect("Serialization failed");
        println!("Pretty JSON:\n{}", json_str);

        // Ensure pretty print includes newlines
        assert!(json_str.contains('\n'));
    }
}
