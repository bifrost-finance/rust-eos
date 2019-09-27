use alloc::string::String;
use crate::Client;
use primitives::names::AccountName;
use rpc_codegen::Fetch;
use serde::{Deserialize, Serialize};


#[derive(Fetch, Debug, Clone, Serialize)]
#[api(path="v1/chain/get_raw_code_and_abi", http_method="POST", returns="GetRawCodeAndAbi")]
pub struct GetRawCodeAndAbiParams {
    account_name: AccountName,
}

pub const fn get_raw_code_and_abi(account_name: AccountName) -> GetRawCodeAndAbiParams {
    GetRawCodeAndAbiParams { account_name }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRawCodeAndAbi {
    pub account_name: String,
    pub wasm: String,
    pub abi: String,
}

#[cfg(feature = "use-hyper")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;
    use std::str::FromStr;

    #[test]
    fn get_raw_code_and_abi_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let account_name: AccountName = AccountName::from_str("eosio").unwrap();
        let response = get_raw_code_and_abi(account_name).fetch(&hyper_client);
        assert!(response.is_ok())
    }

    #[test]
    fn get_raw_code_and_abi_from_invalid_account() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let account_name: AccountName = AccountName::from_str("eosio1").unwrap();
        let response = get_raw_code_and_abi(account_name).fetch(&hyper_client);
        if let Err(e) = response {
            // downcast failure::Error to our own error
            if let Some(crate::Error::EosError{ ref eos_err }) = e.downcast_ref::<crate::Error>() {
                assert_eq!(eos_err.code, 500);
                assert_eq!(eos_err.message, "Internal Service Error");
            } else {
                assert!(true);
            }
        } else {
            assert!(true);
        }
    }
}
