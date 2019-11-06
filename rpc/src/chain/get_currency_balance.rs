use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::Client;
use chain::names::AccountName;
use chain::symbol::Symbol;
use rpc_codegen::Fetch;
use serde::Serialize;


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

#[cfg(feature = "use-hyper")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;
    use std::str::FromStr;

    #[test]
    fn get_currency_balance_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let code: AccountName = AccountName::from_str("eosio.token").unwrap();
        let account_name: AccountName = AccountName::from_str("b1").unwrap();
        let symbol: Symbol = Symbol::from_str("4,EOS").unwrap();
        let response = get_currency_balance(code, account_name, Some(symbol)).fetch(&hyper_client);
        assert!(response.is_ok());
    }

    #[test]
    fn get_currency_balance_from_invalid_account() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        // the balance should be empty if get it from an invalid account
        let code: AccountName = AccountName::from_str("eosio.token").unwrap();
        // an invalid account
        let account_name: AccountName = AccountName::from_str("kkkkk").unwrap();
        let symbol: Symbol = Symbol::from_str("4,EOS").unwrap();

        let response = get_currency_balance(code, account_name, Some(symbol)).fetch(&hyper_client);
        assert!(response.is_ok());

        let lhs = response.unwrap();
        assert_eq!(lhs.is_empty(), true);
    }
}
