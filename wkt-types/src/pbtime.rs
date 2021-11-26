use core::convert::TryFrom;
use core::str::FromStr;
use core::*;

use chrono::prelude::*;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

include!(concat!(env!("OUT_DIR"), "/pbtime/google.protobuf.rs"));
include!(concat!(env!("OUT_DIR"), "/prost_snippet.rs"));


/// Converts chrono's `NaiveDateTime` to `Timestamp`..
impl From<NaiveDateTime> for Timestamp {
    fn from(dt: NaiveDateTime) -> Self {
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }
}

/// Converts chrono's `DateTime<UTtc>` to `Timestamp`..
impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }
}

/// Converts proto timestamp to chrono's DateTime<Utc>
impl From<Timestamp> for DateTime<Utc> {
    fn from(val: Timestamp) -> Self {
        let dt = NaiveDateTime::from_timestamp(val.seconds, val.nanos as u32);
        DateTime::from_utc(dt, Utc)
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut ts = Timestamp {
            seconds: self.seconds,
            nanos: self.nanos,
        };
        ts.normalize();
        let dt: DateTime<Utc> = ts.into();
        serializer.serialize_str(format!("{:?}", dt).as_str())
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimestampVisitor;

        impl<'de> Visitor<'de> for TimestampVisitor {
            type Value = Timestamp;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Timestamp in RFC3339 format")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let utc: DateTime<Utc> = chrono::DateTime::from_str(value).map_err(|err| {
                    serde::de::Error::custom(format!(
                        "Failed to parse {} as datetime: {:?}",
                        value, err
                    ))
                })?;
                let ts = Timestamp::from(utc);
                Ok(ts)
            }
        }
        deserializer.deserialize_str(TimestampVisitor)
    }
}

#[cfg(test)]
mod tests {

    use crate::pbtime::*;
    use chrono::{DateTime, Utc};

    #[test]
    fn timestamp_test() {
        let ts = Timestamp {
            seconds: 10,
            nanos: 10,
        };
        let datetime_utc: DateTime<Utc> = ts.into();

        println!("{:?}", datetime_utc);
    }
}
