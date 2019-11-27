use alloc::vec::Vec;
use alloc::string::{ToString, String};
use alloc::{format, vec};
use core::iter::{IntoIterator, Iterator};
use core::convert::TryFrom;
use core::str::FromStr;
use codec::{Encode, Decode};
#[cfg(feature = "std")]
use keys::secret::SecretKey;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use serde::ser::{Serializer, SerializeStruct};

use crate::{
    Action,
    utils::bitutil,
    Checksum256,
    Extension,
    NumBytes,
    Read,
    ReadError,
    SerializeData,
    TimePointSec,
    TrxKinds,
    UnsignedInt,
    Write,
    WriteError,
};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
pub enum CompressionType {
    None,
    Zlib,
}

impl Default for CompressionType {
    fn default() -> Self {
        CompressionType::None
    }
}

impl From<u8> for CompressionType {
    fn from(mode: u8) -> Self {
        match mode {
            0 => CompressionType::None,
            1 => CompressionType::Zlib,
            _ => CompressionType::None,
        }
    }
}

impl From<CompressionType> for u8 {
    fn from(mode: CompressionType) -> Self {
        match mode {
            CompressionType::None => 0,
            CompressionType::Zlib => 1,
        }
    }
}

impl NumBytes for CompressionType {
    fn num_bytes(&self) -> usize {
        1
    }
}

impl Read for CompressionType {
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u8::read(bytes, pos).map(|res| CompressionType::from(res))
    }
}

impl Write for CompressionType {
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        u8::from(self.clone()).write(bytes, pos)
    }
}

impl core::fmt::Display for CompressionType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            CompressionType::None => write!(f, "None"),
            CompressionType::Zlib => write!(f, "Zlib"),
        }
    }
}

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq, Encode, Decode)]
#[eosio_core_root_path = "crate"]
pub struct PackedTransaction {
    pub signatures: Vec<crate::Signature>,
    pub compression: CompressionType,
    pub packed_context_free_data: Vec<u8>,
    pub packed_trx: Vec<u8>,
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for PackedTransaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct VisitorTrx;
        impl<'de> serde::de::Visitor<'de> for VisitorTrx
        {
            type Value = PackedTransaction;
            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
                where D: serde::de::MapAccess<'de>,
            {
                let mut signatures: Vec<crate::signature::Signature> = vec![];
                let mut compression: CompressionType = CompressionType::None;
                let mut packed_context_free_data: Vec<u8> = vec![];
                let mut packed_trx: Vec<u8> = vec![];
                while let Some(field) = map.next_key()? {
                    match field {
                        "signatures" => {
                            let val: Vec<String> = map.next_value()?;
                            signatures = val.iter().map(|v| {
                                crate::signature::Signature::from_str(v).unwrap()
                            }).collect::<Vec<_>>();
                        }
                        "compression" => {
                            compression = match map.next_value()? {
                                "none" => CompressionType::None,
                                "zlib" => CompressionType::Zlib,
                                _ => panic!("unknown compression variant"),
                            }
                        }
                        "packed_context_free_data" => {
                            let val: String = map.next_value()?;
                            packed_context_free_data = hex::decode(val).unwrap();
                        }
                        "packed_trx" => {
                            let val: String = map.next_value()?;
                            packed_trx = hex::decode(val).unwrap();
                        }
                        _ => {
                            // must give a type annotation here or compile with error
//                            let _: serde_json::Value = map.next_value()?;
                            let _: String = map.next_value()?;
                            continue;
                        }
                    }
                }
                Ok(PackedTransaction {
                    signatures,
                    compression,
                    packed_context_free_data,
                    packed_trx,
                })
            }
        }
        deserializer.deserialize_any(VisitorTrx)
    }
}

impl PackedTransaction {
    pub fn packed_digest(&self) -> crate::Result<Checksum256> {
        let prunable_size = self.signatures.num_bytes() + self.packed_context_free_data.num_bytes();
        let mut prunable_data = vec![0u8; prunable_size];
        let mut pos = 0;
        self.signatures.write(&mut prunable_data, &mut pos).map_err(crate::Error::BytesWriteError)?;
        self.packed_context_free_data.write(&mut prunable_data, &mut pos).map_err(crate::Error::BytesWriteError)?;
        let prunable = Checksum256::hash_from_slice(&prunable_data);

        let enc_size = self.compression.num_bytes() + self.packed_trx.num_bytes() + prunable.num_bytes();
        let mut enc_data = vec![0u8; enc_size];
        let mut pos = 0;
        self.compression.write(&mut enc_data, &mut pos).map_err(crate::Error::BytesWriteError)?;
        self.packed_trx.write(&mut enc_data, &mut pos).map_err(crate::Error::BytesWriteError)?;
        prunable.write(&mut enc_data, &mut pos).map_err(crate::Error::BytesWriteError)?;

        Ok(Checksum256::hash_from_slice(&enc_data))
    }
}

impl From<SignedTransaction> for PackedTransaction {
    fn from(signed: SignedTransaction) -> Self {
        let mut packed_trx = vec![0u8; signed.trx.num_bytes()];
        signed.trx.write(&mut packed_trx, &mut 0).expect("Convert transaction to packed failed");

        PackedTransaction {
            signatures: signed.signatures,
            compression: Default::default(),
            packed_context_free_data: signed.context_free_data,
            packed_trx,
        }
    }
}

impl SerializeData for PackedTransaction {}

impl core::fmt::Display for PackedTransaction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "signatures: {}\n\
            compression: {}\n\
            packed_context_free_data: {}\n\
            packed_trx: {}\n\
            transaction: {}",
            self.signatures.iter().map(|item| format!("{:?}", item)).collect::<String>(),
            self.compression,
            hex::encode(&self.packed_context_free_data),
            hex::encode(&self.packed_trx),
            Transaction::try_from(TrxKinds::PackedTransaction(self.clone())).expect("Convert transaction failed"),
        )
    }
}

#[cfg(feature = "std")]
impl serde::ser::Serialize for PackedTransaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("PackedTransaction", 5)?;
        state.serialize_field("signatures", &self.signatures)?;
        state.serialize_field("compression", &self.compression)?;
        state.serialize_field("packed_context_free_data", &self.packed_context_free_data)?;
        state.serialize_field("packed_trx", &hex::encode(&self.packed_trx))?;
        state.serialize_field("transaction", &Transaction::read(&self.packed_trx.as_slice(), &mut 0)
            .expect("Transaction read from packed trx failed."))?;
        state.end()
    }
}

#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
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
    pub fn new(expiration: TimePointSec, ref_block_num: u16, ref_block_prefix: u32) -> Self {
        TransactionHeader {
            expiration,
            ref_block_num,
            ref_block_prefix,
            max_net_usage_words: 0u32.into(),
            max_cpu_usage_ms: 0,
            delay_sec: 0u32.into(),
        }
    }

    pub fn set_reference_block(&mut self, reference_block_id: &Checksum256) {
        self.ref_block_num = bitutil::endian_reverse_u32(reference_block_id.hash0() as u32) as u16;
        self.ref_block_prefix = reference_block_id.hash1() as u32;
    }

    pub fn verify_reference_block(&self, reference_block_id: &Checksum256) -> bool {
        self.ref_block_num == bitutil::endian_reverse_u32(reference_block_id.hash0() as u32) as u16
            && self.ref_block_prefix == reference_block_id.hash1() as u32
    }
}

impl core::fmt::Display for TransactionHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "expiration: {}\n\
            ref_block_num: {}\n\
            ref_block_prefix: {}\n\
            max_net_usage_words: {}\n\
            max_cpu_usage_ms: {}\n\
            delay_sec: {}",
            self.expiration,
            self.ref_block_num,
            self.ref_block_prefix,
            self.max_net_usage_words,
            self.max_cpu_usage_ms,
            self.delay_sec,
        )
    }
}

#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[derive(NumBytes, Write, Read, Debug, Clone, Default)]
#[eosio_core_root_path = "crate"]
pub struct Transaction {
    pub header: TransactionHeader,
    pub context_free_actions: Vec<Action>,
    pub actions: Vec<Action>,
    pub transaction_extensions: Vec<Extension>,
}

impl Transaction {
    #[cfg(feature = "std")]
    pub fn new(delay_secs: u32, ref_block_num: u16, ref_block_prefix: u32, actions: Vec<Action>) -> Self {
        let expiration = TimePointSec::now().add_seconds(delay_secs);
        let header = TransactionHeader::new(expiration, ref_block_num, ref_block_prefix);

        Transaction {
            header,
            context_free_actions: vec![],
            actions,
            transaction_extensions: vec![],
        }
    }

    pub fn build(expiration: TimePointSec, block_id: Checksum256, actions: Vec<Action>) -> Self {
        let mut header = TransactionHeader::default();
        header.expiration = expiration;
        header.set_reference_block(&block_id);

        Transaction {
            header,
            context_free_actions: vec![],
            actions,
            transaction_extensions: vec![],
        }
    }

    #[cfg(feature = "std")]
    pub fn sign(&self, sk: SecretKey, chain_id: String) -> crate::Result<SignedTransaction> {
        let mut sign_data: Vec<u8>  = Vec::new();
        let mut chain_id_hex = hex::decode(chain_id)
            .map_err(crate::error::Error::FromHexError)?;
        sign_data.append(&mut chain_id_hex);
        sign_data.append(&mut self.to_serialize_data());
        sign_data.append(&mut vec![0u8; 32]);

        let sig = sk.sign(&sign_data.as_slice()).map_err(crate::error::Error::Keys)?;

        Ok(SignedTransaction {
            signatures: vec![sig.into()],
            context_free_data: vec![],
            trx: self.clone(),
        })
    }

    #[cfg(feature = "std")]
    pub fn generate_signature(&self, sk: impl AsRef<str>, chain_id: impl AsRef<str>) -> crate::Result<keys::signature::Signature> {
        let sk = SecretKey::from_wif(sk.as_ref()).map_err(crate::error::Error::Keys)?;
        let mut chain_id_hex = hex::decode(chain_id.as_ref())
            .map_err(crate::error::Error::FromHexError)?;
        let mut serialized = self.to_serialize_data();
        let pre_reserved  = chain_id_hex.len() + serialized.len() + 32;
        let mut sign_data: Vec<u8>  = Vec::with_capacity(pre_reserved);
        sign_data.append(&mut chain_id_hex);
        sign_data.append(&mut serialized);
        sign_data.append(&mut vec![0u8; 32]);

        let sig = sk.sign(&sign_data.as_slice()).map_err(crate::error::Error::Keys)?;
        Ok(sig)
    }

    #[cfg(feature = "std")]
    pub fn generate_signed_transaction(&self, sks: impl IntoIterator<Item=keys::signature::Signature>) -> SignedTransaction
    {
        let sks: Vec<crate::Signature> = sks.into_iter().map(|sk| sk.into()).collect();
        SignedTransaction {
            signatures: sks,
            context_free_data: vec![],
            trx: self.clone(),
        }
    }
}

impl TryFrom<TrxKinds> for Transaction {
    type Error = crate::Error;

    fn try_from(trx: TrxKinds) -> Result<Self, Self::Error> {
        match trx {
            TrxKinds::PackedTransaction(packed) => Transaction::read(packed.packed_trx.as_slice(), &mut 0).map_err(crate::Error::BytesReadError),
            TrxKinds::TransactionId(_) => Err(crate::Error::FromTrxKindsError),

        }
    }
}

impl From<PackedTransaction> for Transaction {
    fn from(packed: PackedTransaction) -> Self {
        Transaction::read(packed.packed_trx.as_slice(), &mut 0).expect("Transaction read from packed trx failed.")
    }
}

impl core::fmt::Display for Transaction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}\n\
            context_free_actions: {}\n\
            actions: {}\n\
            transaction_extensions: {}",
            self.header,
            self.context_free_actions.iter().map(Action::to_string).collect::<String>(),
            self.actions.iter().map(Action::to_string).collect::<String>(),
            self.transaction_extensions.iter().map(Extension::to_string).collect::<String>(),
        )
    }
}


impl SerializeData for Transaction {}

#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[derive(NumBytes, Write, Read, Debug, Clone, Default)]
#[eosio_core_root_path = "crate"]
pub struct SignedTransaction {
    pub signatures: Vec<crate::Signature>,
    pub context_free_data: Vec<u8>,
    pub trx: Transaction,
}

impl From<PackedTransaction> for SignedTransaction {
    fn from(packed: PackedTransaction) -> Self {
        SignedTransaction {
            signatures: packed.signatures,
            context_free_data: packed.packed_context_free_data,
            trx: Transaction::read(packed.packed_trx.as_slice(), &mut 0).expect("Transaction read from packed trx failed."),
        }
    }
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
    use keys::secret::SecretKey;

    use super::*;

    #[test]
    fn sign_tx_should_work() {
        let action = Action::transfer("testa", "testb", "1.0000 EOS", "a memo").ok().unwrap();
        let actions = vec![action];
        let trx = Transaction::new(300, 0, 0, actions);

        let chain_id = "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f".to_string();
        let sk = SecretKey::from_wif("5KUEhweMaSD2szyjU9EKjAyY642ZdVL2qzHW72dQcNRzUMWx9EL").unwrap();
        let signed_trx = trx.sign(sk, chain_id);
        assert!(signed_trx.is_ok());
        assert_eq!(
            hex::encode(&trx.to_serialize_data()[4..]),
            "000000000000000000000100a6823403ea3055000000572d3ccdcd01000000000093b1ca00000000a8ed323227000000000093b1ca000000008093b1ca102700000000000004454f53000000000661206d656d6f00"
        );
        println!("{}", serde_json::to_string_pretty(&signed_trx.ok().unwrap()).unwrap());
    }

    #[test]
    fn packed_trx_to_signed_trx_should_work() {
        let data = hex::decode("0100206b22f146d8bfe03a7a03b760cb2539409b05f9961543ee41c31f0cf493267b8c244d1517a6aa67cf47f294755d9e2fb5dda6779f5d88d6e4461f380a2b02964b000053256fa15db57c56c88ddb000000000100a6823403ea3055000000572d3ccdcd010000000000855c3400000000a8ed3232210000000000855c340000000000000e3d102700000000000004454f53000000000000").unwrap();
        let packed_trx = PackedTransaction::read(data.as_slice(), &mut 0).unwrap();
        let signed_trx: SignedTransaction = packed_trx.into();
        dbg!(&signed_trx);
    }

    #[test]
    fn packed_trx_tt() {
        let data = hex::decode("7785c25dbc8b0fe520ac000000000500a6823403ea3055000000572d3ccdcd01701534524d9d2f5d00000000a8ed323221701534524d9d2f5d301d456a524c9353010000000000000004454f53000000000000a6823403ea3055000000572d3ccdcd01701534524d9d2f5d00000000a8ed323221701534524d9d2f5d301d456a524c9353010000000000000004454f53000000000000a6823403ea3055000000572d3ccdcd01701534524d9d2f5d00000000a8ed323221701534524d9d2f5d301d456a524c9353010000000000000004454f53000000000000a6823403ea3055000000572d3ccdcd01701534524d9d2f5d00000000a8ed323221701534524d9d2f5d301d456a524c9353010000000000000004454f53000000000000a6823403ea3055000000572d3ccdcd01701534524d9d2f5d00000000a8ed323221701534524d9d2f5d301d456a524c9353010000000000000004454f53000000000000").unwrap();
        let packed_trx = PackedTransaction::read(data.as_slice(), &mut 0).unwrap();
        dbg!(&packed_trx);
    }

    #[test]
    fn set_and_verify_reference_block_should_work() {
        let mut data = [0u8; 32];
        data.copy_from_slice(hex::decode("0011dfc5d73d7b00b0c262c7ce4c4eea0494bc1790e8e71a85a2a7c6742320a1").unwrap().as_slice());
        let block_id = Checksum256::from(data);
        let mut trx_header = TransactionHeader::default();
        trx_header.set_reference_block(&block_id);
        assert_eq!(trx_header.ref_block_num, 57285);
        assert_eq!(trx_header.ref_block_prefix, 3345138352);
        assert_eq!(trx_header.verify_reference_block(&block_id), true);
    }

    #[test]
    fn packed_transaction_deserialization_should_work() {
        let p_trx = r#"
            {
            "id": "3463ee3c8a499bfe7b7b1bd1372f8a4840e357f24e8b43f22520a0ef6d9b236a",
            "signatures": [
              "SIG_K1_KYt8J2dEYCVg6j9kZes8vVNdNUrRUy35pAy1ZPPNVFhv1uWQB5G5qC5X6UasuWqejyRiLgH4e3GZfSKs83Ey8BKvP6jdHQ"
            ],
            "compression": "none",
            "packed_context_free_data": "",
            "context_free_data": [],
            "packed_trx": "5a85c25dc88baa41a5cd000000000100a6823403ea3055000000572d3ccdcd0120cf34924d37af3e00000000a8ed32322120cf34924d37af3e301d456a524c9353010000000000000004454f53000000000000",
            "transaction": {
              "expiration": "2019-11-06T08:33:30",
              "ref_block_num": 35784,
              "ref_block_prefix": 3450159530,
              "max_net_usage_words": 0,
              "max_cpu_usage_ms": 0,
              "delay_sec": 0,
              "context_free_actions": [],
              "actions": [
                {
                  "account": "eosio.token",
                  "name": "transfer",
                  "authorization": [
                    {
                      "actor": "burningmanbm",
                      "permission": "active"
                    }
                  ],
                  "data": {
                    "from": "burningmanbm",
                    "to": "eidosonecoin",
                    "quantity": "0.0001 EOS",
                    "memo": ""
                  },
                  "hex_data": "20cf34924d37af3e301d456a524c9353010000000000000004454f530000000000"
                }
              ],
              "transaction_extensions": []
            }
          }"#;
        let packed_trx: Result<PackedTransaction, _> = serde_json::from_str(&p_trx);
        assert!(packed_trx.is_ok());
    }
}
