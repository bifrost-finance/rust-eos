use crate::Client;
use crate::eosio::{AccountName, ActionName, PermissionLevel};
use serde::{Deserialize, Serialize};
use rpc_codegen::Fetch;


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
    pub trx: Trx,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trx {
    pub id: String,
    pub signatures: Vec<String>,
    pub compression: String,
    pub packed_context_free_data: String,
    pub packed_trx: String,
    pub transaction: TransactionInner,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub data: ::serde_json::Value,
    pub hex_data: String,
}

#[cfg(feature = "use-hyper")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;

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
