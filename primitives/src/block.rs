use crate::{Extension, SignedBlockHeader, Read, Write, NumBytes, SerializeData};

#[derive(Debug, Clone, Default, Read, Write, NumBytes)]
#[eosio_core_root_path = "crate"]
pub struct Block {
    pub signed_block_header: SignedBlockHeader,
    pub transactions: Vec<TransactionReceipt>,
    pub block_extensions: Vec<Extension>,
}

impl Block {
    pub fn new(signed_block_header: SignedBlockHeader) -> Self {
        Self {
            signed_block_header,
            transactions: Default::default(),
            block_extensions: Default::default(),
        }
    }
}

impl SerializeData for Block {}

#[derive(Debug, Clone, Default, Read, Write, NumBytes)]
#[eosio_core_root_path = "crate"]
pub struct TransactionReceipt {
    status: u8,
    cpu_usage_us: u32,
    net_usage_words: u32,
}
