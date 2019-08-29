#![allow(dead_code)]
use std::fmt;
use crate::{base58, hash, error};
use std::str::FromStr;

/// An secp256k1 signature.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Signature(pub secp256k1::recovery::RecoverableSignature);

impl Signature {
    pub fn is_canonical(sig: secp256k1::recovery::RecoverableSignature) -> bool {
        let (_, pk) = sig.serialize_compact();

        (pk[0] & 0x80 == 0)
            && !((pk[0] == 0) && (pk[1] & 0x80 == 0))
            && (pk[32] & 0x80 == 0)
            && !((pk[32] == 0) && (pk[33] & 0x80 == 0))
    }
}

impl FromStr for Signature {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Signature, error::Error> {
        if !s.starts_with("SIG_K1_") {
            return Err(error::Error::Secp256k1(secp256k1::Error::InvalidSignature));
        }

        let s_hex = base58::from(&s[7..])?;
        let recid = match secp256k1::recovery::RecoveryId::from_i32((s_hex[0] - 4 - 27) as i32) {
            Ok(recid) => recid,
            Err(err) => return Err(err.into()),
        };
        // TODO verify with checksum
        let data = &s_hex[1..65];
        let _checksum = &s_hex[65..];
        let rec_sig = match secp256k1::recovery::RecoverableSignature::from_compact(&data, recid) {
            Ok(rec_sig) => rec_sig,
            Err(err) => return Err(err.into()),
        };

        Ok(Signature(rec_sig))
    }
}

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

#[cfg(test)]
mod test {
    use super::Signature;
    use std::str::FromStr;

    #[test]
    fn sig_from_str_should_work() {
        let sig_str = "SIG_K1_KomV6FEHKdtZxGDwhwSubEAcJ7VhtUQpEt5P6iDz33ic936aSXx87B2L56C8JLQkqNpp1W8ZXjrKiLHUEB4LCGeXvbtVuR";
        let sig = Signature::from_str(sig_str);
        assert!(sig.is_ok());
        assert_eq!(sig.unwrap().to_string(), sig_str);
    }

    #[test]
    fn sig_from_str_should_error() {
        let sig_str = "KomV6FEHKdtZxGDwhwSubEAcJ7VhtUQpEt5P6iDz33ic936aSXx87B2L56C8JLQkqNpp1W8ZXjrKiLHUEB4LCGeXvbtVuR";
        let sig = Signature::from_str(sig_str);
        assert!(sig.is_err());
    }
}
