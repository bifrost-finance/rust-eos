use std::fmt;
use crate::{base58, hash, error};
use std::str::FromStr;
use secp256k1;

/// An secp256k1 signature.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Signature {
    pub recv_id: secp256k1::RecoveryId,
    pub sig: secp256k1::Signature,
}

impl FromStr for Signature {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Signature, error::Error> {
        if !s.starts_with("SIG_K1_") {
            return Err(error::Error::Secp256k1(secp256k1::Error::InvalidSignature));
        }

        let s_hex = base58::from(&s[7..])?;
        let recv_id = secp256k1::RecoveryId::parse(s_hex[0] - 4 - 27)
            .map_err(|err| error::Error::Secp256k1(err))?;

        // TODO verify with checksum
        let data = &s_hex[1..65];
        let _checksum = &s_hex[65..];
        let sig = secp256k1::Signature::parse_slice(data)
            .map_err(|err| error::Error::Secp256k1(err))?;

        Ok(Signature {
            recv_id,
            sig
        })
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let recovery_id = self.recv_id;
        let mut sig = self.sig.clone();
        sig.normalize_s();

        // See https://github.com/EOSIO/fc/blob/f4755d330faf9d2342d646a93f9a27bf68ca759e/src/crypto/elliptic_impl_priv.cpp
        let mut checksum_data: [u8; 67] = [0u8; 67];
        let recovery_id_i32: i32 = recovery_id.into();
        checksum_data[0] = (recovery_id_i32 + 27 + 4) as u8;
        checksum_data[1..65].copy_from_slice(&sig.serialize());
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
        let sig_str = "SIG_K1_KBJgSuRYtHZcrWThugi4ygFabto756zuQQo8XeEpyRtBXLb9kbJtNW3xDcS14Rc14E8iHqLrdx46nenG5T7R4426Bspyzk";
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
