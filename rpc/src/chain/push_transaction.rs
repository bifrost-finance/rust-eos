use crate::Client;
use serde_derive::{Deserialize, Serialize};
use rpc_codegen::Fetch;
use primitives::transaction::SignedTransaction;
use primitives::SerializeData;

#[derive(Fetch, Debug, Clone, Serialize)]
#[api(path="v1/chain/push_transaction", http_method="POST", returns="PushTransaction")]

pub struct PushTransactionParams {
    signatures: Vec<String>,
    compression: bool,
    packed_context_free_data: Vec<u8>,
    packed_trx: Vec<u8>,
}

pub fn push_transaction(signed_trx: SignedTransaction) -> PushTransactionParams {
    PushTransactionParams {
        signatures: signed_trx.signatures,
        compression: false,
        packed_context_free_data: vec![],
        packed_trx: signed_trx.trx.to_serialize_data(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushTransaction {
    pub transaction_id: String,
    pub processed: Processed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Processed {
    id: String,
    block_num: String,
    block_time: String,
    producer_block_id: String,
    receipt: String,
    elapsed: String,
    net_usage: String,
    scheduled: String,
    action_traces: String,
    account_ram_delta: String,
    except: String,
    error_code: String,
}
