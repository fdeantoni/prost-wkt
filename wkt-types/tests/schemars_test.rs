#[cfg(feature = "schemars")]
mod schemars_tests {
    use prost_wkt_types::*;
    use schemars::{generate::SchemaGenerator, JsonSchema, Schema};

    #[test]
    fn test_empty_schema_generation() {
        let schema = Empty::json_schema(&mut SchemaGenerator::default());
        let json = serde_json::to_value(&schema).unwrap();

        assert_eq!(json["type"], "object");
        assert_eq!(json["description"], "Represents an empty message");
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
        let json = serde_json::to_value(&schema).unwrap();

        assert_eq!(json["type"], "string");
        let description = json["description"].as_str().unwrap();
        assert!(description.contains("timestamp"), "Description should contain 'timestamp': {}", description);
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
        let json = serde_json::to_value(&schema).unwrap();

        assert_eq!(json["type"], "string");
        let description = json["description"].as_str().unwrap();
        assert!(description.contains("duration"), "Description should contain 'duration': {}", description);
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
        let json = serde_json::to_value(&schema).unwrap();

        assert_eq!(json["type"], "object");
        let description = json["description"].as_str().unwrap();
        assert!(description.contains("dynamically typed"), "Description should contain 'dynamically typed': {}", description);
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

        // Verify they can all be serialized to JSON
        assert!(serde_json::to_value(&empty_schema).is_ok());
        assert!(serde_json::to_value(&timestamp_schema).is_ok());
        assert!(serde_json::to_value(&duration_schema).is_ok());
        assert!(serde_json::to_value(&any_schema).is_ok());
    }

    #[test]
    fn test_schema_metadata_consistency() {
        let types: Vec<(std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>, Schema)> = vec![
            (Empty::schema_name(), Empty::schema_id(), Empty::json_schema(&mut SchemaGenerator::default())),
            (Timestamp::schema_name(), Timestamp::schema_id(), Timestamp::json_schema(&mut SchemaGenerator::default())),
            (Duration::schema_name(), Duration::schema_id(), Duration::json_schema(&mut SchemaGenerator::default())),
            (Any::schema_name(), Any::schema_id(), Any::json_schema(&mut SchemaGenerator::default())),
        ];

        for (name, id, schema) in types {
            // Verify name is not empty
            assert!(!name.is_empty());

            // Verify id contains the type name
            assert!(id.contains(name.as_ref()), "ID '{}' should contain name '{}'", id, name);

            // Verify schema can be serialized and has a description
            let json = serde_json::to_value(&schema).unwrap();
            assert!(json.get("description").is_some(), "Schema for {} should have a description", name);
            let description = json["description"].as_str().unwrap();
            assert!(!description.is_empty(), "Description for {} should not be empty", name);
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
