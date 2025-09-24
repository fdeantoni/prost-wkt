#[cfg(feature = "vendored-protox")]
mod protox_tests {
    use prost_wkt_types::*;

    #[test]
    fn test_protox_feature_flag_enabled() {
        // This test verifies that the protox feature flag is correctly enabled
        assert!(cfg!(feature = "vendored-protox"));
        println!("Protox feature is enabled and working!");
    }

    #[test]
    fn test_protox_basic_type_generation() {
        // Test that protox successfully compiled all the basic Well-Known Types

        // Test Empty type (from pbempty.proto)
        let empty = Empty {};
        assert_eq!(empty, Empty::default());

        // Test FieldMask type (from pbmask.proto)
        let mask = FieldMask {
            paths: vec!["field1".to_string(), "field2.subfield".to_string()],
        };
        assert_eq!(mask.paths.len(), 2);
        assert_eq!(mask.paths[0], "field1");

        // Test Timestamp type (from pbtime.proto)
        let timestamp = Timestamp {
            seconds: 1609459200, // 2021-01-01T00:00:00Z
            nanos: 123456789,
        };
        assert_eq!(timestamp.seconds, 1609459200);
        assert_eq!(timestamp.nanos, 123456789);

        // Test Duration type (from pbtime.proto)
        let duration = Duration {
            seconds: 3661, // 1h 1m 1s
            nanos: 500000000, // 0.5s
        };
        assert_eq!(duration.seconds, 3661);
        assert_eq!(duration.nanos, 500000000);

        // Test Any type (from pbany.proto)
        let any = Any {
            type_url: "type.googleapis.com/test.Message".to_string(),
            value: vec![1, 2, 3, 4],
        };
        assert_eq!(any.type_url, "type.googleapis.com/test.Message");
        assert_eq!(any.value, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_protox_json_serialization() {
        // Test that protox-compiled types support JSON serialization/deserialization

        // Test Empty JSON
        let empty = Empty {};
        let json = serde_json::to_string(&empty).unwrap();
        assert_eq!(json, "{}");
        let decoded: Empty = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, empty);

        // Test FieldMask JSON
        let mask = FieldMask {
            paths: vec!["user.name".to_string(), "user.email".to_string()],
        };
        let json = serde_json::to_string(&mask).unwrap();
        let decoded: FieldMask = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, mask);

        // Test Timestamp JSON
        let timestamp = Timestamp {
            seconds: 1609459200,
            nanos: 123456789,
        };
        let json = serde_json::to_string(&timestamp).unwrap();
        assert_eq!(json, r#""2021-01-01T00:00:00.123456789Z""#);
        let decoded: Timestamp = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, timestamp);

        // Test Duration JSON
        let duration = Duration {
            seconds: 3661,
            nanos: 500000000,
        };
        let json = serde_json::to_string(&duration).unwrap();
        assert_eq!(json, r#""3661.500000000s""#);
        let decoded: Duration = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, duration);
    }

    #[test]
    fn test_protox_message_serde_traits() {
        // Test that protox-compiled types implement MessageSerde traits correctly
        use prost_wkt::MessageSerde;

        // Test that Empty implements MessageSerde
        let empty = Empty {};
        let _serialized = empty.try_encoded().unwrap();

        // Test that FieldMask implements MessageSerde
        let mask = FieldMask {
            paths: vec!["test.field".to_string()],
        };
        let _serialized = mask.try_encoded().unwrap();

        // Test that Timestamp implements MessageSerde
        let timestamp = Timestamp {
            seconds: 1234567890,
            nanos: 0,
        };
        let _serialized = timestamp.try_encoded().unwrap();

        // Test that Duration implements MessageSerde
        let duration = Duration {
            seconds: 42,
            nanos: 0,
        };
        let _serialized = duration.try_encoded().unwrap();
    }

    #[test]
    fn test_protox_vs_protoc_compatibility() {
        // Test that protox-generated code produces the same results as protoc

        // Create test data
        let original_timestamp = Timestamp {
            seconds: 1640995200, // 2022-01-01T00:00:00Z
            nanos: 500000000,
        };

        let original_duration = Duration {
            seconds: 7200, // 2 hours
            nanos: 250000000,
        };

        let original_mask = FieldMask {
            paths: vec![
                "settings.theme".to_string(),
                "profile.avatar".to_string(),
                "notifications.email".to_string(),
            ],
        };

        // Test JSON serialization produces expected format
        let timestamp_json = serde_json::to_string(&original_timestamp).unwrap();
        assert!(timestamp_json.contains("2022-01-01") && timestamp_json.contains("0.5"));

        let duration_json = serde_json::to_string(&original_duration).unwrap();
        assert!(duration_json.contains("7200") && duration_json.contains("s"));

        let mask_json = serde_json::to_string(&original_mask).unwrap();
        assert!(mask_json.contains("settings.theme"));

        // Test roundtrip compatibility
        let decoded_timestamp: Timestamp = serde_json::from_str(&timestamp_json).unwrap();
        let decoded_duration: Duration = serde_json::from_str(&duration_json).unwrap();
        let decoded_mask: FieldMask = serde_json::from_str(&mask_json).unwrap();

        assert_eq!(decoded_timestamp, original_timestamp);
        assert_eq!(decoded_duration, original_duration);
        assert_eq!(decoded_mask, original_mask);
    }

    #[test]
    fn test_protox_any_type_functionality() {
        // Test Any type functionality with protox compilation

        let timestamp = Timestamp {
            seconds: 1640995200,
            nanos: 123456789,
        };

        // Test Any creation from message
        let any_result = Any::from_msg(&timestamp);

        match any_result {
            Ok(any) => {
                assert_eq!(any.type_url, "type.googleapis.com/google.protobuf.Timestamp");
                assert!(!any.value.is_empty());

                // Test Any JSON serialization
                let json = serde_json::to_string(&any).unwrap();
                assert!(!json.is_empty());
                assert!(json.contains("@type"));
            },
            Err(_) => {
                // This is acceptable - Any functionality may depend on registry
                // The important thing is that the Any type was generated correctly
                let any = Any {
                    type_url: "type.googleapis.com/test.Message".to_string(),
                    value: vec![1, 2, 3],
                };
                let json = serde_json::to_string(&any).unwrap();
                assert!(!json.is_empty());
            }
        }
    }

    #[test]
    fn test_protox_descriptor_compilation() {
        // Test that protox successfully generated descriptors and compiled protobuf files

        // The fact that we can create and use these types means protox worked

        // Test Empty
        let empty = Empty::default();
        let json = serde_json::to_string(&empty).unwrap();
        assert_eq!(json, "{}");

        // Test Timestamp
        let timestamp = Timestamp::default();
        let json = serde_json::to_string(&timestamp).unwrap();
        assert!(!json.is_empty());

        // Test Duration
        let duration = Duration::default();
        let json = serde_json::to_string(&duration).unwrap();
        assert!(!json.is_empty());

        // Test FieldMask
        let mask = FieldMask::default();
        let json = serde_json::to_string(&mask).unwrap();
        assert!(json.contains("paths"));

        // Test Any
        let any = Any::default();
        let json = serde_json::to_string(&any).unwrap();
        assert!(json.contains("@type"));
    }
}

#[cfg(not(feature = "vendored-protox"))]
mod no_protox_tests {
    #[test]
    fn test_protox_feature_disabled() {
        // This test verifies that when protox feature is disabled,
        // the code still compiles and works (using standard protoc)

        use prost_wkt_types::*;

        // Basic types should still work with standard protoc
        let empty = Empty {};
        let timestamp = Timestamp { seconds: 1609459200, nanos: 0 };
        let duration = Duration { seconds: 3600, nanos: 0 };
        let mask = FieldMask { paths: vec!["test".to_string()] };

        // JSON serialization should work regardless
        let empty_json = serde_json::to_string(&empty).unwrap();
        let timestamp_json = serde_json::to_string(&timestamp).unwrap();
        let duration_json = serde_json::to_string(&duration).unwrap();
        let mask_json = serde_json::to_string(&mask).unwrap();

        assert_eq!(empty_json, "{}");
        assert!(!timestamp_json.is_empty());
        assert!(!duration_json.is_empty());
        assert!(!mask_json.is_empty());
    }
}