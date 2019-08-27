#![allow(dead_code)]

use std::{io, fmt, str::FromStr};
use crypto::ripemd160::Ripemd160;
use crypto::digest::Digest;
use secp256k1::{self, Secp256k1, Message};
use crate::constant::*;
use crate::error;
use crate::secret::SecretKey;
use crate::hash::H160;
use crate::base58;
use crate::signature::Signature;

/// A Secp256k1 public key
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PublicKey {
    /// Whether this public key should be serialized as compressed
    pub compressed: bool,
    /// The actual Secp256k1 key
    pub key: secp256k1::PublicKey,
}

impl PublicKey {
    /// Write the public key into a writer
    pub fn write_into<W: io::Write>(&self, mut writer: W) {
        let write_res: io::Result<()> = if self.compressed {
            writer.write_all(&self.key.serialize())
        } else {
            writer.write_all(&self.key.serialize_uncompressed())
        };
        debug_assert!(write_res.is_ok());
    }

    /// Serialize the public key to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.write_into(&mut buf);

        buf
    }

    /// Serialize the public key to Eos format string
    pub fn to_eos_fmt(&self) -> String {
        let h160 = self.ripemd160();
        let mut public_key: [u8; PUBLIC_KEY_WITH_CHECKSUM_SIZE] = [0u8; PUBLIC_KEY_WITH_CHECKSUM_SIZE];
        public_key[..PUBLIC_KEY_SIZE].copy_from_slice(self.to_bytes().as_ref());
        public_key[PUBLIC_KEY_SIZE..].copy_from_slice(&h160.take()[..PUBLIC_KEY_CHECKSUM_SIZE]);

        format!("EOS{}", base58::encode_slice(&public_key))
    }

    /// Verify a signature on a message with public key.
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), error::Error> {
        let secp = Secp256k1::verification_only();
        let msg = Message::from_slice(&message).unwrap();

        match secp.verify(&msg, &signature.0, &self.key) {
            Ok(()) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    /// Deserialize a public key from a slice
    pub fn from_slice(data: &[u8]) -> Result<PublicKey, error::Error> {
        let compressed: bool = match data.len() {
            PUBLIC_KEY_SIZE => true,
            UNCOMPRESSED_PUBLIC_KEY_SIZE => false,
            len => { return Err(base58::Error::InvalidLength(len).into()); }
        };

        Ok(PublicKey {
            compressed,
            key: secp256k1::PublicKey::from_slice(data)?,
        })
    }


    /// Computes RIPEMD-160 cryptographic hash of key
    fn ripemd160(&self) -> H160 {
        let mut result = H160::default();
        let mut hasher = Ripemd160::new();
        hasher.input(&self.key.serialize());
        hasher.result(&mut *result);

        result
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.compressed {
            write!(f, "{}", self.to_eos_fmt())?;
        } else {
            for ch in &self.key.serialize_uncompressed()[..] {
                write!(f, "{:02x}", ch)?;
            }
        }

        Ok(())
    }
}

impl FromStr for PublicKey {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<PublicKey, error::Error> {
        let key = secp256k1::PublicKey::from_str(s)?;
        Ok(PublicKey {
            key,
            compressed: s.len() == 66,
        })
    }
}

impl<'a> From<&'a SecretKey> for PublicKey {
    /// Derive this public key from its corresponding `SecretKey`.
    fn from(sk: &SecretKey) -> PublicKey {
        let secp = Secp256k1::new();

        PublicKey {
            compressed: true,
            key: secp256k1::PublicKey::from_secret_key(&secp, &sk.key),
        }
    }
}
