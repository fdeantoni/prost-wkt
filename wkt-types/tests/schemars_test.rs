#[cfg(feature = "schemars")]
mod schemars_tests {
    use prost_wkt_types::*;
    use schemars::{JsonSchema, gen::SchemaGenerator, schema::Schema};

    #[test]
    fn test_empty_schema_generation() {
        let schema = Empty::json_schema(&mut SchemaGenerator::default());

        if let Schema::Object(mut schema_obj) = schema {
            assert!(schema_obj.instance_type.is_some());
            assert!(schema_obj.metadata().description.is_some());

            let description = schema_obj.metadata().description.as_ref().unwrap();
            assert_eq!(description, "Represents an empty message");
        } else {
            panic!("Expected Schema::Object for Empty");
        }
    }

    #[test]
    fn test_empty_schema_name() {
        assert_eq!(Empty::schema_name(), "Empty");
    }

    #[test]
    fn test_empty_schema_id() {
        assert_eq!(Empty::schema_id(), "prost_wkt_types::Empty");
    }

    #[test]
    fn test_timestamp_schema_generation() {
        let schema = Timestamp::json_schema(&mut SchemaGenerator::default());

        if let Schema::Object(mut schema_obj) = schema {
            assert!(schema_obj.instance_type.is_some());
            assert!(schema_obj.metadata().description.is_some());

            let description = schema_obj.metadata().description.as_ref().unwrap();
            assert!(description.contains("timestamp"));
        } else {
            panic!("Expected Schema::Object for Timestamp");
        }
    }

    #[test]
    fn test_timestamp_schema_name() {
        assert_eq!(Timestamp::schema_name(), "Timestamp");
    }

    #[test]
    fn test_timestamp_schema_id() {
        assert_eq!(Timestamp::schema_id(), "prost_wkt_types::Timestamp");
    }

    #[test]
    fn test_duration_schema_generation() {
        let schema = Duration::json_schema(&mut SchemaGenerator::default());

        if let Schema::Object(mut schema_obj) = schema {
            assert!(schema_obj.instance_type.is_some());
            assert!(schema_obj.metadata().description.is_some());

            let description = schema_obj.metadata().description.as_ref().unwrap();
            assert!(description.contains("duration"));
        } else {
            panic!("Expected Schema::Object for Duration");
        }
    }

    #[test]
    fn test_duration_schema_name() {
        assert_eq!(Duration::schema_name(), "Duration");
    }

    #[test]
    fn test_duration_schema_id() {
        assert_eq!(Duration::schema_id(), "prost_wkt_types::Duration");
    }

    #[test]
    fn test_any_schema_generation() {
        let schema = Any::json_schema(&mut SchemaGenerator::default());

        if let Schema::Object(mut schema_obj) = schema {
            assert!(schema_obj.instance_type.is_some());
            assert!(schema_obj.metadata().description.is_some());

            let description = schema_obj.metadata().description.as_ref().unwrap();
            assert!(description.contains("dynamically typed"));
        } else {
            panic!("Expected Schema::Object for Any");
        }
    }

    #[test]
    fn test_any_schema_name() {
        assert_eq!(Any::schema_name(), "Any");
    }

    #[test]
    fn test_any_schema_id() {
        assert_eq!(Any::schema_id(), "prost_wkt_types::Any");
    }

    #[test]
    fn test_schema_generation_with_custom_generator() {
        let mut generator = SchemaGenerator::default();

        // Generate schemas for all types to ensure they work with custom generators
        let empty_schema = Empty::json_schema(&mut generator);
        let timestamp_schema = Timestamp::json_schema(&mut generator);
        let duration_schema = Duration::json_schema(&mut generator);
        let any_schema = Any::json_schema(&mut generator);

        // Verify they're all objects
        assert!(matches!(empty_schema, Schema::Object(_)));
        assert!(matches!(timestamp_schema, Schema::Object(_)));
        assert!(matches!(duration_schema, Schema::Object(_)));
        assert!(matches!(any_schema, Schema::Object(_)));
    }

    #[test]
    fn test_schema_metadata_consistency() {
        let types = vec![
            (Empty::schema_name(), Empty::schema_id(), Empty::json_schema(&mut SchemaGenerator::default())),
            (Timestamp::schema_name(), Timestamp::schema_id(), Timestamp::json_schema(&mut SchemaGenerator::default())),
            (Duration::schema_name(), Duration::schema_id(), Duration::json_schema(&mut SchemaGenerator::default())),
            (Any::schema_name(), Any::schema_id(), Any::json_schema(&mut SchemaGenerator::default())),
        ];

        for (name, id, schema) in types {
            // Verify name is not empty
            assert!(!name.is_empty());

            // Verify id contains the type name
            assert!(id.contains(&name));

            // Verify schema is an object with metadata
            if let Schema::Object(mut schema_obj) = schema {
                assert!(schema_obj.metadata().description.is_some());
                let description = schema_obj.metadata().description.as_ref().unwrap();
                assert!(!description.is_empty());
            } else {
                panic!("Expected Schema::Object for {}", name);
            }
        }
    }

    #[test]
    fn test_schema_serialization() {
        let mut generator = SchemaGenerator::default();

        let empty_schema = Empty::json_schema(&mut generator);
        let json_schema = serde_json::to_string(&empty_schema).unwrap();

        // Verify the schema can be serialized to JSON
        assert!(!json_schema.is_empty());
        assert!(json_schema.contains("object"));

        // Verify it can be parsed back
        let _parsed: serde_json::Value = serde_json::from_str(&json_schema).unwrap();
    }

    #[test]
    fn test_all_schemas_unique_ids() {
        let ids = vec![
            Empty::schema_id(),
            Timestamp::schema_id(),
            Duration::schema_id(),
            Any::schema_id(),
        ];

        // Convert to set to check uniqueness
        let unique_ids: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique_ids.len(), ids.len(), "Schema IDs should be unique");
    }
}

#[cfg(not(feature = "schemars"))]
mod no_schemars_tests {
    #[test]
    fn test_schemars_feature_disabled() {
        // This test just verifies that we can compile without the schemars feature
        // The actual schemars implementations won't be available
        assert!(true, "Schemars feature is disabled");
    }
}