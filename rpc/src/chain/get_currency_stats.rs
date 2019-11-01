use alloc::string::{String, ToString};
use crate::Client;
use primitives::names::AccountName;
use primitives::symbol::Symbol;
use rpc_codegen::Fetch;
use serde::{Deserialize, Serialize};

#[derive(Fetch, Debug, Clone, Serialize)]
#[api(path="v1/chain/get_currency_stats", http_method="POST", returns="GetCurrencyStats")]
pub struct GetCurrencyStatsParams {
    code: AccountName,
    symbol: String,
}

pub type GetCurrencyStats = alloc::collections::BTreeMap<String, CurrencyStats>;

pub fn get_currency_stats<C: Into<AccountName>, S: Into<Symbol>>(
    code: C,
    symbol: S,
) -> GetCurrencyStatsParams {
    GetCurrencyStatsParams {
        code: code.into(),
        symbol: symbol.into().code().to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyStats {
    pub supply: String,
    pub max_supply: String,
    pub issuer: AccountName,
}

#[cfg(feature = "use-hyper")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;
    use std::str::FromStr;

    #[test]
    fn get_currency_stats_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let code: AccountName = AccountName::from_str("eosio.token").unwrap();
        let symbol: Symbol = Symbol::from_str("4,EOS").unwrap();
        let response = get_currency_stats(code, symbol).fetch(&hyper_client);
        assert!(response.is_ok());
    }

    #[test]
    fn get_currency_stats_by_invalid_account() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        // eosio.tok2 is an invalid account
        let code: AccountName = AccountName::from_str("eosio.tok2").unwrap();
        let symbol: Symbol = Symbol::from_str("1,EOS").unwrap();
        let response = get_currency_stats(code, symbol).fetch(&hyper_client);

        if let Err(crate::Error::EosError{ ref eos_err }) = response {
            assert_eq!(eos_err.code, 500);
            assert_eq!(eos_err.error.what, "Account Query Exception");
            assert_eq!(eos_err.error.code, 3_060_002);
        } else {
            assert!(true);
        }
    }
}
