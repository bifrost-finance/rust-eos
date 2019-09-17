use crate::Client;
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;
    use eosio::n;

    #[test]
    fn get_abi_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let account_name: AccountName = n!(eosio.token).into();
        let response = get_abi(account_name).fetch(&hyper_client);
        assert!(response.is_ok());
    }

    #[test]
    fn get_abi_by_non_exist_account() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        // eosio.token1 is an invalid account
        let account_name: AccountName = n!(eosio.token1).into();
        let response = get_abi(account_name).fetch(&hyper_client);
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
