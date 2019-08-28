#![allow(dead_code)]
use std::fmt;
use crate::{base58, hash};

/// An secp256k1 signature.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Signature(pub secp256k1::recovery::RecoverableSignature);

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (recovery_id, sig) = self.0.serialize_compact();

        // See https://github.com/EOSIO/fc/blob/f4755d330faf9d2342d646a93f9a27bf68ca759e/src/crypto/elliptic_impl_priv.cpp
        let mut checksum_data: [u8; 67] = [0u8; 67];
        checksum_data[0] = recovery_id.to_i32() as u8 + 27 + 4;
        checksum_data[1..65].copy_from_slice(&sig[..]);
        checksum_data[65..].copy_from_slice(b"K1");

        // Compute ripemd160 checksum
        let checksum_h160 = hash::ripemd160(&checksum_data);
        let checksum = &checksum_h160.take()[..4];

        // Signature slice
        let mut sig_slice: [u8; 69] = [0u8; 69];
        sig_slice[..65].copy_from_slice(&checksum_data[..65]);
        sig_slice[65..].copy_from_slice(&checksum[..]);

        write!(f, "SIG_K1_{}", base58::encode_slice(&sig_slice))?;

        Ok(())
    }
}
