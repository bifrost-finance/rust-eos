//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/time.hpp#L134-L210>
use crate::{TimePoint, TimePointSec, NumBytes, Read, Write};
use chrono::{Utc, TimeZone, SecondsFormat};
#[cfg(feature = "std")]
use serde::ser::{Serialize, Serializer};

/// This class is used in the block headers to represent the block time
/// It is a parameterised class that takes an Epoch in milliseconds and
/// and an interval in milliseconds and computes the number of slots.
#[derive(Read, Write, NumBytes, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Default)]
#[eosio_core_root_path = "crate"]
pub struct BlockTimestamp(u32);

#[cfg(feature = "std")]
impl Serialize for BlockTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl BlockTimestamp {
    /// Time between blocks.
    pub const BLOCK_INTERVAL_MS: i32 = 500;
    /// Epoch is 2000-01-01T00:00.000Z.
    pub const BLOCK_TIMESTAMP_EPOCH: u64 = 946_684_800_000;

    /// Gets the milliseconds
    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

struct BlockTimestampVisitor;

impl<'de> ::serde::de::Visitor<'de> for BlockTimestampVisitor {
    type Value = BlockTimestamp;

    #[inline]
    fn expecting(
        &self,
        formatter: &mut ::std::fmt::Formatter,
    ) -> ::std::fmt::Result {
        formatter.write_str("a second timestamp as a number or string")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        match value.parse::<u32>() {
            Ok(n) => Ok(BlockTimestamp(n)),
            Err(e) => Err(::serde::de::Error::custom(e)),
        }
    }

    #[inline]
    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(BlockTimestamp(value))
    }
}

impl<'de> ::serde::de::Deserialize<'de> for BlockTimestamp {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(BlockTimestampVisitor)
    }
}

impl From<u32> for BlockTimestamp {
    #[inline]
    fn from(i: u32) -> Self {
        Self(i)
    }
}

impl From<BlockTimestamp> for u32 {
    #[inline]
    fn from(t: BlockTimestamp) -> Self {
        t.0
    }
}


impl From<TimePoint> for BlockTimestamp {
    #[inline]
    fn from(t: TimePoint) -> Self {
        let micro_since_epoch = t.time_since_epoch();
        let msec_since_epoch  = micro_since_epoch / 1_000_000;
        Self(((msec_since_epoch - BlockTimestamp::BLOCK_TIMESTAMP_EPOCH as i64) / BlockTimestamp::BLOCK_INTERVAL_MS as i64) as u32)
    }
}

impl From<TimePointSec> for BlockTimestamp {
    #[inline]
    fn from(t: TimePointSec) -> Self {
        let sec_since_epoch = t.sec_since_epoch();
        Self((((sec_since_epoch as u64 * 1000) - BlockTimestamp::BLOCK_TIMESTAMP_EPOCH) / BlockTimestamp::BLOCK_INTERVAL_MS as u64) as u32)
    }
}

impl core::fmt::Display for BlockTimestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let sec_since_epoch = (self.0 as i64 * BlockTimestamp::BLOCK_INTERVAL_MS as i64) + BlockTimestamp::BLOCK_TIMESTAMP_EPOCH as i64;
        let dt = Utc.timestamp_millis(sec_since_epoch);
        write!(f, "{}", dt.to_rfc3339_opts(SecondsFormat::Millis, true))
    }
}
