//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/crypto.hpp#L93-L120>
#[cfg(feature = "std")]
use crate::BigArray;
use crate::{NumBytes, Read, UnsignedInt, Write};
use core::{
    convert::TryInto,
    str::FromStr,
};
use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{Deserialize, ser::{Serialize, Serializer}};

/// EOSIO Signature
#[derive(Read, Write, NumBytes, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[eosio_core_root_path = "crate"]
pub struct Signature {
    /// Type of the signature, could be either K1 or R1
    pub type_: UnsignedInt,
    /// Bytes of the signature
    #[cfg_attr(feature = "std", serde(with = "BigArray"))]
    pub data: [u8; 65],
}

#[cfg(feature = "std")]
impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
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
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.type_, f)?;
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

#[cfg(feature = "std")]
impl core::fmt::Display for Signature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Ok(sig) = keys::signature::Signature::from_compact(&self.data) {
            write!(f, "{}", sig)
        } else {
            write!(f, "{}", hex::encode(self.data.as_ref()))
        }
    }
}

#[cfg(feature = "std")]
impl From<keys::signature::Signature> for Signature {
    fn from(sig: keys::signature::Signature) -> Self {
        Signature {
            type_: UnsignedInt::from(0u8),
            data: sig.serialize_compact()
        }
    }
}

#[cfg(feature = "std")]
impl TryInto<keys::signature::Signature> for Signature {
    type Error = crate::Error;
    fn try_into(self) -> Result<keys::signature::Signature, Self::Error> {
        keys::signature::Signature::from_compact(&self.data).map_err(crate::Error::Keys)
    }
}

#[cfg(feature = "std")]
impl FromStr for Signature {
    type Err = keys::error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key_sig = keys::signature::Signature::from_str(s)?;
        Ok(Signature::from(key_sig))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpack_signature_should_work() {
        let data = hex::decode("00206b22f146d8bfe03a7a03b760cb2539409b05f9961543ee41c31f0cf493267b8c244d1517a6aa67cf47f294755d9e2fb5dda6779f5d88d6e4461f380a2b02964b").unwrap();
        let mut pos = 0;
        let sig = Signature::read(&data.as_slice(), &mut pos);
        assert!(sig.is_ok());
        assert_eq!(pos, 20);
    }

    #[test]
    fn signature_display_should_work() {
        let sig = Signature {
            type_: UnsignedInt::from(0u8),
            data: [0u8; 65],
        };
        assert_eq!(sig, Signature::default());
    }

    #[test]
    fn signture_from_str_should_work() {
        let key_sig = "SIG_K1_KYt8J2dEYCVg6j9kZes8vVNdNUrRUy35pAy1ZPPNVFhv1uWQB5G5qC5X6UasuWqejyRiLgH4e3GZfSKs83Ey8BKvP6jdHQ";
        let sig = Signature::from_str(key_sig);
        assert!(sig.is_ok());
    }

    #[test]
    fn signture_from_invalid_str_should_not_work() {
        let key_sig = "SIG_K1_KYt8J2dEYCVg6j9kZes8vVNdNUrRUy35pAy1ZPPNVFhv1uWQB5G5qC5X6UasuWqejyRiLgH4e3GZfSKs83Ey8BKvP6jdHQ11111";
        let sig = Signature::from_str(key_sig);
        assert!(sig.is_err());
    }
}
