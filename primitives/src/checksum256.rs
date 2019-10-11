use crate::{NumBytes, Read, Write};

// TODO Read, Write, NumBytes needs a custom implementation based on fixed_bytes
#[derive(Read, Write, NumBytes, Default, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[eosio_core_root_path = "crate"]
pub struct Checksum256([u8; 32]);

impl Checksum256 {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub const fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    pub fn hash0(&self) -> u32 {
        (self.0[0] as u32) << 24
            | (self.0[1] as u32) << 16
            | (self.0[2] as u32) << 8
            | (self.0[3] as u32)
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
