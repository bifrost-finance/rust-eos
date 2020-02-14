//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/time.hpp#L79-L132>

use chrono::{SecondsFormat, TimeZone, Utc};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::{NumBytes, Read, TimePoint, Write};

/// A lower resolution `TimePoint` accurate only to seconds from 1970
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
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

    #[cfg(feature = "std")]
    pub fn now() -> Self {
        let now = Utc::now().timestamp();
        Self::from_unix_seconds(now as u32)
    }

    pub fn add_seconds(&mut self, t: u32) -> Self {
        self.0 += t;
        *self
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

#[cfg(feature = "std")]
impl std::fmt::Display for TimePointSec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dt = Utc.timestamp(self.sec_since_epoch() as i64, 0);
        write!(f, "{}", dt.to_rfc3339_opts(SecondsFormat::Secs, true))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn time_point_sec_should_be_ok() {
        let time_now = Utc::now().timestamp();
        let time_now_u32 = time_now as u32;
        let time_point_sec_now = TimePointSec::now().0;
        assert_eq!(time_now_u32, time_point_sec_now);
        let time_point_data = TimePoint::now();
        let time_point_data_from = (time_point_data.as_i64() as u32) / 1_000_000_u32;
        let time_point_sec_data = TimePointSec::from(time_point_data).0;
        assert_eq!(time_point_data_from, time_point_sec_data);
    }
}
