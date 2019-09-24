use crate::Client;
use crate::eosio::{AccountName, Symbol};
use serde::{Deserialize, Serialize};
use rpc_codegen::Fetch;


#[derive(Fetch, Debug, Clone, Serialize)]
#[api(path="v1/chain/get_currency_stats", http_method="POST", returns="GetCurrencyStats")]
pub struct GetCurrencyStatsParams {
    code: AccountName,
    symbol: String,
}

pub type GetCurrencyStats = ::std::collections::HashMap<String, CurrencyStats>;

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
    use crate::eosio::{Symbol, s, n};

    #[test]
    fn get_currency_stats_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let code: AccountName = n!(eosio.token).into();
        let symbol: Symbol = s!(4, EOS).into();
        let response = get_currency_stats(code, symbol).fetch(&hyper_client);
        assert!(response.is_ok());
    }

    #[test]
    fn get_currency_stats_by_invalid_account() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        // eosio.tok2 is an invalid account
        let code: AccountName = n!(eosio.tok2).into();
        let symbol: Symbol = s!(1, EOS).into();
        let response = get_currency_stats(code, symbol).fetch(&hyper_client);

        if let Err(e) = response {
            // downcast failure::Error to our own error
            if let Some(crate::Error::EosError{ ref eos_err }) = e.downcast_ref::<crate::Error>() {
                assert_eq!(eos_err.code, 500);
                assert_eq!(eos_err.error.what, "Account Query Exception");
                assert_eq!(eos_err.error.code, 3_060_002);
            } else {
                assert!(true);
            }
        } else {
            assert!(true);
        }
    }
}
