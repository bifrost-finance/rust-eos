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

#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;
    use crate::ReturnKind;
    use eosio::n;

    #[test]
    fn get_abi_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let account_name: AccountName = n!(eosio.token).into();
        let response = get_abi(account_name).fetch(&hyper_client);
        if let ReturnKind::GetAbi(data) = response.unwrap() {
            dbg!(&data);
        }
    }
}
