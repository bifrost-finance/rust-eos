use crate::Client;
use crate::chain::ReturnKind;
use eosio::{AccountName, Symbol};
use serde_derive::{Deserialize, Serialize};
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;
    use crate::ReturnKind;
    use eosio::{Symbol, s, n};

    #[test]
    fn get_currency_stats_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let code: AccountName = n!(eosio.token).into();
        let symbol: Symbol = s!(4, EOS).into();
        let response = get_currency_stats(code, symbol).fetch(&hyper_client);
        if let ReturnKind::GetCurrencyStats(data) = response.unwrap() {
            dbg!(&data);
        }
    }
}
