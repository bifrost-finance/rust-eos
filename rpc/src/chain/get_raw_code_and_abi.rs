use crate::Client;
use crate::chain::ReturnKind;
use eosio::AccountName;
use serde_derive::{Deserialize, Serialize};
use rpc_codegen::Fetch;


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

#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;
    use crate::ReturnKind;
    use eosio::n;

    #[test]
    fn get_raw_code_and_abi_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let account_name: AccountName = n!(eosio).into();
        let response = get_raw_code_and_abi(account_name).fetch(&hyper_client);
        if let ReturnKind::GetRawCodeAndAbi(data) = response.unwrap() {
            dbg!(&data);
        }
    }
}
