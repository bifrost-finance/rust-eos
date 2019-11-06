use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::{fmt, marker::PhantomData, str::FromStr};
use crate::Client;
use chain::names::{AccountName, ActionName};
use chain::permission_level::PermissionLevel;
use chain::producer_schedule::ProducerSchedule;
use rpc_codegen::Fetch;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{self, Visitor, MapAccess};

#[derive(Fetch, Debug, Clone, Serialize)]
#[api(path="v1/chain/get_block", http_method="POST", returns="GetBlock")]
pub struct GetBlockParams {
    block_num_or_id: String,
}

pub fn get_block<B: ToString>(block_num_or_id: B) -> GetBlockParams {
    GetBlockParams {
        block_num_or_id: block_num_or_id.to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBlock {
    pub timestamp: String,
    pub producer: AccountName,
    pub confirmed: u16,
    pub previous: String,
    pub transaction_mroot: String,
    pub action_mroot: String,
    pub schedule_version: u16,
    pub new_producers: Option<ProducerSchedule>,
    pub header_extensions: Vec<Extension>,
    pub producer_signature: String,
    pub transactions: Vec<Transaction>,
    pub block_extensions: Vec<Extension>,
    pub id: String,
    pub block_num: u64,
    pub ref_block_prefix: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Extension {
    #[serde(rename = "type")]
    pub type_: u16,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub status: String,
    pub cpu_usage_us: u64,
    pub net_usage_words: u64,
    #[serde(deserialize_with = "string_or_struct")]
    // sometimes, trx like 34e9b611b4fe7e6d82c30735f758d2def71e4c4af94e1fa691dc113179265338,
    // sometimes like a json object, need to customize deserialize the field trx.
    pub trx: Trx,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Trx {
    pub id: String,
    pub signatures: Vec<String>,
    pub compression: String,
    pub packed_context_free_data: String,
    pub packed_trx: String,
    pub transaction: TransactionInner,
}

impl FromStr for Trx {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Trx {
            id: s.to_string(),
            ..Default::default()
        })
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de> + FromStr<Err = ()>,
        D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
        where
            T: Deserialize<'de> + FromStr<Err = ()>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
            where
                E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
            where
                M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TransactionInner {
    pub expiration: String,
    pub ref_block_num: u64,
    pub ref_block_prefix: u64,
    pub max_net_usage_words: u64,
    pub max_cpu_usage_ms: u64,
    pub delay_sec: u64,
    pub context_free_actions: Vec<Action>,
    pub actions: Vec<Action>,
    pub transaction_extensions: Vec<Extension>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub account: AccountName,
    pub name: ActionName,
    pub authorization: Vec<PermissionLevel>,
    // actually, data is like this
    // "data": {
    //      "from": "eeoosssanguo",
    //      "opid": 2413859,
    //      "op": "mountadopt",
    //      "sig": "SIG_K1_KfBESRhJv7inodgdbAcYNf7ARmCgEDwCVHxZX8ci7pAdtfRdRDcXztgSMzUhF6KUhNeTvnv1jbQoUfdM7kgFC12sAxwaj9"
    //  },
    // need a customized deserialization
    // Todo, need to deserialize data as u8 array
    // pub data: Vec<u8>,
    pub hex_data: String,
}

#[cfg(feature = "use-hyper")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;

    #[test]
    fn test_block_deserialization() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        // fetch a block that has lots of transactions.
        // ensure there's no problem on deserialization on GetBlock
        let mut block_id = "85638240";
        let response = get_block(block_id).fetch(&hyper_client);
        assert!(response.is_ok());

        // fetch a block with no transaction.
        block_id = "1";
        let response = get_block(block_id).fetch(&hyper_client);
        assert!(response.is_ok());
    }

    #[test]
    fn get_block_by_id_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let block_id = "00000001405147477ab2f5f51cda427b638191c66d2c59aa392d5c2c98076cb0";
        let response = get_block(block_id).fetch(&hyper_client);
        assert!(response.is_ok())
    }

    #[test]
    fn get_block_by_num_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let block_num = 1;
        let response = get_block(block_num).fetch(&hyper_client);
        assert!(response.is_ok())
    }

    #[test]
    fn get_block_by_invalid_num() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let block_num = -1;
        let response = get_block(block_num).fetch(&hyper_client);
        if let Err(crate::Error::EosError{ ref eos_err }) = response {
            assert_eq!(eos_err.code, 500);
            assert_eq!(eos_err.error.what, "Invalid block ID");
        } else {
            assert!(true);
        }
    }

    #[test]
    fn get_block_by_invalid_id() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        // an invalid block id
        let block_id = "04bf8ea548296524ee3913b21763f6b2207476598efb627292c8843b971e6121";
        let response = get_block(block_id).fetch(&hyper_client);
        if let Err(crate::Error::EosError{ ref eos_err }) = response {
            assert_eq!(eos_err.code, 500);
            assert_eq!(eos_err.error.what, "Invalid block ID");
        } else {
            assert!(true);
        }
    }

    #[test]
    fn get_new_producers_from_block() {
        let block = br#"{
        "timestamp": "2018-11-23T17:21:02.000",
        "producer": "eosio",
        "confirmed": 0,
        "previous": "00001a38e7e07793dd42bbe4ab050d5f36df3c7d7ad7126e2de7554c36072145",
        "transaction_mroot": "0000000000000000000000000000000000000000000000000000000000000000",
        "action_mroot": "611f53d8861ff2ba3c8b143100bb3fe99c06810db6d0189639f52a99e70e01b4",
        "schedule_version": 0,
        "new_producers": {
        "version": 1,
        "producers": [{
            "producer_name": "batinthedark",
            "block_signing_key": "EOS6dwoM8XGMQn49LokUcLiony7JDkbHrsFDvh5svLvPDkXtvM7oR"
            },{
            "producer_name": "bighornsheep",
            "block_signing_key": "EOS5xfwWr4UumKm4PqUGnyCrFWYo6j5cLioNGg5yf4GgcTp2WcYxf"
            },{
            "producer_name": "bigpolarbear",
            "block_signing_key": "EOS6oZi9WjXUcLionUtSiKRa4iwCW5cT6oTzoWZdENXq1p2pq53Nv"
            },{
            "producer_name": "clevermonkey",
            "block_signing_key": "EOS5mp5wmRyL5RH2JUeEh3eoZxkJ2ZZJ9PVd1BcLioNuq4PRCZYxQ"
            },{
            "producer_name": "funnyhamster",
            "block_signing_key": "EOS7A9BoRetjpKtE3sqA6HRykRJ955MjQ5XdRmCLionVte2uERL8h"
            },{
            "producer_name": "gorillapower",
            "block_signing_key": "EOS8X5NCx1Xqa1xgQgBa9s6EK7M1SjGaDreAcLion4kDVLsjhQr9n"
            },{
            "producer_name": "hippopotamus",
            "block_signing_key": "EOS7qDcxm8YtAZUA3t9kxNGuzpCLioNnzpTRigi5Dwsfnszckobwc"
            },{
            "producer_name": "hungryolddog",
            "block_signing_key": "EOS6tw3AqqVUsCbchYRmxkPLqGct3vC63cEzKgVzLFcLionoY8YLQ"
            },{
            "producer_name": "iliketurtles",
            "block_signing_key": "EOS6itYvNZwhqS7cLion3xp3rLJNJAvKKegxeS7guvbBxG1XX5uwz"
            },{
            "producer_name": "jumpingfrogs",
            "block_signing_key": "EOS7oVWG413cLioNG7RU5Kv7NrPZovAdRSP6GZEG4LFUDWkgwNXHW"
            },{
            "producer_name": "lioninjungle",
            "block_signing_key": "EOS5BcLionmbgEtcmu7qY6XKWaE1q31qCQSsd89zXij7FDXQnKjwk"
            },{
            "producer_name": "littlerabbit",
            "block_signing_key": "EOS65orCLioNFkVT5uDF7J63bNUk97oF8T83iWfuvbSKWYUUq9EWd"
            },{
            "producer_name": "proudrooster",
            "block_signing_key": "EOS5qBd3T6nmLRsuACLion346Ue8UkCwvsoS5f3EDC1jwbrEiBDMX"
            },{
            "producer_name": "pythoncolors",
            "block_signing_key": "EOS8R7GB5CLionUEy8FgGksGAGtc2cbcQWgty3MTAgzJvGTmtqPLz"
            },{
            "producer_name": "soaringeagle",
            "block_signing_key": "EOS6iuBqJKqSK82QYCGuM96gduQpQG8xJsPDU1CLionPMGn2bT4Yn"
            },{
            "producer_name": "spideronaweb",
            "block_signing_key": "EOS6M4CYEDt3JDKS6nsxMnUcdCLioNcbyEzeAwZsQmDcoJCgaNHT8"
            },{
            "producer_name": "ssssssssnake",
            "block_signing_key": "EOS8SDhZ5CLioNLie9mb7kDu1gHfDXLwTvYBSxR1ccYSJERvutLqG"
            },{
            "producer_name": "thebluewhale",
            "block_signing_key": "EOS6Wfo1wwTPzzBVT8fe3jpz8vxCnf77YscLionBnw39iGzFWokZm"
            },{
            "producer_name": "thesilentowl",
            "block_signing_key": "EOS7y4hU89NJ658H1KmAdZ6A585bEVmSV8xBGJ3SbQM4Pt3pcLion"
            },{
            "producer_name": "wealthyhorse",
            "block_signing_key": "EOS5i1HrfxfHLRJqbExgRodhrZwp4dcLioNn4xZWCyhoBK6DNZgZt"
            }
        ]},
        "header_extensions": [],
        "producer_signature": "SIG_K1_KgdybmKf6gTj8TAX6Cu1yQuRK8P15pEJWa7Xp1cFeCE84NXNpGd6UPkwPJjYGKVstgH7JSf5xCoV1WjKxReRmtVB7vvysp",
        "transactions": [],
        "block_extensions": [],
        "id": "00001a398a9e6015296bb50045d861c656c42a5888476ede73b8350ab564f89d",
        "block_num": 6713,
        "ref_block_prefix": 11889449
        }"#;

        let result: Result<GetBlock, _> = serde_json::from_slice(block);
        assert!(result.is_ok());
        let block = result.unwrap();
        assert!(block.new_producers.is_some());
        let new_producers = block.new_producers.unwrap();
        assert_eq!(new_producers.version, 1u32);
        assert_eq!(new_producers.producers.len(), 20usize);
        let first_producer = Some(chain::ProducerKey {
            producer_name: AccountName::from_str("batinthedark").unwrap(),
            block_signing_key: chain::PublicKey::from_str("EOS6dwoM8XGMQn49LokUcLiony7JDkbHrsFDvh5svLvPDkXtvM7oR").unwrap()
        });
        assert_eq!(new_producers.producers.first(), first_producer.as_ref());
        assert_ne!(new_producers.producers.last(), first_producer.as_ref());

        let last_producer = Some(chain::ProducerKey {
            producer_name: AccountName::from_str("wealthyhorse").unwrap(),
            block_signing_key: chain::PublicKey::from_str("EOS5i1HrfxfHLRJqbExgRodhrZwp4dcLioNn4xZWCyhoBK6DNZgZt").unwrap()
        });
        assert_eq!(new_producers.producers.last(), last_producer.as_ref());

        let producer = new_producers.producers.get(10).unwrap();
        assert_eq!(&producer.block_signing_key.to_string(), "EOS5BcLionmbgEtcmu7qY6XKWaE1q31qCQSsd89zXij7FDXQnKjwk");
    }
}
