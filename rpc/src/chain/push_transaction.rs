use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::Client;
use hex;
use rpc_codegen::Fetch;
use chain::transaction::SignedTransaction;
use chain::SerializeData;
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
        signatures: signed_trx.signatures.iter().map(|sig| sig.to_string()).collect(),
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
    // Todo, skip parsing this filed now.
    // in the first transaction, if there's a new account without any balance in eos
    // Sometimes, after a transaction, this field will be something like
    // "account_ram_deltas": [{ "account": "alice", "delta": 240 }].
    // but continue to transfer balances to this account, the field will be like
    // "account_ram_deltas": [].
    // account_ram_deltas: Vec<String>,
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
    use chain::{SerializeData, Action, Transaction};
    use keys::secret::SecretKey;
    use hex;
    use crate::{HyperClient, GetInfo, GetBlock};
    use crate::{get_info, get_block};

    #[test]
    fn test_deserialize_arr_to_tup() {
        let ar = br#"{
            "receiver": "alice",
            "act_digest": "79e0f2eed690bd92739e5e23fba4b2c61ccefc26834d83e5a9ed668182cfddda",
            "global_sequence": 3597052,
            "recv_sequence": 833,
            "auth_sequence": [
                ["alice", 2432]
            ],
            "code_sequence": 1,
            "abi_sequence": 1
        }"#;
        let r: Result<self::ActionReceipt, _> = serde_json::from_slice(ar);
        assert!(r.is_ok());
    }

    #[test]
    fn test_deserialize_action_trace() {
        let at = br#"
            {
                "action_ordinal": 2,
                "creator_action_ordinal": 1,
                "closest_unnotified_ancestor_action_ordinal": 1,
                "receipt": {
                    "receiver": "alice",
                    "act_digest": "79e0f2eed690bd92739e5e23fba4b2c61ccefc26834d83e5a9ed668182cfddda",
                    "global_sequence": 3597052,
                    "recv_sequence": 833,
                    "auth_sequence": [
                        ["alice", 2432]
                    ],
                    "code_sequence": 1,
                    "abi_sequence": 1
                },
                "receiver": "alice",
                "act": {
                    "account": "eosio.token",
                    "name": "transfer",
                    "authorization": [{
                        "actor": "alice",
                        "permission": "active"
                    }],
                    "data": {
                        "from": "alice",
                        "to": "testw",
                        "quantity": "1.0000 EOS",
                        "memo": "a memo"
                    },
                    "hex_data": "0000000000855c3400000000009eb1ca102700000000000004454f53000000000661206d656d6f"
                },
                "context_free": false,
                "elapsed": 3,
                "console": "",
                "trx_id": "9583a7a0820642fb5e5798279333642e4fc681f0d7c891fb8d7758e3068103e2",
                "block_num": 3594518,
                "block_time": "2019-10-30T07:07:41.500",
                "producer_block_id": null,
                "account_ram_deltas": [],
                "except": null,
                "error_code": null,
                "inline_traces": []
            }
        "#;
        let r: Result<self::ActionTrace, _> = serde_json::from_slice(at);
        assert!(r.is_ok());
    }

    #[test]
    fn test_deserialize_push_transaction() {
        let pt = br#"
            {
                "transaction_id": "755551847ad8b26865c7586bcf47ba5c26267ae4ec4ac72c0b2c7d993bae4240",
                "processed": {
                    "id": "755551847ad8b26865c7586bcf47ba5c26267ae4ec4ac72c0b2c7d993bae4240",
                    "block_num": 3599042,
                    "block_time": "2019-10-30T07:45:23.500",
                    "producer_block_id": null,
                    "receipt": {
                        "status": "executed",
                        "cpu_usage_us": 370,
                        "net_usage_words": 17
                    },
                    "elapsed": 370,
                    "net_usage": 136,
                    "scheduled": false,
                    "action_traces": [{
                        "action_ordinal": 1,
                        "creator_action_ordinal": 0,
                        "closest_unnotified_ancestor_action_ordinal": 0,
                        "receipt": {
                            "receiver": "eosio.token",
                            "act_digest": "773a2cc973cc0d1a226d5935f04a162eb4db0dc939e946af898edf77c8f9d811",
                            "global_sequence": 3601581,
                            "recv_sequence": 842,
                            "auth_sequence": [
                                ["alice", 2437]
                            ],
                            "code_sequence": 1,
                            "abi_sequence": 1
                        },
                        "receiver": "eosio.token",
                        "act": {
                            "account": "eosio.token",
                            "name": "transfer",
                            "authorization": [{
                                "actor": "alice",
                                "permission": "active"
                            }],
                            "data": {
                                "from": "alice",
                                "to": "testx",
                                "quantity": "1.0000 EOS",
                                "memo": "a memo"
                            },
                            "hex_data": "0000000000855c3400000000809eb1ca102700000000000004454f53000000000661206d656d6f"
                        },
                        "context_free": false,
                        "elapsed": 132,
                        "console": "",
                        "trx_id": "755551847ad8b26865c7586bcf47ba5c26267ae4ec4ac72c0b2c7d993bae4240",
                        "block_num": 3599042,
                        "block_time": "2019-10-30T07:45:23.500",
                        "producer_block_id": null,
                        "account_ram_deltas": [{
                            "account": "alice",
                            "delta": 240
                        }],
                        "except": null,
                        "error_code": null,
                        "inline_traces": [{
                            "action_ordinal": 2,
                            "creator_action_ordinal": 1,
                            "closest_unnotified_ancestor_action_ordinal": 1,
                            "receipt": {
                                "receiver": "alice",
                                "act_digest": "773a2cc973cc0d1a226d5935f04a162eb4db0dc939e946af898edf77c8f9d811",
                                "global_sequence": 3601582,
                                "recv_sequence": 835,
                                "auth_sequence": [
                                    ["alice", 2438]
                                ],
                                "code_sequence": 1,
                                "abi_sequence": 1
                            },
                            "receiver": "alice",
                            "act": {
                                "account": "eosio.token",
                                "name": "transfer",
                                "authorization": [{
                                    "actor": "alice",
                                    "permission": "active"
                                }],
                                "data": {
                                    "from": "alice",
                                    "to": "testx",
                                    "quantity": "1.0000 EOS",
                                    "memo": "a memo"
                                },
                                "hex_data": "0000000000855c3400000000809eb1ca102700000000000004454f53000000000661206d656d6f"
                            },
                            "context_free": false,
                            "elapsed": 3,
                            "console": "",
                            "trx_id": "755551847ad8b26865c7586bcf47ba5c26267ae4ec4ac72c0b2c7d993bae4240",
                            "block_num": 3599042,
                            "block_time": "2019-10-30T07:45:23.500",
                            "producer_block_id": null,
                            "account_ram_deltas": [],
                            "except": null,
                            "error_code": null,
                            "inline_traces": []
                        }, {
                            "action_ordinal": 3,
                            "creator_action_ordinal": 1,
                            "closest_unnotified_ancestor_action_ordinal": 1,
                            "receipt": {
                                "receiver": "testx",
                                "act_digest": "773a2cc973cc0d1a226d5935f04a162eb4db0dc939e946af898edf77c8f9d811",
                                "global_sequence": 3601583,
                                "recv_sequence": 1,
                                "auth_sequence": [
                                    ["alice", 2439]
                                ],
                                "code_sequence": 1,
                                "abi_sequence": 1
                            },
                            "receiver": "testx",
                            "act": {
                                "account": "eosio.token",
                                "name": "transfer",
                                "authorization": [{
                                    "actor": "alice",
                                    "permission": "active"
                                }],
                                "data": {
                                    "from": "alice",
                                    "to": "testx",
                                    "quantity": "1.0000 EOS",
                                    "memo": "a memo"
                                },
                                "hex_data": "0000000000855c3400000000809eb1ca102700000000000004454f53000000000661206d656d6f"
                            },
                            "context_free": false,
                            "elapsed": 3,
                            "console": "",
                            "trx_id": "755551847ad8b26865c7586bcf47ba5c26267ae4ec4ac72c0b2c7d993bae4240",
                            "block_num": 3599042,
                            "block_time": "2019-10-30T07:45:23.500",
                            "producer_block_id": null,
                            "account_ram_deltas": [],
                            "except": null,
                            "error_code": null,
                            "inline_traces": []
                        }]
                    }],
                    "account_ram_delta": null,
                    "except": null,
                    "error_code": null
                }
            }
        "#;
        let r: Result<self::PushTransaction, _> = serde_json::from_slice(pt);
        assert!(r.is_ok());
    }

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

        // Construct action
        let action = Action::transfer("alice", "bob", "1.0000 EOS", "a memo").ok().unwrap();
        let actions = vec![action];

        // Construct transaction
        let trx = Transaction::new(300, ref_block_num, ref_block_prefix, actions);
        let signed_trx = trx.sign_and_tx(sk, chain_id).ok().unwrap();
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

        // Construct action
        let action = Action::transfer("alice", "testb", "1.0000 EOS", "a memo").ok().unwrap();
        let actions = vec![action];

        // Construct transaction
        let trx = Transaction::new(300, ref_block_num, ref_block_prefix, actions);
        let signed = trx.generate_signature(&sk, &chain_id).ok().unwrap();
        let signed1 = trx.generate_signature(&sk1, &chain_id).ok().unwrap();
        let signeds = vec![signed, signed1];
        let	signed_trx = trx.generate_signed_transaction(signeds);
        let response = push_transaction(signed_trx).fetch(&hyper_client);
        assert!(response.is_ok());
    }
}
