//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/time.hpp#L49-L77>
use crate::{NumBytes, Read, Write};
use alloc::string::ToString;
use core::convert::{TryFrom, TryInto};
use chrono::{SecondsFormat, TimeZone, Utc};
#[cfg(feature = "std")]
use serde::{Deserialize, ser::{Serialize, Serializer}};

/// High resolution time point in microseconds
#[derive(Read, Write, NumBytes, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Default)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[eosio_core_root_path = "crate"]
pub struct TimePoint(i64);

#[cfg(feature = "std")]
impl Serialize for TimePoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TimePoint {
    /// Gets the microseconds
    #[inline]
    pub const fn as_i64(self) -> i64 {
        self.0
    }

    pub fn time_since_epoch(&self) -> i64 {
        self.0
    }

    pub fn from_unix_nano_seconds(nano_sec: i64) -> Self {
        Self(nano_sec)
    }

    #[cfg(feature = "std")]
    pub fn now() -> Self {
        let now = Utc::now().timestamp_nanos();
        Self::from_unix_nano_seconds(now)
    }
}

impl From<i64> for TimePoint {
    #[inline]
    fn from(i: i64) -> Self {
        Self(i)
    }
}

impl From<TimePoint> for i64 {
    #[inline]
    fn from(t: TimePoint) -> Self {
        t.0
    }
}

impl TryFrom<u64> for TimePoint {
    type Error = core::num::TryFromIntError;
    #[inline]
    fn try_from(i: u64) -> Result<Self, Self::Error> {
        Ok(i64::try_from(i)?.into())
    }
}

impl TryFrom<TimePoint> for u64 {
    type Error = core::num::TryFromIntError;
    #[inline]
    fn try_from(t: TimePoint) -> Result<Self, Self::Error> {
        t.as_i64().try_into()
    }
}

// // TODO: Duration ops similar to core::time::Duration

// #[derive(
//     Read, Write, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash,
// )]
// #[eosio_core_root_path = "crate"]
// pub struct Duration(i64);

#[cfg(feature = "std")]
impl std::fmt::Display for TimePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dt = Utc.timestamp_nanos(self.time_since_epoch());
        write!(f, "{}", dt.to_rfc3339_opts(SecondsFormat::Millis, true))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn time_point_error_code() {
        let max = u64::max_value();
        let timePoint_test = TimePoint::try_from(max);
        assert!(timePoint_test.is_err());
        let time = TimePoint(46);
        let a = TimePoint::from(46);
        assert_eq!(time,a);
        let test_data = a.as_i64();
        let timePoint_test_as_i64 = TimePoint::try_from(max);
        assert!(timePoint_test_as_i64.is_err());
    }
}
