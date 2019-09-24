use crate::Client;
use crate::eosio::AccountName;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Abi {
    pub version: String,
    pub types: Vec<Type>,
    pub structs: Vec<Struct>,
    pub actions: Vec<Action>,
    pub tables: Vec<Table>,
    pub ricardian_clauses: Vec<RicardianClause>,
    pub error_messages: Vec<ErrorMessage>,
    pub abi_extensions: Vec<AbiExtension>,
    // TODO variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Type {
    pub new_type_name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Struct {
    pub name: String,
    pub base: String,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Action {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub ricardian_contract: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub index_type: String,
    pub key_names: Vec<String>,
    pub key_types: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RicardianClause {
    pub id: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ErrorMessage {
    pub error_code: u64,
    pub error_msg: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AbiExtension {
    #[serde(rename = "type")]
    pub type_: u16,
    pub data: String,
}

#[cfg(feature = "use-hyper")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;
    use crate::eosio::n;

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
