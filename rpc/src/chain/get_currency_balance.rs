use crate::Client;
use crate::chain::ReturnKind;
use eosio::{AccountName, PermissionLevel, PermissionName};
use serde_derive::{Deserialize, Serialize};
use rpc_codegen::Fetch;


#[derive(Fetch, Debug, Clone, Serialize)]
#[api(path="v1/chain/get_currency_balance", http_method="POST", returns="GetCurrencyBalance")]
pub struct GetCurrencyBalanceParams {
    code: AccountName,
    account: AccountName,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
}

pub fn get_currency_balance<
    C: Into<AccountName>,
    A: Into<AccountName>,
    S: ToString,
>(
    code: C,
    account: A,
    symbol: Option<S>,
) -> GetCurrencyBalanceParams {
    GetCurrencyBalanceParams {
        code: code.into(),
        account: account.into(),
        symbol: symbol.map(|s| s.to_string()),
    }
}

pub type GetCurrencyBalance = Vec<String>;
