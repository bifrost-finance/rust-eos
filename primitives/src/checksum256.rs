use bitcoin_hashes::{Hash as HashTrait, sha256};
use crate::{NumBytes, Read, Write};
#[cfg(feature = "std")]
use serde::{Deserialize, ser::{Serialize, Serializer}};

// TODO Read, Write, NumBytes needs a custom implementation based on fixed_bytes
#[derive(Read, Write, NumBytes, Default, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[eosio_core_root_path = "crate"]
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
        t.write(&mut data, &mut 0).map_err(crate::error::Error::BytesWriteError)?;

        let hash_data = sha256::Hash::hash(&data);
        Ok(Checksum256(hash_data.into_inner()))
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

impl core::fmt::Display for Checksum256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}
