use alloc::string::String;
use alloc::vec::Vec;
use crate::Client;
use primitives::names::AccountName;
use rpc_codegen::Fetch;
use serde::{Deserialize, Serialize};

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
    use std::str::FromStr;

    #[test]
    fn get_abi_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let account_name: AccountName =AccountName::from_str("eosio.token").unwrap();
        let response = get_abi(account_name).fetch(&hyper_client);
        assert!(response.is_ok());
    }

    #[test]
    fn get_abi_by_non_exist_account() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        // eosio.token1 is an invalid account
        let account_name: AccountName = AccountName::from_str("eosio.token1").unwrap();
        let response = get_abi(account_name).fetch(&hyper_client);
        if let Err(crate::Error::EosError{ ref eos_err }) = response {
            assert_eq!(eos_err.code, 500);
            assert_eq!(eos_err.message, "Internal Service Error");
        } else {
            assert!(true);
        }
    }
}
