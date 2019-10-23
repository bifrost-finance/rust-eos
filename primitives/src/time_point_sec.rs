//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/time.hpp#L79-L132>
use chrono::{SecondsFormat, TimeZone, Utc};

use crate::{NumBytes, Read, TimePoint, Write};

/// A lower resolution `TimePoint` accurate only to seconds from 1970
#[derive(Read, Write, NumBytes, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Default)]
#[eosio_core_root_path = "crate"]
pub struct TimePointSec(u32);

impl TimePointSec {
    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    pub fn from_unix_seconds(sec: u32) -> Self {
        TimePointSec(sec)
    }

    pub fn sec_since_epoch(&self) -> u32 {
        self.0
    }

    pub fn now() -> Self {
        let now = Utc::now().timestamp();
        Self::from_unix_seconds(now as u32)
    }
}

impl From<u32> for TimePointSec {
    #[inline]
    fn from(i: u32) -> Self {
        Self(i)
    }
}

impl From<TimePointSec> for u32 {
    #[inline]
    fn from(t: TimePointSec) -> Self {
        t.0
    }
}

impl From<TimePoint> for TimePointSec {
    #[inline]
    fn from(t: TimePoint) -> Self {
        Self((t.as_i64() as u32) / 1_000_000_u32)
    }
}

impl core::fmt::Display for TimePointSec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let dt = Utc.timestamp(self.sec_since_epoch() as i64, 0);
        write!(f, "{}", dt.to_rfc3339_opts(SecondsFormat::Secs, true))
    }
}
