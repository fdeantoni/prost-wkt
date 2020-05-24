use std::convert::TryFrom;
use std::str::FromStr;
use std::i32;
use std::i64;
use std::time;
use std::fmt;

use chrono::prelude::*;

use serde::ser::{Serialize, Serializer};
use serde::de::{self, Deserialize, Deserializer, Visitor};

include!(concat!(env!("OUT_DIR"), "/pbtime/google.protobuf.rs"));


// The Protobuf `Duration` and `Timestamp` types can't delegate to the standard library equivalents
// because the Protobuf versions are signed. To make them easier to work with, `From` conversions
// are defined in both directions.

const NANOS_PER_SECOND: i32 = 1_000_000_000;

impl Duration {
    /// Normalizes the duration to a canonical format.
    ///
    /// Based on [`google::protobuf::util::CreateNormalized`][1].
    /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L79-L100
    fn normalize(&mut self) {
        // Make sure nanos is in the range.
        if self.nanos <= -NANOS_PER_SECOND || self.nanos >= NANOS_PER_SECOND {
            self.seconds += (self.nanos / NANOS_PER_SECOND) as i64;
            self.nanos %= NANOS_PER_SECOND;
        }

        // nanos should have the same sign as seconds.
        if self.seconds < 0 && self.nanos > 0 {
            self.seconds += 1;
            self.nanos -= NANOS_PER_SECOND;
        } else if self.seconds > 0 && self.nanos < 0 {
            self.seconds -= 1;
            self.nanos += NANOS_PER_SECOND;
        }
        // TODO: should this be checked?
        // debug_assert!(self.seconds >= -315_576_000_000 && self.seconds <= 315_576_000_000,
        //               "invalid duration: {:?}", self);
    }
}

/// Converts a `std::time::Duration` to a `Duration`.
impl From<std::time::Duration> for Duration {
    fn from(duration: std::time::Duration) -> Duration {
        let seconds = duration.as_secs();
        let seconds = if seconds > i64::MAX as u64 {
            i64::MAX
        } else {
            seconds as i64
        };
        let nanos = duration.subsec_nanos();
        let nanos = if nanos > i32::MAX as u32 {
            i32::MAX
        } else {
            nanos as i32
        };
        let mut duration = Duration { seconds, nanos };
        duration.normalize();
        duration
    }
}

impl TryFrom<Duration> for std::time::Duration {
    type Error = std::time::Duration;

    /// Converts a `Duration` to a result containing a positive (`Ok`) or negative (`Err`)
    /// `std::time::Duration`.
    fn try_from(mut duration: Duration) -> Result<std::time::Duration, std::time::Duration> {
        duration.normalize();
        if duration.seconds >= 0 {
            Ok(time::Duration::new(
                duration.seconds as u64,
                duration.nanos as u32,
            ))
        } else {
            Err(time::Duration::new(
                (-duration.seconds) as u64,
                (-duration.nanos) as u32,
            ))
        }
    }
}

impl Timestamp {
    /// Normalizes the timestamp to a canonical format.
    ///
    /// Based on [`google::protobuf::util::CreateNormalized`][1].
    /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L59-L77
    fn normalize(&mut self) {
        // Make sure nanos is in the range.
        if self.nanos <= -NANOS_PER_SECOND || self.nanos >= NANOS_PER_SECOND {
            self.seconds += (self.nanos / NANOS_PER_SECOND) as i64;
            self.nanos %= NANOS_PER_SECOND;
        }

        // For Timestamp nanos should be in the range [0, 999999999].
        if self.nanos < 0 {
            self.seconds -= 1;
            self.nanos += NANOS_PER_SECOND;
        }

        // TODO: should this be checked?
        // debug_assert!(self.seconds >= -62_135_596_800 && self.seconds <= 253_402_300_799,
        //               "invalid timestamp: {:?}", self);
    }

    pub fn new(seconds: i64, nanos: i32) -> Self {
        let mut ts = Timestamp {
            seconds,
            nanos
        };
        ts.normalize();
        ts
    }

    pub fn to_datetime(&self) -> DateTime<Utc> {
        let dt = NaiveDateTime::from_timestamp(self.seconds, self.nanos as u32);
        DateTime::from_utc(dt, Utc)
    }

}

/// Converts a `std::time::SystemTime` to a `Timestamp`.
impl From<std::time::SystemTime> for Timestamp {
    fn from(time: time::SystemTime) -> Timestamp {
        let duration = Duration::from(time.duration_since(time::UNIX_EPOCH).unwrap());
        Timestamp {
            seconds: duration.seconds,
            nanos: duration.nanos,
        }
    }
}

impl TryFrom<Timestamp> for std::time::SystemTime {
    type Error = std::time::Duration;

    /// Converts a `Timestamp` to a `SystemTime`, or if the timestamp falls before the Unix epoch,
    /// a duration containing the difference.
    fn try_from(mut timestamp: Timestamp) -> Result<std::time::SystemTime, std::time::Duration> {
        timestamp.normalize();
        if timestamp.seconds >= 0 {
            Ok(time::UNIX_EPOCH
                + time::Duration::new(timestamp.seconds as u64, timestamp.nanos as u32))
        } else {
            let mut duration = Duration {
                seconds: -timestamp.seconds,
                nanos: timestamp.nanos,
            };
            duration.normalize();
            Err(time::Duration::new(
                duration.seconds as u64,
                duration.nanos as u32,
            ))
        }
    }
}

/// Converts chrono's `NaiveDateTime` to `Timestamp`..
impl From<NaiveDateTime> for Timestamp {
    fn from(dt: NaiveDateTime) -> Self {
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32
        }
    }
}

/// Converts chrono's `DateTime<UTtc>` to `Timestamp`..
impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32
        }
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut ts = Timestamp {
            seconds: self.seconds,
            nanos: self.nanos
        };
        ts.normalize();
        let dt = ts.to_datetime();
        serializer.serialize_str(format!("{:?}", dt).as_str())
    }
}

impl<'de> Deserialize<'de> for Timestamp  {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {

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
                let utc: DateTime<Utc> = chrono::DateTime::from_str(value).unwrap();
                let ts = Timestamp::from(utc);
                Ok(ts)
            }
        }
        deserializer.deserialize_str(TimestampVisitor)
    }
}
