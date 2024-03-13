mod datetime;

use core::convert::TryFrom;
use core::str::FromStr;
use core::*;
use core::time;
use std::convert::TryInto;

use chrono::prelude::*;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

include!(concat!(env!("OUT_DIR"), "/pbtime/google.protobuf.rs"));

////////////////////////////////////////////////////////////////////////////////
/// FROM prost-types/src/lib.rs
/// with impl Name removed until stabilized
////////////////////////////////////////////////////////////////////////////////
const NANOS_PER_SECOND: i32 = 1_000_000_000;
const NANOS_MAX: i32 = NANOS_PER_SECOND - 1;

#[cfg(feature = "std")]
impl std::hash::Hash for Duration {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.seconds.hash(state);
        self.nanos.hash(state);
    }
}

impl Duration {
    /// Normalizes the duration to a canonical format.
    ///
    /// Based on [`google::protobuf::util::CreateNormalized`][1].
    ///
    /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L79-L100
    pub fn normalize(&mut self) {
        // Make sure nanos is in the range.
        if self.nanos <= -NANOS_PER_SECOND || self.nanos >= NANOS_PER_SECOND {
            if let Some(seconds) = self
                .seconds
                .checked_add((self.nanos / NANOS_PER_SECOND) as i64)
            {
                self.seconds = seconds;
                self.nanos %= NANOS_PER_SECOND;
            } else if self.nanos < 0 {
                // Negative overflow! Set to the least normal value.
                self.seconds = i64::MIN;
                self.nanos = -NANOS_MAX;
            } else {
                // Positive overflow! Set to the greatest normal value.
                self.seconds = i64::MAX;
                self.nanos = NANOS_MAX;
            }
        }

        // nanos should have the same sign as seconds.
        if self.seconds < 0 && self.nanos > 0 {
            if let Some(seconds) = self.seconds.checked_add(1) {
                self.seconds = seconds;
                self.nanos -= NANOS_PER_SECOND;
            } else {
                // Positive overflow! Set to the greatest normal value.
                debug_assert_eq!(self.seconds, i64::MAX);
                self.nanos = NANOS_MAX;
            }
        } else if self.seconds > 0 && self.nanos < 0 {
            if let Some(seconds) = self.seconds.checked_sub(1) {
                self.seconds = seconds;
                self.nanos += NANOS_PER_SECOND;
            } else {
                // Negative overflow! Set to the least normal value.
                debug_assert_eq!(self.seconds, i64::MIN);
                self.nanos = -NANOS_MAX;
            }
        }
        // TODO: should this be checked?
        // debug_assert!(self.seconds >= -315_576_000_000 && self.seconds <= 315_576_000_000,
        //               "invalid duration: {:?}", self);
    }
}

impl TryFrom<time::Duration> for Duration {
    type Error = DurationError;

    /// Converts a `std::time::Duration` to a `Duration`, failing if the duration is too large.
    fn try_from(duration: time::Duration) -> Result<Duration, DurationError> {
        let seconds = i64::try_from(duration.as_secs()).map_err(|_| DurationError::OutOfRange)?;
        let nanos = duration.subsec_nanos() as i32;

        let mut duration = Duration { seconds, nanos };
        duration.normalize();
        Ok(duration)
    }
}

impl TryFrom<Duration> for time::Duration {
    type Error = DurationError;

    /// Converts a `Duration` to a `std::time::Duration`, failing if the duration is negative.
    fn try_from(mut duration: Duration) -> Result<time::Duration, DurationError> {
        duration.normalize();
        if duration.seconds >= 0 && duration.nanos >= 0 {
            Ok(time::Duration::new(
                duration.seconds as u64,
                duration.nanos as u32,
            ))
        } else {
            Err(DurationError::NegativeDuration(time::Duration::new(
                (-duration.seconds) as u64,
                (-duration.nanos) as u32,
            )))
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = self.clone();
        d.normalize();
        if self.seconds < 0 && self.nanos < 0 {
            write!(f, "-")?;
        }
        write!(f, "{}", d.seconds.abs())?;

        // Format subseconds to either nothing, millis, micros, or nanos.
        let nanos = d.nanos.abs();
        if nanos == 0 {
            write!(f, "s")
        } else if nanos % 1_000_000 == 0 {
            write!(f, ".{:03}s", nanos / 1_000_000)
        } else if nanos % 1_000 == 0 {
            write!(f, ".{:06}s", nanos / 1_000)
        } else {
            write!(f, ".{:09}s", nanos)
        }
    }
}

/// A duration handling error.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum DurationError {
    /// Indicates failure to parse a [`Duration`] from a string.
    ///
    /// The [`Duration`] string format is specified in the [Protobuf JSON mapping specification][1].
    ///
    /// [1]: https://developers.google.com/protocol-buffers/docs/proto3#json
    ParseFailure,

    /// Indicates failure to convert a `prost_types::Duration` to a `std::time::Duration` because
    /// the duration is negative. The included `std::time::Duration` matches the magnitude of the
    /// original negative `prost_types::Duration`.
    NegativeDuration(time::Duration),

    /// Indicates failure to convert a `std::time::Duration` to a `prost_types::Duration`.
    ///
    /// Converting a `std::time::Duration` to a `prost_types::Duration` fails if the magnitude
    /// exceeds that representable by `prost_types::Duration`.
    OutOfRange,
}

impl fmt::Display for DurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DurationError::ParseFailure => write!(f, "failed to parse duration"),
            DurationError::NegativeDuration(duration) => {
                write!(f, "failed to convert negative duration: {:?}", duration)
            }
            DurationError::OutOfRange => {
                write!(f, "failed to convert duration out of range")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DurationError {}

impl FromStr for Duration {
    type Err = DurationError;

    fn from_str(s: &str) -> Result<Duration, DurationError> {
        datetime::parse_duration(s).ok_or(DurationError::ParseFailure)
    }
}

impl Timestamp {
    /// Normalizes the timestamp to a canonical format.
    ///
    /// Based on [`google::protobuf::util::CreateNormalized`][1].
    ///
    /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L59-L77
    pub fn normalize(&mut self) {
        // Make sure nanos is in the range.
        if self.nanos <= -NANOS_PER_SECOND || self.nanos >= NANOS_PER_SECOND {
            if let Some(seconds) = self
                .seconds
                .checked_add((self.nanos / NANOS_PER_SECOND) as i64)
            {
                self.seconds = seconds;
                self.nanos %= NANOS_PER_SECOND;
            } else if self.nanos < 0 {
                // Negative overflow! Set to the earliest normal value.
                self.seconds = i64::MIN;
                self.nanos = 0;
            } else {
                // Positive overflow! Set to the latest normal value.
                self.seconds = i64::MAX;
                self.nanos = 999_999_999;
            }
        }

        // For Timestamp nanos should be in the range [0, 999999999].
        if self.nanos < 0 {
            if let Some(seconds) = self.seconds.checked_sub(1) {
                self.seconds = seconds;
                self.nanos += NANOS_PER_SECOND;
            } else {
                // Negative overflow! Set to the earliest normal value.
                debug_assert_eq!(self.seconds, i64::MIN);
                self.nanos = 0;
            }
        }

        // TODO: should this be checked?
        // debug_assert!(self.seconds >= -62_135_596_800 && self.seconds <= 253_402_300_799,
        //               "invalid timestamp: {:?}", self);
    }

    /// Normalizes the timestamp to a canonical format, returning the original value if it cannot be
    /// normalized.
    ///
    /// Normalization is based on [`google::protobuf::util::CreateNormalized`][1].
    ///
    /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L59-L77
    pub fn try_normalize(mut self) -> Result<Timestamp, Timestamp> {
        let before = self.clone();
        self.normalize();
        // If the seconds value has changed, and is either i64::MIN or i64::MAX, then the timestamp
        // normalization overflowed.
        if (self.seconds == i64::MAX || self.seconds == i64::MIN) && self.seconds != before.seconds
        {
            Err(before)
        } else {
            Ok(self)
        }
    }

    /// Creates a new `Timestamp` at the start of the provided UTC date.
    pub fn date(year: i64, month: u8, day: u8) -> Result<Timestamp, TimestampError> {
        Timestamp::date_time_nanos(year, month, day, 0, 0, 0, 0)
    }

    /// Creates a new `Timestamp` instance with the provided UTC date and time.
    pub fn date_time(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Timestamp, TimestampError> {
        Timestamp::date_time_nanos(year, month, day, hour, minute, second, 0)
    }

    /// Creates a new `Timestamp` instance with the provided UTC date and time.
    pub fn date_time_nanos(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanos: u32,
    ) -> Result<Timestamp, TimestampError> {
        let date_time = datetime::DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanos,
        };

        if date_time.is_valid() {
            Ok(Timestamp::from(date_time))
        } else {
            Err(TimestampError::InvalidDateTime)
        }
    }
}

/// Implements the unstable/naive version of `Eq`: a basic equality check on the internal fields of the `Timestamp`.
/// This implies that `normalized_ts != non_normalized_ts` even if `normalized_ts == non_normalized_ts.normalized()`.
#[cfg(feature = "std")]
impl Eq for Timestamp {}

#[cfg(feature = "std")]
#[allow(clippy::derived_hash_with_manual_eq)] // Derived logic is correct: comparing the 2 fields for equality
impl std::hash::Hash for Timestamp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.seconds.hash(state);
        self.nanos.hash(state);
    }
}

#[cfg(feature = "std")]
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

/// A timestamp handling error.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum TimestampError {
    /// Indicates that a [`Timestamp`] could not be converted to
    /// [`SystemTime`][std::time::SystemTime] because it is out of range.
    ///
    /// The range of times that can be represented by `SystemTime` depends on the platform. All
    /// `Timestamp`s are likely representable on 64-bit Unix-like platforms, but other platforms,
    /// such as Windows and 32-bit Linux, may not be able to represent the full range of
    /// `Timestamp`s.
    OutOfSystemRange(Timestamp),

    /// An error indicating failure to parse a timestamp in RFC-3339 format.
    ParseFailure,

    /// Indicates an error when constructing a timestamp due to invalid date or time data.
    InvalidDateTime,
}

impl fmt::Display for TimestampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimestampError::OutOfSystemRange(timestamp) => {
                write!(
                    f,
                    "{} is not representable as a `SystemTime` because it is out of range",
                    timestamp
                )
            }
            TimestampError::ParseFailure => {
                write!(f, "failed to parse RFC-3339 formatted timestamp")
            }
            TimestampError::InvalidDateTime => {
                write!(f, "invalid date or time")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TimestampError {}

#[cfg(feature = "std")]
impl TryFrom<Timestamp> for std::time::SystemTime {
    type Error = TimestampError;

    fn try_from(mut timestamp: Timestamp) -> Result<std::time::SystemTime, Self::Error> {
        let orig_timestamp = timestamp.clone();
        timestamp.normalize();

        let system_time = if timestamp.seconds >= 0 {
            std::time::UNIX_EPOCH.checked_add(time::Duration::from_secs(timestamp.seconds as u64))
        } else {
            std::time::UNIX_EPOCH.checked_sub(time::Duration::from_secs(
                timestamp
                    .seconds
                    .checked_neg()
                    .ok_or_else(|| TimestampError::OutOfSystemRange(timestamp.clone()))?
                    as u64,
            ))
        };

        let system_time = system_time.and_then(|system_time| {
            system_time.checked_add(time::Duration::from_nanos(timestamp.nanos as u64))
        });

        system_time.ok_or(TimestampError::OutOfSystemRange(orig_timestamp))
    }
}

impl FromStr for Timestamp {
    type Err = TimestampError;

    fn from_str(s: &str) -> Result<Timestamp, TimestampError> {
        datetime::parse_timestamp(s).ok_or(TimestampError::ParseFailure)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        datetime::DateTime::from(self.clone()).fmt(f)
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Converts chrono's `NaiveDateTime` to `Timestamp`..
impl From<NaiveDateTime> for Timestamp {
    fn from(dt: NaiveDateTime) -> Self {
        Timestamp {
            seconds: dt.and_utc().timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }
}

/// Converts chrono's `DateTime<UTtc>` to `Timestamp`
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
        let mut value = val;
        // A call to `normalize` should capture all out-of-bound sitations hopefully
        // ensuring a panic never happens! Ideally this implementation should be
        // deprecated in favour of TryFrom but unfortunately having `TryFrom` along with
        // `From` causes a conflict.
        value.normalize();
        DateTime::from_timestamp(value.seconds, value.nanos as u32)
            .expect("invalid or out-of-range datetime")
    }
}

/// Converts proto duration to chrono's Duration
impl From<Duration> for chrono::Duration {
    fn from(val: Duration) -> Self {
        let mut value = val;
        // A call to `normalize` should capture all out-of-bound sitations hopefully
        // ensuring a panic never happens! Ideally this implementation should be
        // deprecated in favour of TryFrom but unfortunately having `TryFrom` along with
        // `From` causes a conflict.        
        value.normalize();
        let s = chrono::TimeDelta::try_seconds(value.seconds).expect("invalid or out-of-range seconds");
        let ns = chrono::Duration::nanoseconds(value.nanos as i64);
        s + ns
    }
}

/// Converts chrono Duration to proto duration
impl From<chrono::Duration> for Duration {
    fn from(val: chrono::Duration) -> Self {
        Duration {
            seconds: val.num_seconds(),
            nanos: (val.num_nanoseconds().unwrap() % 1_000_000_000) as i32,
        }
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
        let dt: DateTime<Utc> = ts.try_into().map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(format!("{dt:?}").as_str())
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
                        "Failed to parse {value} as datetime: {err:?}"
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
    fn serialize_duration() {
        let duration = Duration {
            seconds: 10,
            nanos: 100,
        };
        let json = serde_json::to_string_pretty(&duration).expect("json");
        println!("{json}");
        let back: Duration = serde_json::from_str(&json).expect("duration");
        assert_eq!(duration, back);
    }

    #[test]
    fn invalid_timestamp_test() {
        let ts = Timestamp {
            seconds: 10,
            nanos: 2000000000,
        };
        let datetime_utc: DateTime<Utc> = ts.into();

        println!("{datetime_utc:?}");
    }

    #[test]
    fn test_duration_conversion_pb_to_chrono() {
        let duration = Duration {
            seconds: 10,
            nanos: 100,
        };
        let chrono_duration: chrono::Duration = duration.into();
        assert_eq!(chrono_duration.num_seconds(), 10);
        assert_eq!((chrono_duration - chrono::Duration::try_seconds(10).expect("seconds")).num_nanoseconds(), Some(100));
    }

    #[test]
    fn test_duration_conversion_chrono_to_pb() {
        let chrono_duration = chrono::Duration::try_seconds(10).expect("seconds") + chrono::Duration::nanoseconds(100);
        let duration: Duration = chrono_duration.into();
        assert_eq!(duration.seconds, 10);
        assert_eq!(duration.nanos, 100);
    }
}
