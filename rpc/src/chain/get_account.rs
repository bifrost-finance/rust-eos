use alloc::string::String;
use alloc::vec::Vec;
use crate::Client;
use chain::names::{AccountName, PermissionName};
use chain::permission_level::PermissionLevel;
use rpc_codegen::Fetch;
use serde::{Deserialize, Serialize};


#[derive(Fetch, Debug, Clone, Serialize)]
#[api(path="v1/chain/get_account", http_method="POST", returns="GetAccount")]
pub struct GetAccountParams {
    account_name: AccountName,
}

pub const fn get_account(account_name: AccountName) -> GetAccountParams {
    GetAccountParams { account_name }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccount {
    pub account_name: AccountName,
    pub head_block_num: i64,
    pub head_block_time: String,
    pub privileged: bool,
    pub last_code_update: String,
    pub created: String,
    pub core_liquid_balance: Option<String>,
    pub ram_quota: i64,
    pub net_weight: i64,
    pub cpu_weight: i64,
    pub net_limit: Limit,
    pub cpu_limit: Limit,
    pub ram_usage: i64,
    pub permissions: Vec<Permission>,
    pub total_resources: Option<TotalResources>,
    pub self_delegated_bandwidth: Option<SelfDelegatedBandwidth>,
    pub refund_request: Option<RefundRequest>,
    pub voter_info: Option<VoterInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Limit {
    pub used: i64,
    pub available: i64,
    pub max: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    pub perm_name: PermissionName,
    pub parent: PermissionName,
    pub required_auth: RequiredAuth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequiredAuth {
    pub threshold: u32,
    pub keys: Vec<KeyWeight>,
    pub accounts: Vec<PermissionLevelWeight>,
    pub waits: Vec<WaitWeight>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionLevelWeight {
    pub permission: PermissionLevel,
    pub weight: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaitWeight {
    pub wait_sec: u32,
    pub weight: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyWeight {
    pub key: String,
    pub weight: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalResources {
    pub owner: AccountName,
    pub net_weight: String,
    pub cpu_weight: String,
    pub ram_bytes: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfDelegatedBandwidth {
    pub from: AccountName,
    pub to: AccountName,
    pub net_weight: String,
    pub cpu_weight: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefundRequest {
    pub owner: String,
    pub request_time: String,
    pub net_amount: String,
    pub cpu_amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoterInfo {
    pub owner: AccountName,
    pub proxy: AccountName,
    pub producers: Vec<AccountName>,
    pub staked: u64,
    pub last_vote_weight: String,
    pub proxied_vote_weight: String,
    pub is_proxy: u8,
}

#[cfg(feature = "use-hyper")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;
    use std::str::FromStr;

    #[test]
    fn get_account_from_str_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let account_name = AccountName::from_str("eosio").unwrap();
        let response = get_account(account_name).fetch(&hyper_client);
        assert!(response.is_ok());
    }

    #[test]
    fn get_account_from_str_invalid_account() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let account_name = AccountName::from_str("eosio1").unwrap();
        let response = get_account(account_name).fetch(&hyper_client);
        if let Err(crate::Error::EosError{ ref eos_err }) = response {
            assert_eq!(eos_err.code, 500);
            assert_eq!(eos_err.message, "Internal Service Error");
        } else {
            assert!(true);
        }
    }

    #[test]
    fn get_account_by_n_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let account_name: AccountName = AccountName::from_str("eosio").unwrap();
        let response = get_account(account_name).fetch(&hyper_client);
        assert!(response.is_ok())
    }

    #[test]
    fn get_account_by_n_invalid_account() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let account_name: AccountName = AccountName::from_str("eosio2").unwrap();
        let response = get_account(account_name).fetch(&hyper_client);
        if let Err(crate::Error::EosError{ ref eos_err }) = response {
            assert_eq!(eos_err.code, 500);
            assert_eq!(eos_err.message, "Internal Service Error");
        } else {
            assert!(true);
        }
    }
}
