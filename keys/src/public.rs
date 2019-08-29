use std::{io, fmt, str::FromStr};
use crypto::digest::Digest;
use secp256k1::{self, Secp256k1, Message};
use crate::constant::*;
use crate::{error, hash};
use crate::secret::SecretKey;
use crate::base58;
use crate::signature::Signature;
use crypto::sha2::Sha256;

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
        let h160 = hash::ripemd160(&self.key.serialize());
        let mut public_key: [u8; PUBLIC_KEY_WITH_CHECKSUM_SIZE] = [0u8; PUBLIC_KEY_WITH_CHECKSUM_SIZE];
        public_key[..PUBLIC_KEY_SIZE].copy_from_slice(self.to_bytes().as_ref());
        public_key[PUBLIC_KEY_SIZE..].copy_from_slice(&h160.take()[..PUBLIC_KEY_CHECKSUM_SIZE]);

        format!("EOS{}", base58::encode_slice(&public_key))
    }

    /// Verify a signature on a message with public key.
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), error::Error> {
        let mut msg = [0u8; 32];
        let mut hasher = Sha256::new();
        hasher.input(&message);
        hasher.result(&mut msg);

        self.verify_hash(&msg, &signature)
    }

    /// Verify a signature on a hash with public key.
    pub fn verify_hash(&self, hash: &[u8], signature: &Signature) -> Result<(), error::Error> {
        let secp = Secp256k1::verification_only();
        let msg = Message::from_slice(&hash).unwrap();

        match secp.verify(&msg, &signature.to_standard(), &self.key) {
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
        if !s.starts_with("EOS") {
            return Err(error::Error::Secp256k1(secp256k1::Error::InvalidPublicKey));
        }

        let s_hex = base58::from(&s[3..])?;
        let raw = &s_hex[..PUBLIC_KEY_SIZE];
        // TODO verify with checksum
        let _checksum = &s_hex[PUBLIC_KEY_SIZE..];
        let key = secp256k1::PublicKey::from_slice(&raw)?;

        Ok(PublicKey { key, compressed: true })
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

#[cfg(test)]
mod test {
    use super::PublicKey;
    use std::str::FromStr;
    use crate::error;
    use crate::signature::Signature;
    use secp256k1::Error::IncorrectSignature;
    use crate::error::Error::Secp256k1;

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
        assert_eq!(vfy, Err(Secp256k1(IncorrectSignature)));
    }
}
