//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/crypto.hpp#L93-L120>
use crate::{NumBytes, Read, UnsignedInt, Write};

/// EOSIO Signature
#[derive(Read, Write, NumBytes, Clone)]
#[eosio_core_root_path = "crate"]
pub struct Signature {
    /// Type of the signature, could be either K1 or R1
    pub type_: UnsignedInt,
    /// Bytes of the signature
    pub data: [u8; 65],
}

impl Signature {
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub const fn to_bytes(&self) -> [u8; 65] {
        self.data
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self {
            type_: UnsignedInt::from(0u8),
            data: [1u8; 65],
        }
    }
}

impl core::fmt::Debug for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.type_, f)?;
        core::fmt::Debug::fmt(self.as_bytes(), f)
    }
}

impl PartialEq for Signature {
    #[inline]
    fn eq(&self, other: &Signature) -> bool {
        // TODO
        self.type_ == other.type_
    }
}

impl core::fmt::Display for Signature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", hex::encode(self.data.as_ref()))
    }
}
