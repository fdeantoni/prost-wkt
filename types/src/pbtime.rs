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

/*
 * From prost-types: https://github.com/danburkert/prost/blob/6113789f70b69709820becba4242824b4fb3ffec/prost-types/src/lib.rs
 */

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
            Ok(std::time::Duration::new(
                duration.seconds as u64,
                duration.nanos as u32,
            ))
        } else {
            Err(std::time::Duration::new(
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
    fn from(system_time: std::time::SystemTime) -> Timestamp {
        let (seconds, nanos) = match system_time.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => {
                let seconds = i64::try_from(duration.as_secs()).unwrap();
                (seconds, duration.subsec_nanos() as i32)
            }
            Err(error) => {
                let duration = error.duration();
                let seconds = i64::try_from(duration.as_secs()).unwrap();
                let nanos = duration.subsec_nanos() as i32;
                if nanos == 0 {
                    (-seconds, 0)
                } else {
                    (-seconds - 1, 1_000_000_000 - nanos)
                }
            }
        };
        Timestamp { seconds, nanos }
    }
}

impl From<Timestamp> for std::time::SystemTime {
    fn from(mut timestamp: Timestamp) -> std::time::SystemTime {
        timestamp.normalize();
        let system_time = if timestamp.seconds >= 0 {
            std::time::UNIX_EPOCH + time::Duration::from_secs(timestamp.seconds as u64)
        } else {
            std::time::UNIX_EPOCH - time::Duration::from_secs((-timestamp.seconds) as u64)
        };

        system_time + time::Duration::from_nanos(timestamp.nanos as u64)
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

#[cfg(test)]
mod tests {

    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use proptest::prelude::*;

    use crate::pbtime::*;
    use chrono::{DateTime, Utc};

    /*
     * First three tests from prost-types: https://github.com/danburkert/prost/blob/6113789f70b69709820becba4242824b4fb3ffec/prost-types/src/lib.rs
     */

    proptest! {
        #[test]
        fn check_system_time_roundtrip(
            system_time in SystemTime::arbitrary(),
        ) {
            prop_assert_eq!(SystemTime::from(Timestamp::from(system_time)), system_time);
        }
    }

    #[test]
    fn check_timestamp_negative_seconds() {
        // Representative tests for the case of timestamps before the UTC Epoch time:
        // validate the expected behaviour that "negative second values with fractions
        // must still have non-negative nanos values that count forward in time"
        // https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Timestamp
        //
        // To ensure cross-platform compatibility, all nanosecond values in these
        // tests are in minimum 100 ns increments.  This does not affect the general
        // character of the behaviour being tested, but ensures that the tests are
        // valid for both POSIX (1 ns precision) and Windows (100 ns precision).
        assert_eq!(
            Timestamp::from(UNIX_EPOCH - Duration::new(1_001, 0)),
            Timestamp {
                seconds: -1_001,
                nanos: 0
            }
        );
        assert_eq!(
            Timestamp::from(UNIX_EPOCH - Duration::new(0, 999_999_900)),
            Timestamp {
                seconds: -1,
                nanos: 100
            }
        );
        assert_eq!(
            Timestamp::from(UNIX_EPOCH - Duration::new(2_001_234, 12_300)),
            Timestamp {
                seconds: -2_001_235,
                nanos: 999_987_700
            }
        );
        assert_eq!(
            Timestamp::from(UNIX_EPOCH - Duration::new(768, 65_432_100)),
            Timestamp {
                seconds: -769,
                nanos: 934_567_900
            }
        );
    }

    #[test]
    fn check_timestamp_negative_seconds_1ns() {
        // UNIX-only test cases with 1 ns precision
        assert_eq!(
            Timestamp::from(UNIX_EPOCH - Duration::new(0, 999_999_999)),
            Timestamp {
                seconds: -1,
                nanos: 1
            }
        );
        assert_eq!(
            Timestamp::from(UNIX_EPOCH - Duration::new(1_234_567, 123)),
            Timestamp {
                seconds: -1_234_568,
                nanos: 999_999_877
            }
        );
        assert_eq!(
            Timestamp::from(UNIX_EPOCH - Duration::new(890, 987_654_321)),
            Timestamp {
                seconds: -891,
                nanos: 12_345_679
            }
        );
    }

    #[test]
    fn timestamp_test() {
        let ts = Timestamp::new(10, 10);
        let datetime_utc: DateTime<Utc> = ts.to_datetime();

        println!("{:?}", datetime_utc);
    }

}
