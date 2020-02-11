//! <https://github.com/EOSIO/eosio.cdt/blob/796ff8bee9a0fc864f665a0a4d018e0ff18ac383/libraries/eosiolib/contracts/eosio/producer_schedule.hpp#L15-L45>
use codec::{Encode, Decode};
use crate::{AccountName, NumBytes, PublicKey, Read, Write};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Maps producer with its signing key, used for producer schedule
#[derive(Read, Write, NumBytes, Clone, Default, Debug, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct ProducerKey {
    /// Name of the producer
    pub producer_name: AccountName,
    /// Block signing key used by this producer
    #[cfg_attr(feature = "std", serde(deserialize_with = "super::string_to_public_key"))]
    pub block_signing_key: PublicKey,
}
