use alloc::string::String;
use alloc::vec::Vec;
use alloc::{vec, format};
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
    Write,
    WriteError,
    ReadError
};
use core::{convert::TryFrom, str::FromStr};
use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq, Encode, Decode, SerializeData)]
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

    pub fn id(&self) -> crate::Result<Checksum256> {
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

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize))]
pub enum TrxKinds {
    TransactionId(Checksum256),
    PackedTransaction(PackedTransaction),
}

impl Default for TrxKinds {
    fn default() -> Self {
        TrxKinds::TransactionId(Default::default())
    }
}

impl NumBytes for TrxKinds {
    fn num_bytes(&self) -> usize {
        let kind_len = 1;
        match self {
            TrxKinds::TransactionId(id) => id.num_bytes() + kind_len,
            TrxKinds::PackedTransaction(packed) => packed.num_bytes() + kind_len,
        }
    }
}

impl Read for TrxKinds {
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        // read kind from first byte
        let kind = u8::read(bytes, pos)?;
        let result = match kind {
            0x00 => {
                let id = Checksum256::read(bytes, pos)?;
                TrxKinds::TransactionId(id)
            },
            0x01 => {
                let packed = PackedTransaction::read(bytes, pos)?;
                TrxKinds::PackedTransaction(packed)
            },
            _ => return Err(ReadError::NotSupportMessageType)
        };
        Ok(result)
    }
}

impl Write for TrxKinds {
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        match self {
            TrxKinds::TransactionId(id) => {
                0x00u8.write(bytes, pos)?;
                id.write(bytes, pos)
            },
            TrxKinds::PackedTransaction(packed) => {
                0x01u8.write(bytes, pos)?;
                packed.write(bytes, pos)
            },
        }
    }
}

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[eosio_core_root_path = "crate"]
pub struct TransactionReceipt {
    pub trx_receipt_header: TransactionReceiptHeader,
    pub trx: TrxKinds,
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for TransactionReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct VisitorTrxHeader;
        impl<'de> serde::de::Visitor<'de> for VisitorTrxHeader
        {
            type Value = TransactionReceipt;
            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
                where D: serde::de::MapAccess<'de>,
            {
                let mut status = 0u8;
                let mut cpu_usage_us = 0u32;
                let mut net_usage_words = UnsignedInt::default();
                let mut trx = TrxKinds::default();
                while let Some(field) = map.next_key()? {
                    match field {
                        "status" => {
                            status = match map.next_value()? {
                                "executed" => 0,
                                "soft_fail" => 1,
                                "hard_fail" => 2,
                                "delayed" => 3,
                                "expired" => 4,
                                _ => panic!("unknown status")
                            }
                        }
                        "cpu_usage_us" => {
                            let cpu: u32 = map.next_value()?;
                            cpu_usage_us = cpu;
                        }
                        "net_usage_words" => {
                            let val: u32 = map.next_value()?;
                            net_usage_words = UnsignedInt::from(val)
                        }
                        "trx" => {
                            let val: TrxKinds = map.next_value()?;
                            trx = val;
                        }
                        _ => {
//                            let _: serde_json::Value = map.next_value()?;
                            let _: String = map.next_value()?;
                            continue;
                        }
                    }
                }
                let trx_receipt_header = TransactionReceiptHeader {
                    status,
                    cpu_usage_us,
                    net_usage_words,
                };
                Ok(TransactionReceipt {
                    trx_receipt_header,
                    trx,
                })
            }
        }
        deserializer.deserialize_any(VisitorTrxHeader)
    }
}

impl core::str::FromStr for TrxKinds {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = hex::decode(s).map_err(crate::Error::FromHexError)?;
        let mut a: [u8;32] = [0u8;32];
        for i in 0..32 {
            a[i] = t[i];
        }
        let s = Checksum256::new(a);
        Ok(Self::TransactionId(s))
    }
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for TrxKinds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct IdOrPacketTrx(core::marker::PhantomData<fn() -> TrxKinds>);
        impl<'de> serde::de::Visitor<'de> for IdOrPacketTrx
        {
            type Value = TrxKinds;
            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                let t = hex::decode(value).map_err(E::custom)?;
                let mut a: [u8; 32] = [0u8; 32];
                for i in 0..32 {
                    a[i] = t[i];
                }
                let s = Checksum256::from(a);
                Ok(TrxKinds::TransactionId(s))
            }

            fn visit_map<D>(self, map: D) -> Result<Self::Value, D::Error>
                where
                    D: serde::de::MapAccess<'de>,
            {
                let s = serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                Ok(TrxKinds::PackedTransaction(s))
            }
        }
        deserializer.deserialize_any(IdOrPacketTrx(core::marker::PhantomData))
    }
}

impl TransactionReceipt {
    pub fn digest(&self) -> crate::Result<Checksum256> {
        let (digest_len, digest) = match self.trx {
            TrxKinds::TransactionId(ref id) => (id.num_bytes(), *id),
            TrxKinds::PackedTransaction(ref packed) => {
                let digest = packed.packed_digest()?;
                (digest.num_bytes(), digest)
            },
        };

        let enc_size = self.trx_receipt_header.num_bytes() + digest_len;
        let mut enc_data = vec![0u8; enc_size];
        let mut pos = 0;
        self.trx_receipt_header.write(&mut enc_data, &mut pos).map_err(crate::Error::BytesWriteError)?;
        digest.write(&mut enc_data, &mut pos).map_err(crate::Error::BytesWriteError)?;

        Ok(Checksum256::hash_from_slice(&enc_data))
    }
}

impl core::fmt::Display for TransactionReceipt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}",
            self.trx_receipt_header,
//            self.trx,
        )
    }
}

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
pub struct TransactionReceiptHeader {
    pub status: u8,
    pub cpu_usage_us: u32,
    pub net_usage_words: UnsignedInt,
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

    pub fn filter(&self, blocks: &SignedBlock, banker: &AccountName) -> crate::Result<Vec<ActionType>> {
        let mut output: Vec<ActionType> = vec![];
        if !blocks.transactions.is_empty() {
            for trx_receipt in &blocks.transactions {
                let packet_trx = trx_receipt.trx.clone();
                let trx = Transaction::try_from(packet_trx)?;
                for ac in &trx.actions {
                    if *self == *ac {
                        let action_transfer = ActionTransfer::read(&ac.data, &mut 0)
                            .map_err(crate::Error::BytesReadError)?;
                        if action_transfer.from.eq(banker) {
                            output.push(ActionType::Withdraw(action_transfer));
                        } else if action_transfer.to.eq(banker) {
                            output.push(ActionType::Deposit(action_transfer));
                        }
                    }
                }
            }
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use crate::{
        TransactionReceipt, SignedTransaction, Transaction, Action, ActionTransfer,
        AccountName, ActionName, BlockHeader, BlockTimestamp, Asset, merkle::merkle,
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

        let trxs: Vec<TransactionReceipt> = block.transactions;
        let mut trxs_digests: Vec<Checksum256> = Vec::new();
        for trx in trxs {
            trxs_digests.push(trx.digest().unwrap());
        }
        let merkle_root = merkle(trxs_digests.clone()).unwrap();
        assert_eq!(merkle_root, block.signed_block_header.block_header.transaction_mroot);
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
            data: transfer.to_serialize_data().expect("failed to serialize transfer data."),
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
            trx: TrxKinds::PackedTransaction(PackedTransaction::try_from(signed_trx).unwrap()),
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

        assert_eq!(output.len(), 1);
        match output[0] {
            ActionType::Withdraw(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
