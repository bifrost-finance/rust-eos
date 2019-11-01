use alloc::collections::BTreeMap as Map;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::{fmt, marker::PhantomData, str::FromStr};
use crate::Client;
use primitives::names::{AccountName, ActionName};
use primitives::permission_level::PermissionLevel;
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
    pub new_producers: Option<NewProducers>,
    pub header_extensions: Vec<Extension>,
    pub producer_signature: String,
    pub transactions: Vec<Transaction>,
    pub block_extensions: Vec<Extension>,
    pub id: String,
    pub block_num: u64,
    pub ref_block_prefix: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewProducers {
    pub version: u32,
    pub producers: Vec<AccountName>,
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
        if let Err(e) = response {
            // downcast failure::Error to our own error
            if let Some(crate::Error::EosError{ ref eos_err }) = e.downcast_ref::<crate::Error>() {
                assert_eq!(eos_err.code, 500);
                assert_eq!(eos_err.error.what, "Invalid block ID");
            } else {
                assert!(true);
            }
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
        if let Err(e) = response {
            // downcast failure::Error to our own error
            if let Some(crate::Error::EosError{ ref eos_err }) = e.downcast_ref::<crate::Error>() {
                assert_eq!(eos_err.code, 500);
                assert_eq!(eos_err.error.what, "Invalid block ID");
            } else {
                assert!(true);
            }
        } else {
            assert!(true);
        }
    }
}
