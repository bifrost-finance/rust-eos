use crate::Client;
use crate::chain::ReturnKind;
use eosio::AccountName;
use eosio_abi::Abi;
use serde_derive::{Deserialize, Serialize};
use rpc_codegen::Fetch;

#[derive(Fetch, Debug, Clone, Serialize)]
#[api(path="v1/chain/get_abi", http_method="POST", returns="GetAbi")]
pub struct GetAbiParams {
    account_name: AccountName,
}

pub const fn get_abi(account_name: AccountName) -> GetAbiParams {
    GetAbiParams { account_name }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAbi {
    pub account_name: AccountName,
    pub abi: Abi,
}
