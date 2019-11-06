use alloc::string::String;
use core::str::FromStr;
use crate::Client;
use chain::names::{AccountName, ActionName, ParseNameError};
use rpc_codegen::Fetch;
use serde::{Deserialize, Serialize};


#[derive(Fetch, Clone, Debug, Deserialize, Serialize)]
#[api(path="v1/chain/abi_json_to_bin", http_method="POST", returns="GetAbiJsonToBin")]
pub struct GetAbiJsonToBinParams<Args: serde::Serialize>
{
    code: AccountName,
    action: ActionName,
    args: Args,
}

pub fn get_abi_json_to_bin<Args: serde::Serialize>(
    code: impl Into<AccountName>,
    action: Actions,
    args: Args
) -> Result<GetAbiJsonToBinParams<Args>, ParseNameError>
{
    let action: ActionName = match action {
        Actions::Close => ActionName::from_str("close")?,
        Actions::Create => ActionName::from_str("create")?,
        Actions::Transfer => ActionName::from_str("transfer")?,
        Actions::Open => ActionName::from_str("open")?,
        Actions::Retire => ActionName::from_str("retire")?,
        Actions::Issue => ActionName::from_str("issue")?,
    };
    Ok(GetAbiJsonToBinParams { code: code.into(), action, args })
}

// defined six action
pub enum Actions {
    Close,
    Create,
    Transfer,
    Retire,
    Open,
    Issue
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferAction {
    pub from: AccountName,
    pub to: AccountName,
    pub quantity: String,
    pub memo: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloseAction {
    pub owner: AccountName,
    pub symbol: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAction {
    pub issuer: AccountName,
    pub maximum_supply: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssueAction {
    pub to: AccountName,
    pub quantity: String,
    pub memo: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpenAction {
    pub owner: AccountName,
    pub symbol: String,
    pub ram_payer: AccountName
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RetireAction {
    pub quantity: String,
    pub memo: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetAbiJsonToBin {
    pub binargs: String
}
