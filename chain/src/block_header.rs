use alloc::vec::Vec;
use crate::{AccountName, bitutil, BlockTimestamp, Checksum256, Extension, NumBytes, ProducerSchedule, Read, Signature, Write, PublicKey};
use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
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

impl core::fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "block_num: {}\n\
            id: {}\n\
            timestamp: {}\n\
            producer: {}\n\
            confirmed: {}\n\
            previous: {}\n\
            transaction_mroot: {}\n\
            action_mroot: {}\n\
            schedule_version: {}\n\
            new_producers: {:?}\n\
            header_extensions: {:?}",
            self.block_num(),
            self.id().unwrap(),
            self.timestamp,
            self.producer,
            self.confirmed,
            self.previous,
            self.transaction_mroot,
            self.action_mroot,
            self.schedule_version,
            self.new_producers,
            self.header_extensions,
        )
    }
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

    pub fn digest(&self) -> crate::Result<Checksum256> {
        Checksum256::hash(self.clone())
    }

    pub fn id(&self) -> crate::Result<Checksum256> {
        let mut result = self.digest()?;
        let mut hash0 = result.hash0();
        hash0 &= 0xffffffff00000000;
        hash0 += bitutil::endian_reverse_u32(self.block_num()) as u64;
        result.set_hash0(hash0);
        Ok(result)
    }

    pub fn block_num(&self) -> u32 {
        Self::num_from_id(self.previous) + 1
    }

    pub fn num_from_id(id: Checksum256) -> u32 {
        bitutil::endian_reverse_u32(id.hash0() as u32)
    }
}

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
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

    pub fn id(&self) -> crate::Result<Checksum256> {
        self.block_header.id()
    }

    pub fn block_num(&self) -> u32 {
        self.block_header.block_num()
    }

    // Todo, add test cases on this function
    #[cfg(feature = "std")]
    pub fn verify(&self, blockroot_merkle: Checksum256, schedule_hash: Checksum256, pk: PublicKey) -> crate::Result<()> {
        let digest = self.sig_digest(blockroot_merkle, schedule_hash)?;
        pk.verify(digest.as_bytes(), &self.producer_signature)
    }

    fn sig_digest(&self, blockroot_merkle: Checksum256, schedule_hash: Checksum256) -> crate::Result<Checksum256> {
        let block_header_hash = self.block_header.digest()?;
        let header_bmroot = Checksum256::hash((block_header_hash, blockroot_merkle))?;
        Checksum256::hash((header_bmroot, schedule_hash))
    }
}

impl core::fmt::Display for SignedBlockHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}\n\
            producer_signature: {:?}",
            self.block_header,
            self.producer_signature,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::signature::Signature;
    use core::str::FromStr;
    use std::{
        error::Error,
        fs::File,
        io::Read,
        path::Path,
    };
    use super::*;

    fn read_json_from_file(json_name: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/test_data/")).join(json_name.as_ref());
        let mut file = File::open(path)?;
        let mut json_str = String::new();
        file.read_to_string(&mut json_str)?;
        Ok(json_str)
    }

    #[test]
    fn verify_block_header_should_be_ok() {
//        let block_header: Result<SignedBlockHeader, _> = serde_json::from_str(&json);
        let json = "new_producers.json";
        let new_producers_str = read_json_from_file(json);
        assert!(new_producers_str.is_ok());
        let new_producers: Result<ProducerSchedule, _> = serde_json::from_str(&new_producers_str.unwrap());
        assert!(new_producers.is_ok());

        let new_producers = new_producers.unwrap();

        let header = BlockHeader {
            timestamp: BlockTimestamp(1542993662),
            producer: AccountName::from_str("eosio").unwrap(),
            confirmed: 0,
            previous: Checksum256::from("00001a38e7e07793dd42bbe4ab050d5f36df3c7d7ad7126e2de7554c36072145"),
            transaction_mroot: Checksum256::from("0000000000000000000000000000000000000000000000000000000000000000"),
            action_mroot: Checksum256::from("611f53d8861ff2ba3c8b143100bb3fe99c06810db6d0189639f52a99e70e01b4"),
            schedule_version: 0,
            new_producers: Some(new_producers),
            header_extensions: Vec::default(),
        };
        let block_header = SignedBlockHeader {
            block_header: header,
            producer_signature: Signature::from_str("SIG_K1_KgdybmKf6gTj8TAX6Cu1yQuRK8P15pEJWa7Xp1cFeCE84NXNpGd6UPkwPJjYGKVstgH7JSf5xCoV1WjKxReRmtVB7vvysp").unwrap(),
        };

        let pub_key = PublicKey::from_str("EOS5mk56pVBTZUb4Amxte9DbcbZuAHX1GWj4voUatgH7gyz8iUN1o").unwrap();

    }
}