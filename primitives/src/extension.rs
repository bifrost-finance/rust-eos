use crate::{Read, Write, NumBytes};

#[derive(Read, Write, NumBytes, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Default)]
#[eosio_core_root_path = "crate"]
pub struct Extension(u16, Vec<u8>);

impl core::fmt::Display for Extension {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}, {}", self.0, hex::encode(&self.1))
    }
}
