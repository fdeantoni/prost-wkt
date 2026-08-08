#![cfg(feature = "rkyv")]

use prost_wkt_types::{Any, ArchivedTimestamp, Duration, Empty, FieldMask, Timestamp};
use rkyv::rancor::Error;

/// The shape prost generates for an `optional` message field.
#[derive(Debug, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
struct Event {
    name: String,
    at: Option<Timestamp>,
    took: Option<Duration>,
}

#[test]
fn timestamp_round_trip() {
    let ts = Timestamp {
        seconds: 1_590_409_197,
        nanos: 755_998_000,
    };
    let bytes = rkyv::to_bytes::<Error>(&ts).unwrap();
    assert_eq!(rkyv::from_bytes::<Timestamp, Error>(&bytes).unwrap(), ts);
}

#[test]
fn timestamp_accessed_without_deserializing() {
    let ts = Timestamp {
        seconds: -62_135_596_800,
        nanos: 1,
    };
    let bytes = rkyv::to_bytes::<Error>(&ts).unwrap();
    let archived = rkyv::access::<ArchivedTimestamp, Error>(&bytes).unwrap();
    assert_eq!(archived.seconds.to_native(), ts.seconds);
    assert_eq!(archived.nanos.to_native(), ts.nanos);
}

#[test]
fn duration_round_trip() {
    let d = Duration {
        seconds: -315_576_000_000,
        nanos: -999_999_999,
    };
    let bytes = rkyv::to_bytes::<Error>(&d).unwrap();
    assert_eq!(rkyv::from_bytes::<Duration, Error>(&bytes).unwrap(), d);
}

#[test]
fn optional_fields_round_trip() {
    for event in [
        Event {
            name: "start".to_string(),
            at: Some(Timestamp {
                seconds: 1,
                nanos: 2,
            }),
            took: Some(Duration {
                seconds: 3,
                nanos: 4,
            }),
        },
        Event {
            name: "unset".to_string(),
            at: None,
            took: None,
        },
    ] {
        let bytes = rkyv::to_bytes::<Error>(&event).unwrap();
        assert_eq!(rkyv::from_bytes::<Event, Error>(&bytes).unwrap(), event);
    }
}

#[test]
fn empty_and_field_mask_round_trip() {
    let bytes = rkyv::to_bytes::<Error>(&Empty {}).unwrap();
    assert_eq!(rkyv::from_bytes::<Empty, Error>(&bytes).unwrap(), Empty {});

    let mask = FieldMask {
        paths: vec!["f.a".to_string(), "f.b.d".to_string()],
    };
    let bytes = rkyv::to_bytes::<Error>(&mask).unwrap();
    assert_eq!(rkyv::from_bytes::<FieldMask, Error>(&bytes).unwrap(), mask);
}

#[test]
fn any_survives_and_still_unpacks() {
    let ts = Timestamp {
        seconds: 99,
        nanos: 100,
    };
    let any = Any::try_pack(ts).unwrap();
    let bytes = rkyv::to_bytes::<Error>(&any).unwrap();
    let restored = rkyv::from_bytes::<Any, Error>(&bytes).unwrap();
    assert_eq!(restored, any);
    assert_eq!(restored.unpack_as(Timestamp::default()).unwrap(), ts);
}
