use crate::{Checksum256, Read, Write, NumBytes};

#[derive(Clone, Debug, Read, Write, NumBytes, Default, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct ChainSizeMessage {
    last_irreversible_block_num: u32,
    last_irreversible_block_id: Checksum256,
    head_num: u32,
    head_id: Checksum256,
}

impl core::fmt::Display for ChainSizeMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\nlast_irreversible_block_num: {}\n\
            last_irreversible_block_id: {}\n\
            head_num: {}\n\
            head_id: {}\n",
            self.last_irreversible_block_num,
            self.last_irreversible_block_id,
            self.head_num,
            self.head_id,
        )
    }
}
