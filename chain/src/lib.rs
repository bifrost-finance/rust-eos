#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod action;
pub mod action_receipt;
pub mod asset;
pub mod bytes;
pub mod block;
pub mod block_header;
pub mod block_timestamp;
pub mod checksum160;
pub mod checksum256;
pub mod checksum512;
pub mod error;
pub mod extension;
pub mod incremental_merkle;
pub mod merkle;
pub mod names;
pub mod ops;
pub mod permission_level;
pub mod producer_key;
pub mod producer_schedule;
pub mod producer_schedule_v2;
pub mod public_key;
pub mod signature;
pub mod symbol;
pub mod symbol_code;
pub mod time_point;
pub mod time_point_sec;
pub mod transaction;
pub mod unsigned_int;
pub mod utils;

pub use eosio_core_derive::*;

pub use self::{
    action::*,
    action_receipt::*,
    asset::*,
    bytes::*,
    block::*,
    block_header::*,
    block_timestamp::*,
    checksum160::*,
    checksum256::*,
    checksum512::*,
    error::*,
    incremental_merkle::*,
    merkle::*,
    extension::*,
    names::*,
    ops::*,
    permission_level::*,
    producer_key::*,
    producer_schedule::*,
    producer_schedule_v2::*,
    public_key::*,
    signature::*,
    symbol::*,
    symbol_code::*,
    time_point::*,
    time_point_sec::*,
    transaction::*,
    unsigned_int::*,
};
use alloc::vec;
use alloc::vec::Vec;

pub trait SerializeData: Write + NumBytes {
    fn to_serialize_data(&self) -> crate::Result<Vec<u8>> {
        let mut data = vec![0u8; self.num_bytes()];
        self.write(&mut data, &mut 0).map_err(crate::Error::BytesWriteError)?;
        Ok(data.to_vec())
    }
}

pub trait Digest: Clone + Write + NumBytes {
    fn digest(&self) -> crate::Result<Checksum256> {
        Checksum256::hash(self.clone())
    }
}
