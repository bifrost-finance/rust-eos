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
            data: [0u8; 65],
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
        let sig = keys::signature::Signature::from_compact(&self.data);
        if sig.is_ok() {
            write!(f, "{}", sig.unwrap())
        } else {
            write!(f, "{}", hex::encode(self.data.as_ref()))
        }
    }
}

impl From<keys::signature::Signature> for Signature {
    fn from(sig: keys::signature::Signature) -> Self {
        Signature {
            type_: UnsignedInt::from(0u8),
            data: sig.serialize_compact()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpack_signature_should_work() {
        let data = hex::decode("00206b22f146d8bfe03a7a03b760cb2539409b05f9961543ee41c31f0cf493267b8c244d1517a6aa67cf47f294755d9e2fb5dda6779f5d88d6e4461f380a2b02964b").unwrap();
        let mut pos = 0;
        let sig = Signature::read(&data.as_slice(), &mut pos).unwrap();
        dbg!(&sig);
        dbg!(&pos);
    }

    #[test]
    fn signature_display_should_work() {
        println!("{}", Signature::default());
    }
}
