use crate::{
    AccountName,
    BlockTimestamp,
    Checksum256,
    Extension,
    ProducerSchedule,
    Signature,
    NumBytes,
    Read,
    Write,
};

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct BlockHeader {
    pub timestamp: BlockTimestamp,
    pub producer: AccountName,
    pub confirmed: u16,
    pub previous: Checksum256,
    pub transaction_mroot: Checksum256,
    pub action_mroot: Checksum256,
    pub schedule_version: u32,
    pub new_producers: Option<ProducerSchedule>,
    pub header_extensions: Vec<Extension>,
}

impl BlockHeader {
    pub fn new(
        timestamp: BlockTimestamp,
        producer: AccountName,
        confirmed: u16,
        previous: Checksum256,
        transaction_mroot: Checksum256,
        action_mroot: Checksum256,
        schedule_version: u32,
        new_producers: Option<ProducerSchedule>,
        header_extensions: Vec<Extension>,
    ) -> Self {
        Self {
            timestamp,
            producer,
            confirmed,
            previous,
            transaction_mroot,
            action_mroot,
            schedule_version,
            new_producers,
            header_extensions,
        }
    }
}

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct SignedBlockHeader {
    pub block_header: BlockHeader,
    pub producer_signature: Signature,
}

impl SignedBlockHeader {
    pub fn new(block_header: BlockHeader, producer_signature: Signature) -> Self {
        Self {
            block_header,
            producer_signature,
        }
    }
}


