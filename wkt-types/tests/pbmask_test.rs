use prost_wkt_types::*;

#[test]
fn test_pbmask_creation() {
    let mask = FieldMask {
        paths: vec!["field1".to_string(), "field2".to_string()],
    };
    assert_eq!(mask.paths.len(), 2);
    assert_eq!(mask.paths[0], "field1");
    assert_eq!(mask.paths[1], "field2");
}

#[test]
fn test_pbmask_empty() {
    let empty_mask = FieldMask { paths: vec![] };
    assert!(empty_mask.paths.is_empty());
    assert_eq!(empty_mask, FieldMask::default());
}

#[test]
fn test_pbmask_json_serialization() {
    let mask = FieldMask {
        paths: vec!["user.name".to_string(), "user.email".to_string()],
    };
    let json = serde_json::to_string(&mask).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Verify it's a JSON object with paths array
    assert!(parsed.is_object());
    let paths = parsed.get("paths").unwrap();
    assert!(paths.is_array());
    let paths_array = paths.as_array().unwrap();
    assert_eq!(paths_array.len(), 2);
    assert_eq!(paths_array[0].as_str().unwrap(), "user.name");
    assert_eq!(paths_array[1].as_str().unwrap(), "user.email");
}

#[test]
fn test_pbmask_json_deserialization() {
    let json = r#"{"paths": ["field.subfield", "other_field"]}"#;
    let mask: FieldMask = serde_json::from_str(json).unwrap();
    assert_eq!(mask.paths.len(), 2);
    assert_eq!(mask.paths[0], "field.subfield");
    assert_eq!(mask.paths[1], "other_field");
}

#[test]
fn test_pbmask_json_roundtrip() {
    let original = FieldMask {
        paths: vec![
            "settings.theme".to_string(),
            "profile.avatar".to_string(),
            "notifications.email".to_string(),
        ],
    };

    let json = serde_json::to_string(&original).unwrap();
    let decoded: FieldMask = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_pbmask_empty_json() {
    let empty_mask = FieldMask { paths: vec![] };
    let json = serde_json::to_string(&empty_mask).unwrap();
    let decoded: FieldMask = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, empty_mask);

    // Also test deserializing from minimal JSON
    let minimal_json = r#"{"paths": []}"#;
    let decoded_minimal: FieldMask = serde_json::from_str(minimal_json).unwrap();
    assert_eq!(decoded_minimal, empty_mask);
}

#[test]
fn test_pbmask_single_path() {
    let mask = FieldMask {
        paths: vec!["single_field".to_string()],
    };
    let json = serde_json::to_string(&mask).unwrap();
    let decoded: FieldMask = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, mask);
    assert_eq!(decoded.paths.len(), 1);
    assert_eq!(decoded.paths[0], "single_field");
}

#[test]
fn test_pbmask_special_characters() {
    let mask = FieldMask {
        paths: vec![
            "field_with_underscore".to_string(),
            "field.with.dots".to_string(),
            "field-with-dashes".to_string(),
        ],
    };
    let json = serde_json::to_string(&mask).unwrap();
    let decoded: FieldMask = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, mask);
}

#[test]
fn test_pbmask_clone_and_debug() {
    let mask = FieldMask {
        paths: vec!["test.field".to_string()],
    };
    let cloned = mask.clone();
    assert_eq!(mask, cloned);

    let debug_str = format!("{:?}", mask);
    assert!(debug_str.contains("FieldMask"));
    assert!(debug_str.contains("test.field"));
}

#[test]
fn test_pbmask_default() {
    let default_mask = FieldMask::default();
    assert!(default_mask.paths.is_empty());

    let empty_mask = FieldMask { paths: vec![] };
    assert_eq!(default_mask, empty_mask);
}

#[test]
fn test_pbmask_large_paths() {
    let paths: Vec<String> = (0..100)
        .map(|i| format!("field_{}", i))
        .collect();

    let mask = FieldMask { paths: paths.clone() };
    let json = serde_json::to_string(&mask).unwrap();
    let decoded: FieldMask = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded.paths.len(), 100);
    assert_eq!(decoded.paths, paths);
}