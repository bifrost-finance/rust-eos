use crate::Client;
use crate::chain::ReturnKind;
use eosio::AccountName;
use eosio::Symbol;
use serde_derive::Serialize;
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
    S: Into<Symbol>,
>(
    code: C,
    account: A,
    symbol: Option<S>,
) -> GetCurrencyBalanceParams {
    GetCurrencyBalanceParams {
        code: code.into(),
        account: account.into(),
        symbol: symbol.map(|s| s.into().code().to_string() ),
    }
}

pub type GetCurrencyBalance = Vec<String>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;
    use crate::ReturnKind;
    use eosio::{Symbol, s, n};
    use std::str::FromStr;

    #[test]
    fn get_currency_balance_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let code: AccountName = n!(eosio.token).into();
        let account_name: AccountName = n!(b1).into();
        let symbol: Symbol = s!(4, EOS).into();
        let response = get_currency_balance(code, account_name, Some(symbol)).fetch(&hyper_client);
        if let ReturnKind::GetCurrencyBalance(data) = response.unwrap() {
            dbg!(&data);
        }
    }
}
