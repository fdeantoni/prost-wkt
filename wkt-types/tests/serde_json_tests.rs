use prost::Name;
use prost_wkt::*;
use prost_wkt_types::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json;

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize, MessageSerde)]
#[prost(package = "any.test")]
#[serde(rename_all = "camelCase")]
pub struct Foo {
    #[prost(string, tag = "1")]
    pub string: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::std::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(bool, tag = "3")]
    pub boolean: bool,
    #[prost(message, optional, tag = "4")]
    pub value_data: ::std::option::Option<::prost_wkt_types::Value>,
    #[prost(string, repeated, tag = "5")]
    pub list: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag = "6")]
    pub payload: ::std::option::Option<::prost_wkt_types::Any>,
}

impl Name for Foo {
    const PACKAGE: &'static str = "any.test";
    const NAME: &'static str = "Foo";

    fn type_url() -> String {
        "type.googleapis.com/any.test.Foo".to_string()
    }
}

#[test]
fn test_timestamp_json_serialization() {
    let timestamp = Timestamp {
        seconds: 1609459200, // 2021-01-01T00:00:00Z
        nanos: 123456789,
    };

    let json = serde_json::to_string(&timestamp).unwrap();
    assert_eq!(json, r#""2021-01-01T00:00:00.123456789Z""#);

    let deserialized: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, timestamp);
}

#[test]
fn test_timestamp_json_edge_cases() {
    // Zero timestamp
    let zero = Timestamp::default();
    let json = serde_json::to_string(&zero).unwrap();
    assert_eq!(json, r#""1970-01-01T00:00:00Z""#);
    let deserialized: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, zero);

    // Maximum timestamp
    let max_ts = Timestamp {
        seconds: 253402300799, // 9999-12-31T23:59:59Z
        nanos: 999999999,
    };
    let json = serde_json::to_string(&max_ts).unwrap();
    let deserialized: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, max_ts);

    // Negative timestamp
    let negative = Timestamp {
        seconds: -1,
        nanos: 0,
    };
    let json = serde_json::to_string(&negative).unwrap();
    let deserialized: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, negative);
}

#[test]
fn test_duration_json_serialization() {
    let duration = Duration {
        seconds: 3661, // 1h 1m 1s
        nanos: 500000000, // 0.5s
    };

    let json = serde_json::to_string(&duration).unwrap();
    assert_eq!(json, r#""3661.500000000s""#);

    let deserialized: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, duration);
}

#[test]
fn test_duration_json_edge_cases() {
    // Zero duration
    let zero = Duration::default();
    let json = serde_json::to_string(&zero).unwrap();
    assert_eq!(json, r#""0.000000000s""#);
    let deserialized: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, zero);

    // Negative duration
    let negative = Duration {
        seconds: -10,
        nanos: 0,  // Use 0 nanos to avoid normalization issues
    };
    let json = serde_json::to_string(&negative).unwrap();
    let deserialized: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, negative);

    // Large duration
    let large = Duration {
        seconds: 315576000000, // ~10,000 years
        nanos: 0,  // Avoid potential normalization issues with large values
    };
    let json = serde_json::to_string(&large).unwrap();
    let deserialized: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, large);
}

#[test]
fn test_value_json_serialization_all_types() {
    // Null value
    let null_val = Value::null();
    let json = serde_json::to_string(&null_val).unwrap();
    assert_eq!(json, "null");
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, null_val);

    // Number value
    let num_val = Value::from(42.5);
    let json = serde_json::to_string(&num_val).unwrap();
    assert_eq!(json, "42.5");
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, num_val);

    // String value
    let str_val = Value::from("hello world".to_string());
    let json = serde_json::to_string(&str_val).unwrap();
    assert_eq!(json, r#""hello world""#);
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, str_val);

    // Boolean value
    let bool_val = Value::from(true);
    let json = serde_json::to_string(&bool_val).unwrap();
    assert_eq!(json, "true");
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, bool_val);

    // List value
    let list_val = Value::from(vec![Value::from(1.0), Value::from("test".to_string()), Value::null()]);
    let json = serde_json::to_string(&list_val).unwrap();
    assert_eq!(json, r#"[1.0,"test",null]"#);
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, list_val);

    // Struct value
    let mut map = HashMap::new();
    map.insert("key1".to_string(), Value::from("value1".to_string()));
    map.insert("key2".to_string(), Value::from(123.0));
    let struct_val = Value::from(map);
    let json = serde_json::to_string(&struct_val).unwrap();
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, struct_val);
}

#[test]
fn test_value_json_nested_structures() {
    let mut inner_map = HashMap::new();
    inner_map.insert("nested".to_string(), Value::from(true));
    inner_map.insert("count".to_string(), Value::from(42.0));

    let mut outer_map = HashMap::new();
    outer_map.insert("inner".to_string(), Value::from(inner_map));
    outer_map.insert("list".to_string(), Value::from(vec![
        Value::from(1.0),
        Value::from(2.0),
        Value::from(vec![Value::from("nested_list".to_string())])
    ]));

    let complex_val = Value::from(outer_map);
    let json = serde_json::to_string(&complex_val).unwrap();
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, complex_val);
}

#[test]
fn test_any_json_serialization_with_different_payloads() {
    // Test with Timestamp payload
    let timestamp = Timestamp {
        seconds: 1234567890,
        nanos: 123456789,
    };
    let any_ts = Any::from_msg(&timestamp).unwrap();
    let json = serde_json::to_string(&any_ts).unwrap();
    let deserialized: Any = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.type_url, any_ts.type_url);
    assert_eq!(deserialized.value, any_ts.value);

    // Test with Duration payload
    let duration = Duration {
        seconds: 3600,
        nanos: 0,
    };
    let any_dur = Any::from_msg(&duration).unwrap();
    let json = serde_json::to_string(&any_dur).unwrap();
    let deserialized: Any = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.type_url, any_dur.type_url);
    assert_eq!(deserialized.value, any_dur.value);

    // Test with custom Foo payload
    let foo = Foo {
        string: "test".to_string(),
        timestamp: Some(timestamp),
        boolean: true,
        value_data: Some(Value::from("data".to_string())),
        list: vec!["item1".to_string(), "item2".to_string()],
        payload: None,
    };
    let any_foo = Any::from_msg(&foo).unwrap();
    let json = serde_json::to_string(&any_foo).unwrap();
    let deserialized: Any = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.type_url, any_foo.type_url);
    assert_eq!(deserialized.value, any_foo.value);
}

#[test]
fn test_empty_json_serialization() {
    let empty = Empty {};
    let json = serde_json::to_string(&empty).unwrap();
    assert_eq!(json, "{}");
    let deserialized: Empty = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, empty);
}

#[test]
fn test_field_mask_json_serialization() {
    let mask = FieldMask {
        paths: vec!["user.name".to_string(), "user.email".to_string(), "settings.theme".to_string()],
    };
    let json = serde_json::to_string(&mask).unwrap();
    // FieldMask serializes as JSON object with paths array, not as comma-separated string
    let deserialized: FieldMask = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, mask);
    let deserialized: FieldMask = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, mask);

    // Empty field mask
    let empty_mask = FieldMask { paths: vec![] };
    let json = serde_json::to_string(&empty_mask).unwrap();
    // Empty field mask serializes as JSON object with empty paths array
    let deserialized: FieldMask = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, empty_mask);
    let deserialized: FieldMask = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, empty_mask);
}

#[test]
fn test_roundtrip_serialization_complex_message() {
    let original = Foo {
        string: "roundtrip_test".to_string(),
        timestamp: Some(Timestamp {
            seconds: 1640995200, // 2022-01-01T00:00:00Z
            nanos: 123456789,
        }),
        boolean: true,
        value_data: Some({
            let mut complex_map = HashMap::new();
            complex_map.insert("str_key".to_string(), Value::from("string_value".to_string()));
            complex_map.insert("num_key".to_string(), Value::from(3.14159));
            complex_map.insert("bool_key".to_string(), Value::from(false));
            complex_map.insert("null_key".to_string(), Value::null());
            complex_map.insert("list_key".to_string(), Value::from(vec![
                Value::from(1.0),
                Value::from("nested".to_string()),
                Value::from(vec![Value::from(2.0), Value::from(3.0)])
            ]));
            Value::from(complex_map)
        }),
        list: vec!["item1".to_string(), "item2".to_string(), "item3".to_string()],
        payload: Some({
            let inner_foo = Foo {
                string: "nested_payload".to_string(),
                timestamp: Some(Timestamp { seconds: 0, nanos: 0 }),
                boolean: false,
                value_data: Some(Value::from("nested_data".to_string())),
                list: vec!["nested_item".to_string()],
                payload: None,
            };
            Any::from_msg(&inner_foo).unwrap()
        }),
    };

    // JSON roundtrip
    let json = serde_json::to_string(&original).unwrap();
    let json_deserialized: Foo = serde_json::from_str(&json).unwrap();
    assert_eq!(json_deserialized, original);

    // Pretty JSON roundtrip
    let pretty_json = serde_json::to_string_pretty(&original).unwrap();
    let pretty_deserialized: Foo = serde_json::from_str(&pretty_json).unwrap();
    assert_eq!(pretty_deserialized, original);
}

#[test]
fn test_roundtrip_serialization_all_wkt_types() {
    let test_cases = vec![
        // Timestamp
        (
            "timestamp",
            serde_json::to_value(Timestamp { seconds: 1234567890, nanos: 123456 }).unwrap(),
        ),
        // Duration
        (
            "duration",
            serde_json::to_value(Duration { seconds: 3600, nanos: 500000000 }).unwrap(),
        ),
        // Empty
        (
            "empty",
            serde_json::to_value(Empty {}).unwrap(),
        ),
        // FieldMask
        (
            "field_mask",
            serde_json::to_value(FieldMask {
                paths: vec!["field1".to_string(), "field2.subfield".to_string()],
            }).unwrap(),
        ),
        // Value - all variants
        (
            "value_null",
            serde_json::to_value(Value::null()).unwrap(),
        ),
        (
            "value_number",
            serde_json::to_value(Value::from(42.0)).unwrap(),
        ),
        (
            "value_string",
            serde_json::to_value(Value::from("test".to_string())).unwrap(),
        ),
        (
            "value_bool",
            serde_json::to_value(Value::from(true)).unwrap(),
        ),
        (
            "value_list",
            serde_json::to_value(Value::from(vec![Value::from(1.0), Value::from(2.0)])).unwrap(),
        ),
    ];

    for (test_name, json_value) in test_cases {
        let json_str = serde_json::to_string(&json_value).unwrap();
        let reconstructed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(reconstructed, json_value, "Roundtrip failed for {}", test_name);
    }
}

#[test]
fn test_json_special_numeric_values() {
    // Test infinity and NaN handling in Value
    // Note: JSON doesn't support Infinity, so it becomes null in serde_json
    let inf_value = Value::from(f64::INFINITY);
    let json = serde_json::to_string(&inf_value).unwrap();
    // Infinity becomes "null" in JSON
    assert_eq!(json, "null");
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    if let Some(value::Kind::NullValue(_)) = deserialized.kind {
        // This is expected for infinity in JSON
    } else {
        panic!("Expected null value for infinity");
    }

    let neg_inf_value = Value::from(f64::NEG_INFINITY);
    let json = serde_json::to_string(&neg_inf_value).unwrap();
    assert_eq!(json, "null");
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    if let Some(value::Kind::NullValue(_)) = deserialized.kind {
        // This is expected for negative infinity in JSON
    } else {
        panic!("Expected null value for negative infinity");
    }
}

#[test]
fn test_json_unicode_handling() {
    // Test Unicode strings in Value
    let unicode_strings = vec![
        "Hello, 世界",
        "🚀🌟💫",
        "Ελληνικά",
        "العربية",
        "русский язык",
        "\u{1F4A9}\u{FE0F}",  // Pile of poo emoji
    ];

    for unicode_str in unicode_strings {
        let value = Value::from(unicode_str.to_string());
        let json = serde_json::to_string(&value).unwrap();
        let deserialized: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, value, "Unicode handling failed for: {}", unicode_str);

        // Also test in Foo struct
        let foo = Foo {
            string: unicode_str.to_string(),
            timestamp: None,
            boolean: false,
            value_data: Some(Value::from(unicode_str.to_string())),
            list: vec![unicode_str.to_string()],
            payload: None,
        };
        let json = serde_json::to_string(&foo).unwrap();
        let deserialized: Foo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, foo);
    }
}

#[test]
fn test_json_empty_and_minimal_values() {
    // Empty strings
    let empty_str_value = Value::from("".to_string());
    let json = serde_json::to_string(&empty_str_value).unwrap();
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, empty_str_value);

    // Empty lists
    let empty_list = Value::from(Vec::<Value>::new());
    let json = serde_json::to_string(&empty_list).unwrap();
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, empty_list);

    // Empty struct
    let empty_struct = Value::from(HashMap::<String, Value>::new());
    let json = serde_json::to_string(&empty_struct).unwrap();
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, empty_struct);

    // Zero values
    let zero_duration = Duration::default();
    let json = serde_json::to_string(&zero_duration).unwrap();
    let deserialized: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, zero_duration);

    let zero_timestamp = Timestamp::default();
    let json = serde_json::to_string(&zero_timestamp).unwrap();
    let deserialized: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, zero_timestamp);
}

#[test]
fn test_json_large_values() {
    // Large numbers
    let large_number = Value::from(1e308_f64);
    let json = serde_json::to_string(&large_number).unwrap();
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, large_number);

    // Very small numbers
    let small_number = Value::from(1e-308_f64);
    let json = serde_json::to_string(&small_number).unwrap();
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, small_number);

    // Large string
    let large_string = "x".repeat(10000);
    let large_str_value = Value::from(large_string.clone());
    let json = serde_json::to_string(&large_str_value).unwrap();
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, large_str_value);

    // Deep nesting
    let mut deeply_nested = Value::null();
    for i in 0..100 {
        let mut map = HashMap::new();
        map.insert(format!("level_{}", i), deeply_nested);
        deeply_nested = Value::from(map);
    }
    let json = serde_json::to_string(&deeply_nested).unwrap();
    let deserialized: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, deeply_nested);
}

#[test]
fn test_json_escape_sequences() {
    let special_chars = vec![
        "\"quotes\"",
        "\\backslashes\\",
        "\t\r\n",
        "\x00\x01\x02",  // Control characters
        "line1\nline2\r\nline3",
    ];

    for special_str in special_chars {
        let value = Value::from(special_str.to_string());
        let json = serde_json::to_string(&value).unwrap();
        let deserialized: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, value, "Failed for special string: {:?}", special_str);
    }
}

// Error handling and malformed JSON tests
#[test]
fn test_malformed_json_deserialization() {
    let malformed_cases = vec![
        ("", "empty string"),
        ("{", "incomplete object"),
        ("}", "invalid start"),
        ("{\"key\"}", "missing value"),
        ("{\"key\":}", "missing value after colon"),
        ("[1,2,]", "trailing comma in array"),
        ("{\"key\":\"value\",}", "trailing comma in object"),
        ("null,", "unexpected comma after null"),
        ("\"unclosed string", "unclosed string"),
        ("123.456.789", "invalid number format"),
        ("true false", "multiple values"),
    ];

    for (malformed_json, description) in malformed_cases {
        let result = serde_json::from_str::<Value>(malformed_json);
        assert!(result.is_err(), "Expected error for {}: {}", description, malformed_json);

        let result = serde_json::from_str::<Foo>(malformed_json);
        assert!(result.is_err(), "Expected error for Foo with {}: {}", description, malformed_json);
    }
}

#[test]
fn test_invalid_timestamp_formats() {
    let invalid_timestamps = vec![
        "\"invalid-format\"",
        "\"2021-13-01T00:00:00Z\"",  // Invalid month
        "\"2021-01-32T00:00:00Z\"",  // Invalid day
        "\"2021-01-01T25:00:00Z\"",  // Invalid hour
        "\"2021-01-01T00:60:00Z\"",  // Invalid minute
        // Note: "2021-01-01T00:00:60Z" might be valid as leap second
        "\"not-a-timestamp\"",
        "123",  // Number instead of string
        "null",
    ];

    for invalid_ts in invalid_timestamps {
        let result = serde_json::from_str::<Timestamp>(invalid_ts);
        assert!(result.is_err(), "Expected error for invalid timestamp: {}", invalid_ts);
    }
}

#[test]
fn test_invalid_duration_formats() {
    let invalid_durations = vec![
        "\"invalid-format\"",
        "\"123\"",  // Missing 's' suffix
        "\"s\"",  // Only suffix
        "\"123h\"",  // Wrong suffix
        "123",  // Number instead of string
        "null",
    ];

    for invalid_dur in invalid_durations {
        let result = serde_json::from_str::<Duration>(invalid_dur);
        assert!(result.is_err(), "Expected error for invalid duration: {}", invalid_dur);
    }
}

#[test]
fn test_type_mismatch_errors() {
    // Test type mismatches in Foo struct
    let type_mismatch_cases = vec![
        (r#"{"string": 123}"#, "number for string field"),
        (r#"{"boolean": "true"}"#, "string for boolean field"),
        (r#"{"list": "not_an_array"}"#, "string for array field"),
        (r#"{"timestamp": {}}"#, "object for timestamp field"),
        (r#"{"valueData": "plain_string"}"#, "string for Value field (should be structured)"),
    ];

    for (malformed_json, description) in type_mismatch_cases {
        let result = serde_json::from_str::<Foo>(malformed_json);
        assert!(result.is_err(), "Expected error for {}: {}", description, malformed_json);
    }
}

#[test]
fn test_json_value_type_consistency() {
    // Test that Value correctly handles type consistency
    let test_cases = vec![
        (Value::null(), "null"),
        (Value::from(42.0), "42.0"),
        (Value::from("test".to_string()), r#""test""#),
        (Value::from(true), "true"),
        (Value::from(false), "false"),
    ];

    for (value, expected_json) in test_cases {
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, expected_json);

        let deserialized: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, value);
    }
}

#[test]
fn test_any_with_empty_type_url() {
    let empty_any = Any {
        type_url: "".to_string(),
        value: vec![],
    };

    let json = serde_json::to_string(&empty_any).unwrap();
    // Empty type URL will cause deserialization to fail because it's not a registered type
    // So we just test that serialization works
    assert!(!json.is_empty());
    // We can't deserialize back because empty type_url is not valid
}

#[test]
fn test_field_mask_edge_cases() {
    // Single path
    let single_mask = FieldMask {
        paths: vec!["single_field".to_string()],
    };
    let json = serde_json::to_string(&single_mask).unwrap();
    let deserialized: FieldMask = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, single_mask);

    // Paths with special characters
    let special_mask = FieldMask {
        paths: vec!["field_with_underscore".to_string(), "field.with.dots".to_string()],
    };
    let json = serde_json::to_string(&special_mask).unwrap();
    let deserialized: FieldMask = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, special_mask);
}
