use crate::{
    Action,
    ActionName,
    AccountName,
    ActionTransfer,
    Checksum256,
    Extension,
    NumBytes,
    PackedTransaction,
    Read,
    SerializeData,
    SignedBlockHeader,
    Transaction,
    UnsignedInt,
    Write
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
pub struct SignedBlock {
    pub signed_block_header: SignedBlockHeader,
    pub transactions: Vec<TransactionReceipt>,
    pub block_extensions: Vec<Extension>,
}

impl SignedBlock {
    pub fn new(signed_block_header: SignedBlockHeader) -> Self {
        Self {
            signed_block_header,
            transactions: Default::default(),
            block_extensions: Default::default(),
        }
    }

    pub fn id(&self) -> Checksum256 {
        self.signed_block_header.id()
    }

    pub fn block_num(&self) -> u32 {
        self.signed_block_header.block_num()
    }
}

impl core::fmt::Display for SignedBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}\n\
            transactions: {}\n\
            block_extensions: {}",
            self.signed_block_header,
            self.transactions.iter().map(|item| format!("{}", item)).collect::<String>(),
            self.block_extensions.iter().map(|item| format!("{}", item)).collect::<String>(),
        )
    }
}

impl SerializeData for SignedBlock {}

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
pub struct TransactionReceipt {
    pub trx_receipt_header: TransactionReceiptHeader,
    pub trx: PackedTransaction,
}

impl core::fmt::Display for TransactionReceipt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}\n{}",
            self.trx_receipt_header,
            self.trx,
        )
    }
}

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
pub struct TransactionReceiptHeader {
    status: u8,
    cpu_usage_us: u32,
    // TODO net_usage_words maybe use UnsignedInt instead
    net_usage_words: u16,
}

impl core::fmt::Display for TransactionReceiptHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "status: {}\n\
            cpu_usage_us: {}\n\
            net_usage_words: {}",
            self.status,
            self.cpu_usage_us,
            self.net_usage_words,
        )
    }
}

impl SerializeData for Option<u8> {}
impl SerializeData for Option<UnsignedInt> {}
impl SerializeData for UnsignedInt {}
impl SerializeData for Vec<UnsignedInt> {}

#[derive(Clone, Debug)]
pub struct ActionFilter {
    pub account: AccountName,
    pub name: ActionName,
}

// filter transactions by account and name
impl PartialEq<Action> for ActionFilter {
    fn eq(&self, rhs: &Action) -> bool {
        self.account.eq(&rhs.account) && self.name.eq(&rhs.name)
    }
}

#[derive(Clone, Debug)]
pub enum ActionType {
    Deposit(ActionTransfer),
    Withdraw(ActionTransfer),
}

impl ActionFilter {
    pub fn from_str<T: AsRef<str>>(account: T, name: T) -> crate::Result<Self> {
        Ok(
            Self {
                account: AccountName::from_str(account.as_ref())?,
                name: ActionName::from_str(name.as_ref())?,
            }
        )
    }

    pub fn filter(&self, blocks: &SignedBlock, banker: &AccountName)
        -> crate::Result<(Vec<ActionType>, Vec<ActionType>)>
    {
        let mut deposits: Vec<ActionType> = vec![];
        let mut withdraws: Vec<ActionType> = vec![];
        if !blocks.transactions.is_empty() {
            for trx_receipt in &blocks.transactions {
                let packet_trx = trx_receipt.trx.clone();
                let trx = Transaction::from(packet_trx);
                for ac in &trx.actions {
                    if *self == *ac {
                        let action_transfer = ActionTransfer::read(&ac.data, &mut 0)
                            .map_err(crate::Error::BytesReadError)?;
                        if action_transfer.from.eq(banker) {
                            deposits.push(ActionType::Withdraw(action_transfer));
                        } else if action_transfer.to.eq(banker) {
                            withdraws.push(ActionType::Deposit(action_transfer));
                        }
                    }
                }
            }
        }
        Ok((deposits, withdraws))
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use crate::{
        TransactionReceipt, SignedTransaction, Transaction, Action, ActionTransfer,
        AccountName, ActionName, BlockHeader, BlockTimestamp, Asset
    };
    use super::*;

    #[test]
    fn block_generate_should_work() {
        let block_timestamp = BlockTimestamp::now();
        let producer = AccountName::from_str("eosio").unwrap();
        let block_header = BlockHeader::new(
            block_timestamp,
            producer,
            0,
            Checksum256::from([10u8; 32]),
            Checksum256::from([11u8; 32]),
            Checksum256::from([12u8; 32]),
            0,
            None,
            vec![]
        );
        let producer_signature = Default::default();
        let signed_block_header = SignedBlockHeader::new(block_header, producer_signature);

        let block = SignedBlock::new(signed_block_header);
        println!("{}", serde_json::to_string_pretty(&block).unwrap());
    }

    #[test]
    fn block_read_should_work() {
        let data = hex::decode("dded404a0000000000ea3055000000001b41d39f263026aa8916529450c964a8724a2d71498dbcefead211a24f720000000000000000000000000000000000000000000000000000000000000000bf17e8f5e8024c2f017f7861004750287b861c08ddb74b15c848ebf3bde11afd000000000000001f6db047c02fb436bd3c6d04593b5d3254be0f72a6c747453ef66d4d4c7b7987a128705a976b8f653997849b6c17191866be8d2f384ea01cac75eb1fecf67c7e910000").unwrap();
        let block = SignedBlock::read(&data.as_slice(), &mut 0).unwrap();
        println!("{}", serde_json::to_string_pretty(&block).unwrap());
    }

    #[test]
    fn block_read_with_transaction_should_work() {
        let data = hex::decode("0f57684a0000000000ea3055000000077cb6d5534a23579751f578148b8f0f2da54cd22243b4d6c17ba398ab8a900096714e43362a3bf531eaf43114603689e5561a36aa08225329eca7d939d22049b91659d7073782d1c456a29dde5ace92dffde0cfa78bb284e8d4d7f976fda1000000000000001f36f6f52520fa593f567826935186688d6bb6de7938ec8102c7f726bafe7cc8ae2b5585a3c8ee3a1e79011726b77a2b5f9a0593391ce7fc42c42b2e4a43cc011001005301000010010100206b22f146d8bfe03a7a03b760cb2539409b05f9961543ee41c31f0cf493267b8c244d1517a6aa67cf47f294755d9e2fb5dda6779f5d88d6e4461f380a2b02964b000053256fa15db57c56c88ddb000000000100a6823403ea3055000000572d3ccdcd010000000000855c3400000000a8ed3232210000000000855c340000000000000e3d102700000000000004454f5300000000000000").unwrap();
        let mut pos = 0;
        let block = SignedBlock::read(&data.as_slice(), &mut pos).unwrap();
        println!("{}", serde_json::to_string_pretty(&block).unwrap());
    }

    #[test]
    fn transaction_receipt_header_should_work() {
        let data = hex::decode("00530100001001").unwrap();
        let mut pos = 0;
        let header = TransactionReceiptHeader::read(&data.as_slice(), &mut pos).unwrap();
        dbg!(&header);
        dbg!(&pos);
    }

    #[test]
    fn action_filter_should_work() {
        let alice = AccountName::from_str("alice").unwrap();
        let bob = AccountName::from_str("bob").unwrap();

        let account = AccountName::from_str("eosio.token").unwrap();
        let name = ActionName::from_str("transfer").unwrap();

        let transfer = ActionTransfer {
            from: alice,
            to: bob,
            quantity: Asset::from_str("1.0000 EOS").unwrap(),
            memo: "test transfer".to_string()
        };
        let action = Action {
            account,
            name,
            data: transfer.to_serialize_data(),
            ..Default::default()
        };
        let raw_trx = Transaction {
            actions: vec![action],
            ..Default::default()
        };

        let signed_trx = SignedTransaction {
            trx: raw_trx,
            ..Default::default()
        };

        let tx_receipt = TransactionReceipt {
            trx: PackedTransaction::from(signed_trx),
            ..Default::default()
        };

        let block = SignedBlock {
            transactions: vec![tx_receipt],
            ..Default::default()
        };

        let filter = ActionFilter {
            account,
            name,
        };

        let output = filter.filter(&block, &alice).unwrap();

        match output.0[0] {
            ActionType::Withdraw(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(output.1.len(), 0);
    }
}
