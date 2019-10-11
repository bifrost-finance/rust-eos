use crate::{
    Extension,
    NumBytes,
    Read,
    SerializeData,
    SignedBlockHeader,
    UnsignedInt,
    Write
};

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq)]
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

    pub fn block_num(&self) -> u32 {
        self.signed_block_header.block_num()
    }
}

impl core::fmt::Display for Block {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}\n\
            transactions: {:?}\n\
            block_extensions: {:?}",
            self.signed_block_header,
            self.transactions,
            self.block_extensions,
        )
    }
}

impl SerializeData for Block {}

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct TransactionReceipt {
    status: u8,
    cpu_usage_us: u32,
    net_usage_words: u32,
}


impl SerializeData for Option<u8> {}
impl SerializeData for Option<UnsignedInt> {}
impl SerializeData for UnsignedInt {}
impl SerializeData for Vec<UnsignedInt> {}


#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{AccountName, Block, BlockHeader, BlockTimestamp, Checksum256, NumBytes, Read, SerializeData, SignedBlockHeader, TimePointSec, UnsignedInt};

    #[test]
    fn block_test() {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let tps = TimePointSec::from_unix_seconds(since_the_epoch.as_secs() as u32);
        let block_timestamp = BlockTimestamp::from(tps);
        let producer = AccountName::from_str("eosio").unwrap();
        let block_header = BlockHeader::new(
            block_timestamp,
            producer,
            0,
            Checksum256::from([10u8; 32]),
            Checksum256::from([11u8; 32]),
            Checksum256::from([12u8; 32]),
            0,
            None,
            vec![]
        );
        let producer_signature = Default::default();
        let signed_block_header = SignedBlockHeader::new(block_header, producer_signature);

        let block = Block::new(signed_block_header);
        dbg!(&block);
        dbg!(&block.to_serialize_data());
        dbg!(&block.num_bytes());
        dbg!(hex::encode(&block.to_serialize_data()));

        let op: Option<UnsignedInt> = Some(UnsignedInt::from(10000u16));
        let op2 = UnsignedInt::from(10000u16);
        let op3: Option<UnsignedInt> = None;
        let op4: Vec<UnsignedInt> = vec![UnsignedInt::from(10000u16), UnsignedInt::from(10001u16)];
        let op5: Vec<UnsignedInt> = vec![];
        let op6 = UnsignedInt::from(0x01u16);
        dbg!(hex::encode(&op.to_serialize_data()));
        dbg!(hex::encode(&op2.to_serialize_data()));
        dbg!(hex::encode(&op3.to_serialize_data()));
        dbg!(hex::encode(&op4.to_serialize_data()));
        dbg!(hex::encode(&op5.to_serialize_data()));
        dbg!(hex::encode(&op6.to_serialize_data()));

        let data = hex::decode("dded404a0000000000ea3055000000001b41d39f263026aa8916529450c964a8724a2d71498dbcefead211a24f720000000000000000000000000000000000000000000000000000000000000000bf17e8f5e8024c2f017f7861004750287b861c08ddb74b15c848ebf3bde11afd000000000000001f6db047c02fb436bd3c6d04593b5d3254be0f72a6c747453ef66d4d4c7b7987a128705a976b8f653997849b6c17191866be8d2f384ea01cac75eb1fecf67c7e910000");
        let data = data.unwrap();
        let mut pos = 0usize;
        let bk = Block::read(&data.as_slice(), &mut pos);
        dbg!(&bk);
        dbg!(&pos);
    }
}
