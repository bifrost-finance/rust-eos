use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use bitcoin_hashes::{sha256, Hash as HashTrait};
use core::{fmt, str::FromStr};
use crate::constant::*;
use crate::{error, hash};
use crate::secret::SecretKey;
use crate::base58;
use crate::signature::Signature;

/// A Secp256k1 public key
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PublicKey {
    /// Whether this public key should be serialized as compressed
    pub compressed: bool,
    /// The actual Secp256k1 key
    pub key: secp256k1::PublicKey,
}

impl PublicKey {

    /// Serialize the public key to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        if self.compressed {
            self.key.serialize_compressed().to_vec()
        } else {
            self.key.serialize().to_vec()
        }
    }

    /// Serialize the public key to Eos format string
    pub fn to_eos_fmt(&self) -> String {
        let h160 = hash::ripemd160(&self.key.serialize_compressed());
        let mut public_key = [0u8; PUBLIC_KEY_WITH_CHECKSUM_SIZE];
        public_key[..PUBLIC_KEY_SIZE].copy_from_slice(self.to_bytes().as_ref());
        public_key[PUBLIC_KEY_SIZE..].copy_from_slice(&h160.take()[..PUBLIC_KEY_CHECKSUM_SIZE]);

        format!("EOS{}", base58::encode_slice(&public_key))
    }

    /// Verify a signature on a message with public key.
    pub fn verify(&self, message_slice: &[u8], signature: &Signature) -> crate::Result<()> {
        let msg_hash = sha256::Hash::hash(&message_slice);
        self.verify_hash(&msg_hash, &signature)
    }

    /// Verify a signature on a hash with public key.
    pub fn verify_hash(&self, hash: &[u8], signature: &Signature) -> crate::Result<()> {
        let msg = secp256k1::Message::parse_slice(&hash)?;
        match secp256k1::verify(&msg, &signature.sig, &self.key) {
            true => Ok(()),
            false => Err(crate::error::Error::VerifyFailed),
        }
    }

    /// Deserialize a public key from a slice
    pub fn from_slice(data: &[u8]) -> crate::Result<PublicKey> {
        let compressed: bool = match data.len() {
            PUBLIC_KEY_SIZE => true,
            UNCOMPRESSED_PUBLIC_KEY_SIZE => false,
            len => { return Err(base58::Error::InvalidLength(len).into()); }
        };

        Ok(PublicKey {
            compressed,
            key: secp256k1::PublicKey::parse_slice(&data, Some(secp256k1::PublicKeyFormat::Compressed))?,
        })
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.compressed {
            write!(f, "{}", self.to_eos_fmt())?;
        } else {
            for ch in &self.key.serialize()[..] {
                write!(f, "{:02x}", ch)?;
            }
        }

        Ok(())
    }
}

impl FromStr for PublicKey {
    type Err = error::Error;
    fn from_str(s: &str) -> crate::Result<PublicKey> {
        if !s.starts_with("EOS") {
            return Err(secp256k1::Error::InvalidPublicKey.into());
        }

        let s_hex = base58::from(&s[3..])?;
        let format = match s_hex.len() {
            PUBLIC_KEY_WITH_CHECKSUM_SIZE => secp256k1::PublicKeyFormat::Compressed,
            _ => secp256k1::PublicKeyFormat::Full,
        };
        let raw = &s_hex[..PUBLIC_KEY_SIZE];
        // TODO verify with checksum
        let _checksum = &s_hex[PUBLIC_KEY_SIZE..];
        let key = secp256k1::PublicKey::parse_slice(&raw, Some(format))?;

        Ok(PublicKey { key, compressed: true })
    }
}

impl<'a> From<&'a SecretKey> for PublicKey {
    /// Derive this public key from its corresponding `SecretKey`.
    fn from(sk: &SecretKey) -> PublicKey {
        let pk = secp256k1::PublicKey::from_secret_key(&sk.key);

        PublicKey {
            compressed: true,
            key: pk,
        }
    }
}

#[cfg(test)]
mod test {
    use super::PublicKey;
    use core::str::FromStr;
    use crate::error;
    use crate::signature::Signature;
    use secp256k1;
    use alloc::string::ToString;

    #[test]
    fn pk_from_str_should_work() {
        let pk_str = "EOS8FdQ4gt16pFcSiXAYCcHnkHTS2nNLFWGZXW5sioAdvQuMxKhAm";
        let pk = PublicKey::from_str(pk_str);
        assert!(pk.is_ok());
        assert_eq!(pk.unwrap().to_string(), pk_str);
    }

    #[test]
    fn pk_from_str_should_error() {
        let pk_str = "8FdQ4gt16pFcSiXAYCcHnkHTS2nNLFWGZXW5sioAdvQuMxKhAm";
        let pk = PublicKey::from_str(pk_str);
        assert!(pk.is_err());
        assert_eq!(pk.unwrap_err(), error::Error::Secp256k1(secp256k1::Error::InvalidPublicKey));
    }

    #[test]
    fn pk_verify_should_work() {
        let pk_str = "EOS86jwjSu9YkD4JDJ7nGK1Rx2SmvNMQ3XiKrvFndABzLDPwk1ZHx";
        let sig_str = "SIG_K1_KomV6FEHKdtZxGDwhwSubEAcJ7VhtUQpEt5P6iDz33ic936aSXx87B2L56C8JLQkqNpp1W8ZXjrKiLHUEB4LCGeXvbtVuR";

        let pk = PublicKey::from_str(pk_str);
        assert!(pk.is_ok());
        let sig = Signature::from_str(sig_str);
        assert!(sig.is_ok());

        let vfy = pk.unwrap().verify("hello".as_bytes(), &sig.unwrap());
        assert!(vfy.is_ok());
    }

    #[test]
    fn pk_verify_should_error() {
        let pk_str = "EOS86jwjSu9YkD4JDJ7nGK1Rx2SmvNMQ3XiKrvFndABzLDPwk1ZHx";
        let sig_str = "SIG_K1_KomV6FEHKdtZxGDwhwSubEAcJ7VhtUQpEt5P6iDz33ic936aSXx87B2L56C8JLQkqNpp1W8ZXjrKiLHUEB4LCGeXvbtVuR";

        let pk = PublicKey::from_str(pk_str);
        assert!(pk.is_ok());
        let sig = Signature::from_str(sig_str);
        assert!(sig.is_ok());

        let vfy = pk.unwrap().verify("world".as_bytes(), &sig.unwrap());
        assert!(vfy.is_err());
        assert_eq!(vfy, Err(crate::error::Error::VerifyFailed));
    }
}
