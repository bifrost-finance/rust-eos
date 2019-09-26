use crate::{Action, NumBytes, Read, TimePointSec, UnsignedInt, Write, SerializeData};
use keys::secret::SecretKey;
use hex;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

#[derive(Read, Write, NumBytes, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Default)]
#[eosio_core_root_path = "crate"]
pub struct Extension(u16, Vec<char>);

#[derive(Read, Write, NumBytes, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Default)]
#[eosio_core_root_path = "crate"]
pub struct TransactionHeader {
    pub expiration: TimePointSec,
    pub ref_block_num: u16,
    pub ref_block_prefix: u32,
    /// number of 8 byte words this transaction can serialize into after compressions
    pub max_net_usage_words: UnsignedInt,
    /// number of CPU usage units to bill transaction for
    pub max_cpu_usage_ms: u8,
    /// number of seconds to delay transaction, default: 0
    pub delay_sec: UnsignedInt,
}

impl TransactionHeader {
    pub fn new(expiration: TimePointSec, ref_block_num: u16, ref_block_prefix: u32, ) -> Self {
        TransactionHeader {
            expiration,
            ref_block_num,
            ref_block_prefix,
            max_net_usage_words: 0u32.into(),
            max_cpu_usage_ms: 0,
            delay_sec: 0u32.into(),
        }
    }
}

#[derive(Write, NumBytes, Read, Debug, Clone, Default)]
#[eosio_core_root_path = "crate"]
pub struct Transaction {
    pub header: TransactionHeader,
    pub context_free_actions: Vec<Action>,
    pub actions: Vec<Action>,
    pub transaction_extensions: Vec<Extension>,
}

impl Transaction {
    pub fn new(header: TransactionHeader, actions: Vec<Action>, ) -> Self {
        Transaction {
            header,
            context_free_actions: vec![],
            actions,
            transaction_extensions: vec![],
        }
    }

    pub fn sign(&self, sk: SecretKey, chain_id: String) -> Result<SignedTransaction, crate::error::Error> {
        let mut sign_data: Vec<u8>  = Vec::new();
        let mut chain_id_hex = hex::decode(chain_id)
            .map_err(|err| crate::error::Error::FromHexError(err))?;
        sign_data.append(&mut chain_id_hex);
        sign_data.append(&mut self.to_serialize_data());
        sign_data.append(&mut vec![0u8; 32]);

        let sig = sk.sign(&sign_data.as_slice());

        Ok(SignedTransaction {
            signatures: vec![sig.to_string()],
            context_free_data: vec![],
            trx: self.clone(),
        })
    }
}

impl SerializeData for Transaction {}

#[derive( Debug, Clone, Default)]
pub struct SignedTransaction {
    pub signatures: Vec<String>,
    pub context_free_data: Vec<u8>,
    pub trx: Transaction,
}

#[derive(Clone, Debug)]
pub struct DeferredTransactionId(u128);

impl DeferredTransactionId {
    pub const fn as_u128(&self) -> u128 {
        self.0
    }
}

impl From<u128> for DeferredTransactionId {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::{ActionTransfer, PermissionLevel};
    use keys::secret::SecretKey;
    use std::time::{SystemTime, UNIX_EPOCH, Duration};

    #[test]
    fn sign_tx_should_work() {
        let sk = SecretKey::from_wif("5KUEhweMaSD2szyjU9EKjAyY642ZdVL2qzHW72dQcNRzUMWx9EL").unwrap();

        let start = SystemTime::now().checked_add(Duration::from_secs(600)).unwrap();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let expiration = TimePointSec::from_unix_seconds(since_the_epoch.as_secs() as u32);
        let ref_block_num = 0;
        let ref_block_prefix = 0;
        let trx_header = TransactionHeader::new(expiration, ref_block_num, ref_block_prefix);
        let permission_level = PermissionLevel::from_str(
            "testa",
            "active"
        ).ok().unwrap();
        let action_transfer = ActionTransfer::from_str(
            "testa",
            "testb",
            "1.0000 EOS",
            "a memo"
        ).ok().unwrap();
        let action = Action::from_str(
            "eosio.token",
            "transfer",
            vec![permission_level],
            action_transfer
        ).ok().unwrap();
        let actions = vec![action];

        let chain_id = "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f".to_string();
        let trx = Transaction::new(trx_header, actions);
        let signed_trx = trx.sign(sk, chain_id);
        assert!(signed_trx.is_ok());
        assert_eq!(
            hex::encode(&trx.to_serialize_data()[4..]),
            "000000000000000000000100a6823403ea3055000000572d3ccdcd01000000000093b1ca00000000a8ed323227000000000093b1ca000000008093b1ca102700000000000004454f53000000000661206d656d6f00"
        );
        dbg!(signed_trx.ok().unwrap());
    }
}
