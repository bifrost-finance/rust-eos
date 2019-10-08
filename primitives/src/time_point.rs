//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/time.hpp#L49-L77>
use crate::{NumBytes, Read, Write};
use core::convert::{TryFrom, TryInto};
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};

/// High resolution time point in microseconds
#[derive(Read, Write, NumBytes, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Default)]
#[eosio_core_root_path = "crate"]
pub struct TimePoint(i64);

impl TimePoint {
    /// Gets the microseconds
    #[inline]
    pub const fn as_i64(self) -> i64 {
        self.0
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

impl core::fmt::Display for TimePoint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let d = UNIX_EPOCH + Duration::from_nanos(self.0 as u64);
        let datetime = DateTime::<Utc>::from(d);
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        write!(f, "{}", timestamp_str)
    }
}
