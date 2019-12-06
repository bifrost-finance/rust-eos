//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/crypto.hpp#L22-L48>
use crate::{NumBytes, Read, UnsignedInt, Write, Signature};
#[cfg(feature = "std")]
use crate::BigArray;
use core::{
    convert::{TryFrom, TryInto},
    fmt, marker::PhantomData,
    str::FromStr
};
use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{Deserialize,
            Deserializer,
            de::{self, Visitor},
            ser::{Serialize, Serializer},
};

/// EOSIO Public Key
#[derive(Read, Write, NumBytes, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[eosio_core_root_path = "crate"]
pub struct PublicKey {
    /// Type of the public key, could be either K1 or R1
    pub type_: UnsignedInt,
    /// Bytes of the public key
    #[cfg_attr(feature = "std", serde(with = "BigArray"))]
    pub data: [u8; 33],
}

#[cfg(feature = "std")]
impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl PublicKey {
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub const fn to_bytes(&self) -> [u8; 33] {
        self.data
    }

    #[cfg(feature = "std")]
    pub fn verify(&self, hash: &[u8], signature: &Signature) -> crate::Result<()> {
        let keys = keys::public::PublicKey::try_from(self.clone())?;
        let sig: &keys::signature::Signature = &signature.clone().try_into()?;
        keys.verify_hash(hash, sig).map_err(crate::Error::VerificationError)
    }
}

#[cfg(feature = "std")]
impl TryFrom<PublicKey> for keys::public::PublicKey {
    type Error = crate::error::Error;
    fn try_from(pk: PublicKey) -> Result<Self, Self::Error> {
        keys::public::PublicKey::from_slice(&pk.data).map_err(Self::Error::Keys)
    }
}

#[cfg(feature = "std")]
impl Into<PublicKey> for keys::public::PublicKey {
    fn into(self) -> PublicKey {
        PublicKey {
            type_: Default::default(),
            data: self.key.serialize(),
        }
    }
}

#[cfg(feature = "std")]
impl FromStr for PublicKey {
    type Err = crate::error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pk = keys::public::PublicKey::from_str(s).map_err(Self::Err::Keys)?;
        Ok(pk.into())
    }
}

#[cfg(feature = "std")]
pub(crate) fn string_to_public_key<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de> + FromStr<Err = crate::error::Error>,
        D: Deserializer<'de>,
{
    struct StringToPublicKey<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringToPublicKey<T>
        where
            T: Deserialize<'de> + FromStr<Err = crate::error::Error>,
    {
        type Value = T;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
            where
                E: de::Error,
        {
            Ok(FromStr::from_str(value).map_err(|_| E::custom("public_key deserialization error."))?)
        }
    }
    deserializer.deserialize_any(StringToPublicKey(PhantomData))
}

impl Default for PublicKey {
    fn default() -> Self {
        Self {
            type_: UnsignedInt::default(),
            data: [0_u8; 33],
        }
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.as_bytes() == other.as_bytes()
    }
}

impl core::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.type_, f)?;
        core::fmt::Debug::fmt(self.as_bytes(), f)
    }
}

#[cfg(feature = "std")]
impl core::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Ok(pk) = keys::public::PublicKey::try_from(self.clone()) {
            write!(f, "{}", pk.to_string())
        } else {
            write!(f, "{}", hex::encode(self.data.as_ref()))
        }
    }
}

#[cfg(test)]
mod test {
    use core::convert::TryFrom;
    use core::str::FromStr;

    #[test]
    fn generate_public_key_from_key_str() {
        let sig_key = "EOS7y4hU89NJ658H1KmAdZ6A585bEVmSV8xBGJ3SbQM4Pt3pcLion";
        let pk = super::PublicKey::from_str(sig_key);
        assert!(pk.is_ok());
    }

    #[test]
    fn generate_public_key_from_invalid_key_str() {
        // this is a invalid public key string
        let sig_key = "EOS7y4hU89NJ658H1KmAdZ6A585bEVmSV8xBGJ3SbQM4Pt3pcLionwwwwww";
        let pk = super::PublicKey::from_str(sig_key);
        assert!(pk.is_err());
    }

    #[test]
    fn generate_public_key_from_secp256k1_public_key() {
        let sig_key = "EOS8FdQ4gt16pFcSiXAYCcHnkHTS2nNLFWGZXW5sioAdvQuMxKhAm";
        let secp_pk = keys::public::PublicKey::from_str(sig_key);
        assert!(secp_pk.is_ok());
        let pk = super::PublicKey::try_from(secp_pk.unwrap());
        assert!(pk.is_ok());
    }
}
