use alloc::string::String;
use crate::Client;
use chain::names::AccountName;
use rpc_codegen::Fetch;
use serde::{Deserialize, Serialize};

#[derive(Fetch, Debug, Clone, Serialize)]
#[api(path="v1/chain/get_info", http_method="POST", returns="GetInfo")]
pub struct GetInfoParams {}

pub const fn get_info() -> GetInfoParams {
    GetInfoParams {}
}

pub type ChainId = String;

pub type BlockId = String;

pub type BlockNum = u32;

pub type ServerVersion = String;

pub type BlockTimestamp = String;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetInfo {
    pub server_version: ServerVersion,
    pub server_version_string: String,
    pub chain_id: ChainId,
    pub head_block_num: BlockNum,
    pub head_block_id: BlockId,
    pub head_block_time: BlockTimestamp,
    pub head_block_producer: AccountName,
    pub last_irreversible_block_num: BlockNum,
    pub last_irreversible_block_id: BlockId,
    pub virtual_block_cpu_limit: u32,
    pub virtual_block_net_limit: u32,
    pub block_cpu_limit: u32,
    pub block_net_limit: u32,
}

#[cfg(feature = "use-hyper")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::HyperClient;

    #[test]
    fn get_info_should_work() {
        let node: &'static str = "https://eos.greymass.com/";
        let hyper_client = HyperClient::new(node);

        let response = get_info().fetch(&hyper_client);
        assert!(response.is_ok());
    }

    #[test]
    fn get_info_from_non_exist_server() {
        // this is a non exist server
        let node: &'static str = "https://eos.greymass.com/2";
        let hyper_client = HyperClient::new(node);

        let response = get_info().fetch(&hyper_client);
        if let Err(crate::Error::EosError{ ref eos_err }) = response {
            assert_eq!(eos_err.code, 404);
            assert_eq!(eos_err.message, "Not Found");
        } else {
            assert!(true);
        }
    }
}
