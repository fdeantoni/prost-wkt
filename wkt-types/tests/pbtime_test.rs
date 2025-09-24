use prost_wkt_types::*;

#[test]
fn test_timestamp_json_serialization() {
    let timestamp = Timestamp {
        seconds: 1609459200, // 2021-01-01T00:00:00Z
        nanos: 123456789,
    };

    let json = serde_json::to_string(&timestamp).unwrap();
    assert_eq!(json, r#""2021-01-01T00:00:00.123456789Z""#);
}

#[test]
fn test_timestamp_json_deserialization() {
    let json = r#""2021-01-01T00:00:00.123456789Z""#;
    let timestamp: Timestamp = serde_json::from_str(json).unwrap();

    assert_eq!(timestamp.seconds, 1609459200);
    assert_eq!(timestamp.nanos, 123456789);
}

#[test]
fn test_timestamp_json_roundtrip() {
    let original = Timestamp {
        seconds: 1640995200, // 2022-01-01T00:00:00Z
        nanos: 500000000,
    };

    let json = serde_json::to_string(&original).unwrap();
    let decoded: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_timestamp_zero_serialization() {
    let zero = Timestamp::default();
    let json = serde_json::to_string(&zero).unwrap();
    assert_eq!(json, r#""1970-01-01T00:00:00Z""#);

    let decoded: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, zero);
}

#[test]
fn test_timestamp_negative_serialization() {
    let negative = Timestamp {
        seconds: -86400, // 1969-12-31T00:00:00Z
        nanos: 0,
    };

    let json = serde_json::to_string(&negative).unwrap();
    let decoded: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, negative);
}

#[test]
fn test_duration_json_serialization() {
    let duration = Duration {
        seconds: 3661, // 1h 1m 1s
        nanos: 500000000, // 0.5s
    };

    let json = serde_json::to_string(&duration).unwrap();
    assert_eq!(json, r#""3661.500000000s""#);
}

#[test]
fn test_duration_json_deserialization() {
    let json = r#""3661.500000000s""#;
    let duration: Duration = serde_json::from_str(json).unwrap();

    assert_eq!(duration.seconds, 3661);
    assert_eq!(duration.nanos, 500000000);
}

#[test]
fn test_duration_json_roundtrip() {
    let original = Duration {
        seconds: 7200, // 2 hours
        nanos: 250000000, // 0.25s
    };

    let json = serde_json::to_string(&original).unwrap();
    let decoded: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_duration_zero_serialization() {
    let zero = Duration::default();
    let json = serde_json::to_string(&zero).unwrap();
    assert_eq!(json, r#""0.000000000s""#);

    let decoded: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, zero);
}

#[test]
fn test_duration_negative_serialization() {
    let negative = Duration {
        seconds: -10,
        nanos: 0,
    };

    let json = serde_json::to_string(&negative).unwrap();
    let decoded: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, negative);
}

#[test]
fn test_duration_large_values_serialization() {
    let large = Duration {
        seconds: 315576000000, // ~10,000 years
        nanos: 0, // Use 0 nanos to avoid normalization issues
    };

    let json = serde_json::to_string(&large).unwrap();
    let decoded: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, large);
}

#[test]
fn test_timestamp_max_values_serialization() {
    let max_ts = Timestamp {
        seconds: 253402300799, // 9999-12-31T23:59:59Z
        nanos: 999999999,
    };

    let json = serde_json::to_string(&max_ts).unwrap();
    let decoded: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, max_ts);
}

#[test]
fn test_duration_microseconds_precision() {
    let duration = Duration {
        seconds: 1,
        nanos: 123456000, // 123.456 milliseconds
    };

    let json = serde_json::to_string(&duration).unwrap();
    let decoded: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, duration);
    assert_eq!(decoded.nanos, 123456000);
}

#[test]
fn test_timestamp_precise_nanos() {
    let timestamp = Timestamp {
        seconds: 1234567890,
        nanos: 987654321,
    };

    let json = serde_json::to_string(&timestamp).unwrap();
    let decoded: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, timestamp);
    assert_eq!(decoded.nanos, 987654321);
}

#[test]
fn test_duration_only_nanos() {
    let duration = Duration {
        seconds: 0,
        nanos: 123456789,
    };

    let json = serde_json::to_string(&duration).unwrap();
    let decoded: Duration = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, duration);
    assert!(json.starts_with(r#""0."#));
    assert!(json.ends_with(r#"s""#));
}

#[test]
fn test_timestamp_far_future() {
    let far_future = Timestamp {
        seconds: 4102444800, // 2100-01-01T00:00:00Z
        nanos: 0,
    };

    let json = serde_json::to_string(&far_future).unwrap();
    let decoded: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, far_future);
}

#[test]
fn test_timestamp_far_past() {
    let far_past = Timestamp {
        seconds: -2208988800, // 1900-01-01T00:00:00Z
        nanos: 0,
    };

    let json = serde_json::to_string(&far_past).unwrap();
    let decoded: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, far_past);
}