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
#[derive(Read, Write, NumBytes, Clone, Debug, PartialEq, Encode, Decode)]
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

impl Default for ProducerAuthoritySchedule {
    fn default() -> Self {
        let default_schedule = r#"
        {
            "version":0,
            "producers":[{
                "producer_name":"eosio",
                "authority":[
                    0,
                    {"threshold":1,"keys":[{"key":"EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV","weight":1}
                ]}
            ]
        }]
        }
        "#;
        let producers = serde_json::from_str(default_schedule).expect("failed to create default producers schedule");
        producers
    }
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
                let authority_v0 = BlockSigningAuthority(_tag, storage );
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
pub struct BlockSigningAuthority (
    pub UnsignedInt,
    pub BlockSigningAuthorityV0,
);

#[derive(Read, Write, NumBytes, Clone, Debug, Default, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct KeyWeight {
    pub key: PublicKey,
    pub weight: u16,
}

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
                match i.authority.1.keys.get(0) {
                    Some(weight) => return weight.key.clone(),
                    None => return Default::default(),
                }
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
        assert_eq!(Checksum256::hash(new_producers.unwrap().authority).unwrap().to_string(), "3fabc0689fc42171e87886909098dfe86415936373e848f417dd937d3bd6deef");
    }

    #[test]
    fn deserialize_block_signing_authority_should_be_ok() {
        let json = r#"{"threshold":1,"keys":[{"key":"EOS8JTznQrfvYcoFskidgKeKsmPsx3JBMpTo1jsEG2y1Ho6sQhFuL","weight":1}]}"#;
        let authority: Result<BlockSigningAuthorityV0, _> = serde_json::from_str(json);
        assert!(authority.is_ok());
        assert_eq!(Checksum256::hash(authority.unwrap()).unwrap().to_string(), "00c00dfaf1e8bcef34e5f692845315eacc234ccf3b5f4d6b4b32b4fb4f3bf5e2");
    }

    #[test]
    fn deserialize_producer_authority_schedule_should_be_ok() {
        let json = "producer_authority_schedule_v2.json";
        let new_producers_v2 = read_json_from_file(json);

        assert!(new_producers_v2.is_ok());
        let new_producers: Result<ProducerAuthoritySchedule, _> = serde_json::from_str(&new_producers_v2.unwrap());
        assert!(new_producers.is_ok());

        let new_producers = new_producers.unwrap();
        assert_eq!(Checksum256::hash(new_producers).unwrap().to_string(), "cedd80489dcc2133068079b1d1c47f0c43f084f56ea6e02c2961841c93a0d2ba");
    }

    #[test]
    fn hash_producer_authority_schedule_should_be_ok() {
        let json = r#"{"version":37,"producers":[]}"#;
        let new_producers: Result<ProducerAuthoritySchedule, _> = serde_json::from_str(json);
        assert!(new_producers.is_ok());

        let new_producers = new_producers.unwrap();
        assert_eq!(Checksum256::hash(new_producers).unwrap().to_string(), "e64c3331f87231a5c4e541f98853baf1295c17ca22b631e503aa5bdf381180d6");

        assert_eq!(
            ProducerAuthoritySchedule::default().schedule_hash().unwrap().to_string(),
            "af0197daded4f5d512c6210f58658256025d581da3d658fdfc9b11d7b8abe22e"
        );
    }

    #[test]
    fn get_public_key_by_producer_should_be_ok() {
        let json = "producer_authority_schedule_v2.json";
        let new_producers_v2 = read_json_from_file(json);

        assert!(new_producers_v2.is_ok());
        let new_producers: Result<ProducerAuthoritySchedule, _> = serde_json::from_str(&new_producers_v2.unwrap());
        assert!(new_producers.is_ok());

        let new_producers = new_producers.unwrap();

        let producer = AccountName::from_str("alohaeostest").unwrap();
        assert_eq!(new_producers.get_producer_key(producer).to_string(), "EOS8JTznQrfvYcoFskidgKeKsmPsx3JBMpTo1jsEG2y1Ho6sQhFuL");

        let producer = AccountName::from_str("eosarabianet").unwrap();
        assert_eq!(new_producers.get_producer_key(producer).to_string(), "EOS6nrJJGhoZPShQ2T4se2RqxRh5rD2LUvqBK6r5y5VVN9x1oTBwa");

        let producer = AccountName::from_str("tokenika4tst").unwrap();
        assert_eq!(new_producers.get_producer_key(producer).to_string(), "EOS6wkp1PpqQUgEA6UtgW21Zo3o1XcQeLXzcLLgKcPJhTz2aSF6fz");
    }
}
