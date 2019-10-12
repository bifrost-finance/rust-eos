use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::Client;
use hex;
use rpc_codegen::Fetch;
use primitives::transaction::SignedTransaction;
use primitives::SerializeData;
use serde::{Deserialize, Serialize};


#[derive(Fetch, Debug, Clone, Serialize)]
#[api(path="v1/chain/push_transaction", http_method="POST", returns="PushTransaction")]
pub struct PushTransactionParams {
    signatures: Vec<String>,
    compression: String,
    packed_context_free_data: String,
    packed_trx: String,
}

pub fn push_transaction(signed_trx: SignedTransaction) -> PushTransactionParams {
    PushTransactionParams {
        signatures: signed_trx.signatures.map(|sig| hex::encode(sig)),
        compression: "none".to_string(),
        packed_context_free_data: "".to_string(),
        packed_trx: hex::encode(&signed_trx.trx.to_serialize_data()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushTransaction {
    pub transaction_id: String,
    pub processed: TransactionTrace,
}

/// https://github.com/EOSIO/eos/blob/c3817b3f965aaf3d7ac3be5809893ef17aa770f6/libraries/chain/include/eosio/chain/trace.hpp#L53
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionTrace {
    id: String,
    block_num: u32,
    block_time: String,
    producer_block_id: Option<String>,
    receipt: Option<TransactionReceiptHeader>,
    elapsed: u64,
    net_usage: u64,
    scheduled: bool,
    action_traces: Vec<ActionTrace>,
    account_ram_delta: Option<String>,
    except: Option<String>,
    error_code: Option<u64>,
}

/// https://github.com/EOSIO/eos/blob/7923af931a0e788676e546b79f88bf7c65ea47b4/libraries/chain/include/eosio/chain/block.hpp#L12
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionReceiptHeader {
    status: String,
    cpu_usage_us: u32,
    net_usage_words: u32,
}

/// https://github.com/EOSIO/eos/blob/c3817b3f965aaf3d7ac3be5809893ef17aa770f6/libraries/chain/include/eosio/chain/trace.hpp#L26
#[derive(Serialize, Deserialize, Debug)]
pub struct ActionTrace {
    action_ordinal: u32,
    creator_action_ordinal: u32,
    closest_unnotified_ancestor_action_ordinal: u32,
    receipt: Option<ActionReceipt>,
    receiver: String,
    // TODO make act work
    // act: Object,
    context_free: bool,
    elapsed: u64,
    console: String,
    trx_id: String,
    block_num: u32,
    block_time: String,
    producer_block_id: Option<String>,
    account_ram_deltas: Vec<String>,
    except: Option<String>,
    error_code: Option<u64>,
    inline_traces: Vec<ActionTrace>,
}

/// https://github.com/EOSIO/eos/blob/9b036b432c241bcb5f506ab179988ef63c08f053/libraries/chain/include/eosio/chain/action_receipt.hpp#L14
#[derive(Serialize, Deserialize, Debug)]
pub struct ActionReceipt {
    receiver: String,
    act_digest: String,
    global_sequence: u64,
    recv_sequence: u64,
    auth_sequence: Vec<(String, u64)>,
    code_sequence: u32,
    abi_sequence: u32,
}

#[cfg(feature = "use-hyper")]
#[cfg(test)]
mod tests {
    use super::*;
    use primitives::{SerializeData, TimePointSec, TransactionHeader, PermissionLevel,
        ActionTransfer, Action, Transaction};
    use keys::secret::SecretKey;
    use hex;
    use crate::{HyperClient, GetInfo, GetBlock};
    use crate::{get_info, get_block};
    use std::time::{Duration, SystemTime,UNIX_EPOCH};

    #[test]
    fn push_transaction_should_work() {
        // import private key
        let sk = SecretKey::from_wif("5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3");
        assert!(sk.is_ok());
        let sk = sk.unwrap();

        let node: &'static str = "http://47.101.139.226:8888/";
        let hyper_client = HyperClient::new(node);

        // fetch info
        let response = get_info().fetch(&hyper_client);
        let info: GetInfo = response.unwrap();
        let chain_id = info.chain_id;
        let head_block_id = info.head_block_id;

        // fetch block
        let response = get_block(head_block_id).fetch(&hyper_client);
        let block: GetBlock = response.unwrap();
        let ref_block_num = (block.block_num & 0xffff) as u16;
        let ref_block_prefix = block.ref_block_prefix as u32;

        let start = SystemTime::now().checked_add(Duration::from_secs(600)).unwrap();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        // Construct action
        let expiration = TimePointSec::from_unix_seconds(since_the_epoch.as_secs() as u32);
        let trx_header = TransactionHeader::new(expiration, ref_block_num, ref_block_prefix);
        let permission_level = PermissionLevel::from_str(
            "alice",
            "active"
        ).ok().unwrap();
        let action_transfer = ActionTransfer::from_str(
            "alice",
            "testb",
            "1.0000 EOS",
            "a memo"
        ).ok().unwrap();
        let action = Action::from_str(
            "eosio.token",
            "transfer",
            vec![permission_level],
            action_transfer
        ).ok().unwrap();
        let actions = vec![action];

        // Construct transaction
        let trx = Transaction::new(trx_header, actions);
        let signed_trx = trx.sign(sk, chain_id).ok().unwrap();
        dbg!(hex::encode(trx.to_serialize_data()));
        dbg!(signed_trx.clone());
        let response = push_transaction(signed_trx).fetch(&hyper_client);
        let res: PushTransaction = response.unwrap();
        dbg!(res);
    }

	#[test]
	fn push_transaction_with_mul_sign_should_work() {
		// import private keys
		let sk = "5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3";
		let sk1 = "5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3";

		let node: &'static str = "http://47.101.139.226:8888/";
		let hyper_client = HyperClient::new(node);

		// fetch info
		let response = get_info().fetch(&hyper_client);
		let info: GetInfo = response.unwrap();
		let chain_id = info.chain_id;
		let head_block_id = info.head_block_id;

		// fetch block
		let response = get_block(head_block_id).fetch(&hyper_client);
		let block: GetBlock = response.unwrap();
		let ref_block_num = (block.block_num & 0xffff) as u16;
		let ref_block_prefix = block.ref_block_prefix as u32;

		let start = SystemTime::now().checked_add(Duration::from_secs(600)).unwrap();
		let since_the_epoch = start
			.duration_since(UNIX_EPOCH)
			.expect("Time went backwards");

		// Construct action
		let expiration = TimePointSec::from_unix_seconds(since_the_epoch.as_secs() as u32);
		let trx_header = TransactionHeader::new(expiration, ref_block_num, ref_block_prefix);
		let permission_level = PermissionLevel::from_str(
			"alice",
			"active"
		).ok().unwrap();
		let action_transfer = ActionTransfer::from_str(
			"alice",
			"testb",
			"1.0000 EOS",
			"a memo"
		).ok().unwrap();
		let action = Action::from_str(
			"eosio.token",
			"transfer",
			vec![permission_level],
			action_transfer
		).ok().unwrap();
		let actions = vec![action];

		// Construct transaction
		let trx = Transaction::new(trx_header, actions);
		let signed = trx.generate_signature(&sk, &chain_id).ok().unwrap();
		let signed1 = trx.generate_signature(&sk1, &chain_id).ok().unwrap();
		let signeds = vec![signed, signed1];
		let	signed_trx = trx.generate_signed_transaction(signeds);
		let response = push_transaction(signed_trx).fetch(&hyper_client);
		assert!(response.is_ok());
	}
}
