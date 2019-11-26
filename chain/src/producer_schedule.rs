//! <https://github.com/EOSIO/eosio.cdt/blob/796ff8bee9a0fc864f665a0a4d018e0ff18ac383/libraries/eosiolib/contracts/eosio/producer_schedule.hpp#L54-L69>
use alloc::vec::Vec;
use crate::{AccountName, NumBytes, ProducerKey, Read, Write, PublicKey, Checksum256};
use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Defines both the order, account name, and signing keys of the active set
/// of producers.
#[derive(Read, Write, NumBytes, Clone, Default, Debug, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
pub struct ProducerSchedule {
    /// Version number of the schedule. It is sequentially incrementing
    /// version number.
    pub version: u32,
    /// List of producers for this schedule, including its signing key
    pub producers: Vec<ProducerKey>,
}

impl ProducerSchedule {
    pub fn new(version: u32, producers: Vec<ProducerKey>) -> Self {
        Self {
            version,
            producers
        }
    }

    pub fn get_producer_key(&self, p: AccountName) -> PublicKey {
        for i in self.producers.iter() {
            if i.producer_name == p {
                return i.block_signing_key.clone();
            }
        }
        Default::default()
    }

    pub fn schedule_hash(&self) -> crate::Result<Checksum256> {
        Checksum256::hash(self.clone())
    }
}

#[cfg(test)]
mod test {
    use std::{
        error::Error,
        fs::File,
        io::Read,
        path::Path,
    };
    use super::*;
    use std::str::FromStr;

    fn read_json_from_file(json_name: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/test_data/")).join(json_name.as_ref());
        let mut file = File::open(path)?;
        let mut json_str = String::new();
        file.read_to_string(&mut json_str)?;
        Ok(json_str)
    }

    #[test]
    fn new_producers_should_be_ok() {
        let json = "new_producers.json";
        let new_producers_str = read_json_from_file(json);
        assert!(new_producers_str.is_ok());
        let new_producers: Result<ProducerSchedule, _> = serde_json::from_str(&new_producers_str.unwrap());
        assert!(new_producers.is_ok());

        let new_producers = new_producers.unwrap();
        assert_eq!(new_producers.version, 1u32);
        assert_eq!(new_producers.producers.len(), 20);

        let producer_1 = new_producers.producers.first();
        assert!(producer_1.is_some());
        assert_eq!(producer_1.unwrap().producer_name.to_string(), "batinthedark");
        assert_eq!(producer_1.unwrap().block_signing_key.to_string(), "EOS6dwoM8XGMQn49LokUcLiony7JDkbHrsFDvh5svLvPDkXtvM7oR");

        let producer_20 = new_producers.producers.last();
        assert!(producer_20.is_some());
        assert_eq!(producer_20.unwrap().producer_name.to_string(), "wealthyhorse");
        assert_eq!(producer_20.unwrap().block_signing_key.to_string(), "EOS5i1HrfxfHLRJqbExgRodhrZwp4dcLioNn4xZWCyhoBK6DNZgZt");

        let producer_x = new_producers.producers.get(10);
        assert!(producer_x.is_some());
        assert_eq!(producer_x.unwrap().producer_name.to_string(), "lioninjungle");
        assert_eq!(producer_x.unwrap().block_signing_key.to_string(), "EOS5BcLionmbgEtcmu7qY6XKWaE1q31qCQSsd89zXij7FDXQnKjwk");

        let producer_x = new_producers.producers.get(13);
        assert!(producer_x.is_some());
        assert_ne!(producer_x.unwrap().producer_name.to_string(), "lioninjungle");
        assert_ne!(producer_x.unwrap().block_signing_key.to_string(), "EOS5BcLionmbgEtcmu7qY6XKWaE1q31qCQSsd89zXij7FDXQnKjwk");
    }

    #[test]
    fn get_producer_key_should_work() {
        let json = "new_producers.json";
        let new_producers_str = read_json_from_file(json);
        assert!(new_producers_str.is_ok());
        let new_producers: Result<ProducerSchedule, _> = serde_json::from_str(&new_producers_str.unwrap());
        assert!(new_producers.is_ok());

        let new_producers = new_producers.unwrap();
        let pk = new_producers.get_producer_key(AccountName::from_str("wealthyhorse").unwrap());
        assert_eq!(pk, PublicKey::from_str("EOS5i1HrfxfHLRJqbExgRodhrZwp4dcLioNn4xZWCyhoBK6DNZgZt").unwrap());
        let pk = new_producers.get_producer_key(AccountName::from_str("pythoncolors").unwrap());
        assert_eq!(pk, PublicKey::from_str("EOS8R7GB5CLionUEy8FgGksGAGtc2cbcQWgty3MTAgzJvGTmtqPLz").unwrap());
        let pk = new_producers.get_producer_key(AccountName::from_str("littlerabbit").unwrap());
        assert_eq!(pk, PublicKey::from_str("EOS65orCLioNFkVT5uDF7J63bNUk97oF8T83iWfuvbSKWYUUq9EWd").unwrap());
    }

    #[test]
    fn schedule_hash_should_work() {
        let json = "new_producers.json";
        let new_producers_str = read_json_from_file(json);
        assert!(new_producers_str.is_ok());
        let new_producers: Result<ProducerSchedule, _> = serde_json::from_str(&new_producers_str.unwrap());
        assert!(new_producers.is_ok());

        let new_producers = new_producers.unwrap();
        let hash = new_producers.schedule_hash();
        assert!(hash.is_ok());
        let hash = hash.unwrap();
        assert_eq!(hash, "e2b28d9dbe1948d0f36973014bbe1c1c936cd38b7907ba29f2d3fb9061b2dd3c".into());
    }
}
