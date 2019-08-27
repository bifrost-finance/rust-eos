#![allow(dead_code)]

use secp256k1::{Secp256k1, Message};
use rand::{CryptoRng, Rng};
use crate::constant::*;
use crate::error;
use crate::public::PublicKey;
use crate::secret::SecretKey;
use crate::signature::Signature;


/// A secp256k1 keypair.
pub struct Keypair {
    /// The secret half of this keypair.
    pub sk: SecretKey,
    /// The public half of this keypair.
    pub pk: PublicKey,
}

impl Keypair {
    /// Convert this keypair to bytes.
    pub fn to_bytes(&self) -> [u8; KEYPAIR_LENGTH] {
        let mut bytes: [u8; KEYPAIR_LENGTH] = [0u8; KEYPAIR_LENGTH];

        bytes[..SECRET_KEY_SIZE].copy_from_slice(self.sk.to_bytes().as_slice());
        bytes[SECRET_KEY_SIZE..].copy_from_slice(self.pk.to_bytes().as_slice());

        bytes
    }

    /// Generate an secp256k1 keypair.
    pub fn generate<R>(csprng: &mut R) -> Keypair where R: CryptoRng + Rng {
        let secp = Secp256k1::new();
        let sk = SecretKey::generate(csprng);
        let pk = PublicKey::from(&sk);

        Keypair { sk, pk }
    }

    /// Generate an secp256k1 keypair from secret in WIF format
    pub fn from_secret_wif(wif: &str) -> Result<Keypair, error::Error> {
        let sk = SecretKey::from_wif(wif)?;
        let pk = PublicKey::from(&sk);

        Ok(Keypair { sk, pk })
    }

    /// Sign a message with this keypair's secret key.
    pub fn sign(&self, message: &[u8]) -> Result<Signature, error::Error> {
        self.sk.sign(&message)
    }

    /// Verify a signature on a message with this keypair's public key
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), error::Error> {
        self.pk.verify(&message, &signature)
    }
}
