use crate::Client;
use crate::eosio::{AccountName, ActionName, n};
use serde::{Deserialize, Serialize};
use rpc_codegen::Fetch;


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
) -> GetAbiJsonToBinParams<Args>
{
    let action: ActionName = match action {
        Actions::Close => n!(close).into(),
        Actions::Create => n!(create).into(),
        Actions::Transfer => n!(transfer).into(),
        Actions::Open => n!(open).into(),
        Actions::Retire => n!(retire).into(),
        Actions::Issue => n!(issue).into(),
    };
    GetAbiJsonToBinParams { code: code.into(), action, args }
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

// #[cfg(test)]
// mod test {
//     // unimplemented!();
// }
