use alloc::vec;
use alloc::string::ToString;
use bitcoin_hashes::{Hash as HashTrait, sha256};
use crate::{NumBytes, Read, Write};
use codec::{Encode, Decode};
use core::str::FromStr;
#[cfg(feature = "std")]
use serde::ser::{Serialize, Serializer};

// TODO Read, Write, NumBytes needs a custom implementation based on fixed_bytes
#[derive(Read, Write, NumBytes, Default, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Encode, Decode)]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct Checksum256(pub [u8; 32]);

#[cfg(feature = "std")]
impl Serialize for Checksum256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for Checksum256 {
    fn deserialize<D>(deserializer: D) -> Result<Self,D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct VisitorChecksum256;
        impl<'de> serde::de::Visitor<'de> for VisitorChecksum256 {
            type Value = Checksum256;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("this is error")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: serde::de::Error
            {
                Ok(Self::Value::from_str(value).map_err(|_| E::custom("error"))?)
            }
        }
        deserializer.deserialize_any(VisitorChecksum256)
    }
}

impl Checksum256 {
    pub fn new(data: [u8; 32]) -> Self {
        Self(data)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub const fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    pub fn hash0(&self) -> u64 {
        (self.0[0] as u64)
            | (self.0[1] as u64) << 8
            | (self.0[2] as u64) << 16
            | (self.0[3] as u64) << 24
            | (self.0[4] as u64) << 32
            | (self.0[5] as u64) << 40
            | (self.0[6] as u64) << 48
            | (self.0[7] as u64) << 56
    }

    pub fn hash1(&self) -> u64 {
        (self.0[8] as u64)
            | (self.0[9] as u64) << 8
            | (self.0[10] as u64) << 16
            | (self.0[11] as u64) << 24
            | (self.0[12] as u64) << 32
            | (self.0[13] as u64) << 40
            | (self.0[14] as u64) << 48
            | (self.0[15] as u64) << 56
    }

    pub fn set_hash0(&mut self, hash0: u64) {
        self.0[0] = hash0 as u8;
        self.0[1] = (hash0 >> 8) as u8;
        self.0[2] = (hash0 >> 16) as u8;
        self.0[3] = (hash0 >> 24) as u8;
    }

    pub fn hash<T: Write + NumBytes>(t: T) -> crate::Result<Checksum256> {
        let mut data = vec![0u8; t.num_bytes()];
        t.write(&mut data, &mut 0).map_err(crate::Error::BytesWriteError)?;

        Ok(Checksum256::hash_from_slice(&data))
    }

    pub fn hash_from_slice(data: &[u8]) -> Checksum256 {
        let hash_data = sha256::Hash::hash(&data);
        Checksum256(hash_data.into_inner())
    }
}

impl From<[u8; 32]> for Checksum256 {
    #[inline]
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl From<Checksum256> for [u8; 32] {
    #[inline]
    fn from(value: Checksum256) -> Self {
        value.0
    }
}

impl From<&str> for Checksum256 {
    fn from(value: &str) -> Self {
        Checksum256::from_str(value).unwrap()
    } // Todo, replace from with try_from
}

impl FromStr for Checksum256 {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Checksum256> {
        let raw = hex::decode(s).map_err(crate::error::Error::FromHexError)?;
        if raw.len() != 32 {
            return Err(crate::Error::InvalidLength);
        }
        let mut target: [u8;32] = [0u8;32];
        target.copy_from_slice(&raw[0..32]);

        Ok(Checksum256::new(target))
    }
}

impl core::fmt::Display for Checksum256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn checksum256_should_be_ok() {
        let s: Result<Checksum256,_> = Checksum256::from_str("05458d0eb126feab3e44b7298a021298d9a4fa96795adf68399b95c82d231aff");
        assert!(s.is_ok());

        let checked = Checksum256(
            [0,5,4,5,8,0,0,4,6,1,2,6,5,5,5,0,3,1,4,4,7,7,2,9,8,3,6,6,8,5,4,2]
        );
        let asb = Checksum256::as_bytes(&checked);
        let arr:[u8;32] = [0,5,4,5,8,0,0,4,6,1,2,6,5,5,5,0,3,1,4,4,7,7,2,9,8,3,6,6,8,5,4,2];
        assert_eq!(asb,&arr);

        let hash_test_data = Checksum256(
            [0,5,4,5,8,0,0,4,6,1,2,6,5,5,5,0,3,1,4,4,7,7,2,9,8,3,6,6,8,5,4,2,]
        );
        let hash_test_new_data = Checksum256(
            [0,5,4,5,8,0,0,4,6,1,2,6,5,5,5,0,3,1,0,0,0,0,0,0,0,0,0,0,8,5,4,2,]
        );
        let hash0_test_data = Checksum256::hash0(&hash_test_data);
        let hash0_test_new_data = Checksum256::hash0(&hash_test_new_data);
        assert_eq!(hash0_test_data, hash0_test_new_data);

        //Change the first eight bits of the array, the hash value changes
        let hash_test_data1 = Checksum256(
            [0,2,3,0,1,0,8,0,0,1,2,6,5,5,5,0,3,1,4,4,7,7,2,9,8,3,6,6,8,5,4,2,]
        );
        let hash_test_new_data2 = Checksum256(
            [0,5,4,5,8,0,0,4,6,1,2,6,5,5,5,0,3,1,0,0,0,0,0,0,0,0,0,0,8,5,4,2,]
        );
        let hash0_test_data1 = Checksum256::hash0(&hash_test_data1);
        let hash0_test_new_data1 = Checksum256::hash0(&hash_test_new_data2);
        assert_ne!(hash0_test_data1,hash0_test_new_data1);
    }

    #[test]
    fn checksum256_deserialization_should_be_ok() {
        let dut_str = r#"
        [
            [
            "0000004928647cc5305748cc67caa9610886278e828dfd54996c445d1c011207",
            "0000004a45d8d6f0d819c46c7c7bed25b0cd5c84b5c7195e109b884cd6318cba",
            "0000004b13d7c252648f7dab13f0fb768770661dba9680da52137b7f8701d5cf"
            ],
            [
            "00000053e086dcd817526b00f9f94a0d3fe9baeae902111306238ecfa73e8092",
            "00000054e658541a25d25c4cfff54f611b76982f398e9cc69f966dd9d0d7e9cb",
            "0000005510edcf0dc2c25e8fbaf550e137ddf80be8b39f2607de2522e4eedeba"
            ],
            []
        ]
        "#;
        let result: Result<Vec<Vec<Checksum256>>, _> = serde_json::from_str(dut_str);
        assert!(result.is_ok());
    }
}