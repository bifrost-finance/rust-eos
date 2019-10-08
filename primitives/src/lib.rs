//#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod action;
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
pub mod message;
pub mod names;
pub mod ops;
pub mod permission_level;
pub mod producer_key;
pub mod producer_schedule;
pub mod public_key;
pub mod signature;
pub mod symbol;
pub mod symbol_code;
pub mod transaction;
pub mod time_point;
pub mod time_point_sec;
pub mod unsigned_int;

pub use eosio_core_derive::*;

pub use self::{
    action::*,
    asset::*,
    bytes::*,
    block::*,
    block_header::*,
    block_timestamp::*,
    checksum160::*,
    checksum256::*,
    checksum512::*,
    error::*,
    extension::*,
    message::*,
    names::*,
    ops::*,
    permission_level::*,
    producer_key::*,
    producer_schedule::*,
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
    fn to_serialize_data(&self) -> Vec<u8> {
        let mut data = vec![0u8; self.num_bytes()];
        self.write(&mut data, &mut 0).expect("write");
        data.to_vec()
    }
}
