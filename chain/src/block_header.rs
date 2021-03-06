use alloc::string::String;
use alloc::vec::Vec;
use core::{convert::From, str::FromStr};
use crate::{
    AccountName, utils::bitutil, BlockTimestamp,
    Checksum256, Extension, NumBytes, ProducerSchedule,
    Read, Signature, Write, PublicKey, TimePoint, SerializeData
};
use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{de::Error, Deserialize, Serialize, ser::{Serializer, SerializeStruct}};

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq, Encode, Decode, SerializeData)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
#[repr(C)]
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
            timestamp: {:?}\n\
            producer: {}\n\
            confirmed: {}\n\
            previous: {}\n\
            transaction_mroot: {}\n\
            action_mroot: {}\n\
            schedule_version: {}\n\
            new_producers: {:?}\n\
            header_extensions: {:?}",
            self.block_num(),
            self.id().map_err(|_| core::fmt::Error)?,
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

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq, Encode, Decode, SerializeData)]
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

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for SignedBlockHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct VisitorSignedHeader;
        impl<'de> serde::de::Visitor<'de> for VisitorSignedHeader
        {
            type Value = SignedBlockHeader;
            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
                where D: serde::de::MapAccess<'de>,
            {
                let mut timestamp = BlockTimestamp::default();
                let mut producer = AccountName::default();
                let mut confirmed = 0u16;
                let mut previous = Checksum256::default();
                let mut transaction_mroot = Checksum256::default();
                let mut action_mroot = Checksum256::default();
                let mut schedule_version = 0u32;
                let mut new_producers: Option<ProducerSchedule> = None;
                let mut producer_signature = Signature::default();
                while let Some(field) = map.next_key()? {
                    match field {
                        "timestamp" => {
                            let val: String = map.next_value()?;
                            let t = val.trim_matches(|c| c == 'z' || c == 'Z').parse::<chrono::NaiveDateTime>().map_err(|e| D::Error::custom(e))?.timestamp_nanos();
                            timestamp = BlockTimestamp::from(TimePoint::from_unix_nano_seconds(t));
                        }
                        "producer" => {
                            let val: String = map.next_value()?;
                            producer = AccountName::from_str(&val).map_err(|e| D::Error::custom(e))?;
                        }
                        "confirmed" => {
                            confirmed= map.next_value()?;
                        }
                        "previous" => {
                            let val: String = map.next_value()?;
                            previous = Checksum256::from_str(&val).map_err(|_| D::Error::custom("checksum256 deserialization error."))?;
                        }
                        "transaction_mroot" => {
                            let val: String = map.next_value()?;
                            transaction_mroot = Checksum256::from_str(&val).map_err(|_| D::Error::custom("checksum256 deserialization error."))?;
                        }
                        "action_mroot" => {
                            let val: String = map.next_value()?;
                            action_mroot = Checksum256::from_str(&val).map_err(|_| D::Error::custom("checksum256 deserialization error."))?;
                        }
                        "schedule_version" => {
                            schedule_version= map.next_value()?;
                        }
                        "new_producers" => {
                            let np: Option<ProducerSchedule> = map.next_value()?;
                            new_producers = np;
                        }
                        "producer_signature" => {
                            let val: String = map.next_value()?;
                            producer_signature = Signature::from_str(&val).map_err(|e| D::Error::custom(e))?;
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let block_header = BlockHeader {
                    timestamp,
                    producer,
                    confirmed,
                    previous,
                    transaction_mroot,
                    action_mroot,
                    schedule_version,
                    new_producers,
                    header_extensions: Default::default()
                };
                let sb = SignedBlockHeader {
                    block_header,
                    producer_signature
                };
                Ok(sb)
            }
        }
        deserializer.deserialize_any(VisitorSignedHeader)
    }
}

#[cfg(feature = "std")]
impl serde::ser::Serialize for SignedBlockHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("SignedBlockHeader", 10)?;
        state.serialize_field("timestamp", &self.block_header.timestamp)?;
        state.serialize_field("producer", &self.block_header.producer)?;
        state.serialize_field("confirmed", &self.block_header.confirmed)?;
        state.serialize_field("previous", &self.block_header.previous)?;
        state.serialize_field("transaction_mroot", &self.block_header.transaction_mroot)?;
        state.serialize_field("action_mroot", &self.block_header.action_mroot)?;
        state.serialize_field("schedule_version", &self.block_header.schedule_version)?;
        state.serialize_field("new_producers", &self.block_header.new_producers)?;
        state.serialize_field("header_extensions", &self.block_header.header_extensions)?;
        state.serialize_field("producer_signature", &self.producer_signature)?;
        state.end()
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
    use std::{
        error::Error,
        fs::File,
        io::Read,
        path::Path,
    };

    fn read_json_from_file(json_name: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/test_data/")).join(json_name.as_ref());
        let mut file = File::open(path)?;
        let mut json_str = String::new();
        file.read_to_string(&mut json_str)?;
        Ok(json_str)
    }

    #[test]
    fn verify_block_header_should_be_ok() {
        let json = "signed_block_header.json";
        let signed_block_str = read_json_from_file(json);
        let signed_block: Result<SignedBlockHeader, _> = serde_json::from_str(&signed_block_str.unwrap());
        assert!(signed_block.is_ok());
        let signed_block_header = signed_block.unwrap();

        let new_producers = signed_block_header.block_header.new_producers.as_ref().unwrap().clone();
        let schedule_hash = new_producers.schedule_hash();
        assert!(schedule_hash.is_ok());
        let schedule_hash = schedule_hash.unwrap();
        let pk = new_producers.get_producer_key(signed_block_header.block_header.producer);

        // 9312 merkle root
        let mroot: Checksum256 = "bd1dc07bd4f14bf4d9a32834ec1d35ea92eda26cc220fe91f4f65052bfb1d45a".into();
        assert!(signed_block_header.verify(mroot, schedule_hash, pk).is_ok());
    }

    #[test]
    fn deserialize_signed_block_should_be_ok() {
        let json = "transaction_with_new_producers.json";
        let signed_block_str = read_json_from_file(json);
        assert!(signed_block_str.is_ok());
        let signed_block: Result<SignedBlockHeader, _> = serde_json::from_str(&signed_block_str.unwrap());
        assert!(signed_block.is_ok());
    }

    #[test]
    fn serialize_signed_block_should_be_ok() {
        let json = r#"{
			"timestamp": "2020-10-22T17:10:00.000Z",
			"producer": "eosio",
			"confirmed": 0,
			"previous": "0000016c09878425dcfd26d3f999b6980433b8546389cae21f910df2e8cccc51",
			"transaction_mroot": "0000000000000000000000000000000000000000000000000000000000000000",
			"action_mroot": "b81b0fb9240be43c1252952ee03a37bfca84589b1fad30eea0d414f2e37530c3",
			"schedule_version": 0,
			"new_producers": null,
			"header_extensions": [],
            "producer_signature": "SIG_K1_KWxmx24YRKnYLzYc99HskdpiAQzk7bJYKW2NcgxRWyz5C9BxytFDrFsA75AWebYem6PSVTg6bt5yuLPTkpKGwL6WNbRUTg"
        }"#;
        let signed_block: Result<SignedBlockHeader, _> = serde_json::from_str(json);
        assert!(signed_block.is_ok());
        let serialized_signed_block_header = serde_json::to_string(&signed_block.unwrap());
        assert!(serialized_signed_block_header.is_ok());
    }
}
