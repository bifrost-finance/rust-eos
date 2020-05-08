//! EOS 2.0.x add a new feature: Weighted-Threshold-Multi-Signature (WTMsig) Block Production
//! see this: https://github.com/EOSIO/eos/issues/7403

use alloc::vec::Vec;
use core::{convert::From, str::FromStr};
use crate::{AccountName, NumBytes, Read, Write, PublicKey, Checksum256, UnsignedInt};
use codec::{Encode, Decode};
use core::default::Default;
#[cfg(feature = "std")]
use serde::{de::Error, Deserialize, Serialize};

/// Defines both the order, account name, and signing keys of the active set
/// of producers.
#[derive(Read, Write, NumBytes, Clone, Debug, Default, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct ProducerAuthoritySchedule {
    /// Version number of the schedule. It is sequentially incrementing
    /// version number.
    pub version: u32,
    /// List of producers for this schedule, including its signing key
    pub producers: Vec<ProducerAuthority>,
}

#[derive(Read, Write, NumBytes, Clone, Debug, Default, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct ProducerAuthority {
    pub producer_name: AccountName,
    pub authority: BlockSigningAuthority,
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for ProducerAuthority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct VisitorProducerAuthority;
        impl<'de> serde::de::Visitor<'de> for VisitorProducerAuthority {
            type Value = ProducerAuthority;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "expect a struct or map, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
                where D: serde::de::MapAccess<'de>,
            {
                let mut producer_name = Default::default();
                let mut storage = BlockSigningAuthorityV0::default();
                let mut _tag = UnsignedInt::from(0u32);
                while let Some(field) = map.next_key()? {
                    match field {
                        "producer_name" => {
                            let val: String = map.next_value()?;
                            producer_name = AccountName::from_str(&val).map_err(|e| D::Error::custom(e))?;
                        }
                        "authority" => {
                            let val: serde_json::Value = map.next_value()?;
                            for v in val.as_array().ok_or("no object found").map_err(|e| D::Error::custom(e))? {
                                if v.is_object() {
                                    storage = serde_json::from_value(v.clone()).map_err(|e| D::Error::custom(e))?;
                                }
                                if v.is_number() {
                                    _tag = UnsignedInt::from(v.as_i64().ok_or("no number found").map_err(|e| D::Error::custom(e))? as u32);
                                }
                            }
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let authority_v0 = BlockSigningAuthority { _tag, storage };
                let authority = ProducerAuthority {
                    producer_name, authority: authority_v0
                };

                Ok(authority)
            }
        }
        deserializer.deserialize_any(VisitorProducerAuthority)
    }
}

#[derive(Read, Write, NumBytes, Clone, Debug, Default, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct BlockSigningAuthorityV0 {
    pub threshold: u32,
    pub keys: Vec<KeyWeight>,
}

#[derive(Read, Write, NumBytes, Clone, Debug, Default, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct BlockSigningAuthority {
    pub _tag: UnsignedInt,
    pub storage: BlockSigningAuthorityV0
}

#[derive(Read, Write, NumBytes, Clone, Debug, Default, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct KeyWeight {
    pub weight: u16,
    pub key: PublicKey,
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
    fn deserialize_producer_authority_should_be_ok() {
        let json = r#"{
          "producer_name": "alohaeostest",
          "authority": [
            0,
            {
              "threshold": 1,
              "keys": [
                {
                  "key": "EOS8JTznQrfvYcoFskidgKeKsmPsx3JBMpTo1jsEG2y1Ho6sQhFuL",
                  "weight": 1
                }
              ]
            }
          ]
        }
        "#;
        let new_producers: Result<ProducerAuthority, _> = serde_json::from_str(json);
        assert!(new_producers.is_ok());
        dbg!(&new_producers);
        assert_eq!(Checksum256::hash(new_producers.unwrap().authority).unwrap().to_string(), "6ab1bbed360f8bc900b074af7942d2d07210b6c7653f89c21ae1490f2e0d3733");
    }

    #[test]
    fn deserialize_block_signing_authority_should_be_ok() {
        let json = r#"
        {
          "threshold": 1,
          "keys": [
            {
              "key": "EOS8JTznQrfvYcoFskidgKeKsmPsx3JBMpTo1jsEG2y1Ho6sQhFuL",
              "weight": 1
            }
          ]
        }
        "#;
        let authority: Result<BlockSigningAuthority, _> = serde_json::from_str(json);
        assert!(authority.is_ok());
        assert_eq!(Checksum256::hash(authority.unwrap()).unwrap().to_string(), "3fabc0689fc42171e87886909098dfe86415936373e848f417dd937d3bd6deef");
    }

    #[test]
    fn deserialize_producer_authority_schedule_should_be_ok() {
        let json = "producer_authority_schedule_v2.json";
        let new_producers_v2 = read_json_from_file(json);

        assert!(new_producers_v2.is_ok());
        let new_producers: Result<ProducerAuthoritySchedule, _> = serde_json::from_str(&new_producers_v2.unwrap());
        assert!(new_producers.is_ok());

        let new_producers = new_producers.unwrap();
        dbg!(&new_producers);
        assert_eq!(Checksum256::hash(new_producers).unwrap().to_string(), "cedd80489dcc2133068079b1d1c47f0c43f084f56ea6e02c2961841c93a0d2ba");
    }

    #[test]
    fn hash_producer_authority_schedule_should_be_ok() {
        let json = r#"{"version":37,"producers":[]}"#;
        let new_producers: Result<ProducerAuthoritySchedule, _> = serde_json::from_str(json);
        assert!(new_producers.is_ok());

        let new_producers = new_producers.unwrap();
        assert_eq!(Checksum256::hash(new_producers).unwrap().to_string(), "e64c3331f87231a5c4e541f98853baf1295c17ca22b631e503aa5bdf381180d6");
    }
}

/*
impl ProducerAuthoritySchedule {
    pub fn new(version: u32, producers: Vec<ProducerAuthority>) -> Self {
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

// This is just for testing
impl Default for ProducerSchedule {
    fn default() -> Self {
        let version = 0u32;
        let producers = {
            let producer_name = AccountName::from(6138663577826885632);
            let type_ = UnsignedInt::from(0u32);
            let data = [
                2u8, 192, 222, 210, 188, 31, 19, 5, 251, 15, 170, 197, 230, 192, 62, 227,
                161, 146, 66, 52, 152, 84, 39, 182, 22, 124, 165, 105, 209, 61, 244, 53, 207
            ];
            let pk = PublicKey { type_, data };
            ProducerKey { producer_name, block_signing_key: pk}
        };
        ProducerSchedule {
            version,
            producers: alloc::vec![producers]
        }
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
    fn producers_schedule_deserialization_should_be_ok() {
        let s = r#"
        {
            "version":0,
            "producers":[
                {
                    "producer_name":"eosio",
                    "block_signing_key":"EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV"
                }
            ]
        }
        "#;
        let new_producers: Result<ProducerSchedule, _> = serde_json::from_str(&s);
        assert!(new_producers.is_ok());

        let new_producers = new_producers.unwrap();
        assert_eq!(new_producers.version, 0);
        assert_eq!(new_producers.producers[0].producer_name.to_string(), "eosio");
        assert_eq!(new_producers.producers[0].block_signing_key.to_string(), "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV");
    }

    #[test]
    fn producers_schedule_default_should_be_ok() {
        let new_producers = ProducerSchedule::default();
        assert_eq!(new_producers.version, 0);
        assert_eq!(new_producers.producers[0].producer_name.to_string(), "eosio");
        assert_eq!(new_producers.producers[0].block_signing_key.to_string(), "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV");
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
*/
