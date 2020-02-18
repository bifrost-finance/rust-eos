use core::fmt;
use core::str::FromStr;
use crate::{base58, hash, error};
use byteorder::{ByteOrder, LittleEndian};

/// An secp256k1 signature.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Signature {
    pub recv_id: secp256k1::RecoveryId,
    pub sig: secp256k1::Signature,
}

impl Signature {
    pub fn is_canonical(&self) -> bool {
        self.sig.is_canonical()
    }

    pub fn serialize_compact(&self) -> [u8; 65] {
        let mut data: [u8; 65] = [0u8; 65];
        data[0] = self.recv_id.serialize() + 27 + 4;
        data[1..65].copy_from_slice(&self.sig.serialize());
        data
    }

    pub fn from_compact(data: &[u8; 65]) -> crate::Result<Self> {
        let recv_id = if data[0] >= 31 {
            data[0] - 4
        } else {
            data[0]
        };
        let recv_id = secp256k1::RecoveryId::parse_rpc(recv_id)?;
        let sig = secp256k1::Signature::parse_slice(&data[1..])?;
        Ok(Self {
            recv_id,
            sig,
        })
    }
}

impl FromStr for Signature {
    type Err = error::Error;

    fn from_str(s: &str) -> crate::Result<Signature> {
        if !s.starts_with("SIG_K1_") {
            return Err(secp256k1::Error::InvalidSignature.into());
        }

        let s_hex = base58::from(&s[7..])?;
        // recovery id length: 1
        // signature length: 64
        // checksum length: 4
        if s_hex.len() != 1 + 64 + 4 {
            return Err(secp256k1::Error::InvalidSignature.into());
        }
        let mut recv_id = s_hex[0];
        if recv_id >= 4 + 27 {
            recv_id = recv_id - 4 - 27
        }
        let recv_id = secp256k1::RecoveryId::parse(recv_id)
            .map_err(|err| error::Error::Secp256k1(err))?;
        let data = &s_hex[1..65];

        // Verify checksum
        let mut checksum_data = [0u8; 67];
        checksum_data[..65].copy_from_slice(&s_hex[..65]);
        checksum_data[65..67].copy_from_slice(b"K1");
        let expected = LittleEndian::read_u32(&hash::ripemd160(&checksum_data)[..4]);
        let actual = LittleEndian::read_u32(&s_hex[65..69]);
        if expected != actual {
            return Err(base58::Error::BadChecksum(expected, actual).into());
        }

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
        let recv_id = self.recv_id;
        let sig = self.sig.clone();

        // See https://github.com/EOSIO/fc/blob/f4755d330faf9d2342d646a93f9a27bf68ca759e/src/crypto/elliptic_impl_priv.cpp
        let mut checksum_data: [u8; 67] = [0u8; 67];
        let recovery_id_i32: u8 = recv_id.into();
        checksum_data[0] = recovery_id_i32 + 4 + 27;
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
    use core::str::FromStr;
    use alloc::string::ToString;

    #[test]
    fn sig_from_str_should_work() {
        let sig_str = "SIG_K1_KBJgSuRYtHZcrWThugi4ygFabto756zuQQo8XeEpyRtBXLb9kbJtNW3xDcS14Rc14E8iHqLrdx46nenG5T7R4426Bspyzk";
        let sig = Signature::from_str(sig_str);
        assert!(sig.is_ok());
        let sig = sig.unwrap();
        assert!(sig.is_canonical());
        assert_eq!(sig.to_string(), sig_str);
    }

    #[test]
    fn sig_from_str_should_error() {
        let sig_str = "KomV6FEHKdtZxGDwhwSubEAcJ7VhtUQpEt5P6iDz33ic936aSXx87B2L56C8JLQkqNpp1W8ZXjrKiLHUEB4LCGeXvbtVuR";
        let sig = Signature::from_str(sig_str);
        assert!(sig.is_err());
    }
}
