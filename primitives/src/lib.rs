pub mod action;
pub mod asset;
pub mod bytes;
pub mod error;
pub mod names;
pub mod ops;
pub mod permission_level;
pub mod symbol;
pub mod symbol_code;
pub mod unsigned_int;

pub use eosio_core_derive::*;

pub use self::{
    action::*, asset::*, bytes::*, error::*, names::*, ops::*, permission_level::*,
    symbol::*, symbol_code::*, unsigned_int::*,
};

pub trait SerializeData: Write + NumBytes {
    fn to_serialize_data(&self) -> Vec<u8> {
        let mut data = vec![0u8; self.num_bytes()];
        self.write(&mut data, &mut 0).expect("write");
        data.to_vec()
    }
}
