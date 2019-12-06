use crate::{
    AccountName, Checksum256, UnsignedInt,
    Digest, Read, Write, NumBytes, SerializeData,
    utils::flat_map::FlatMap
};
use core::str::FromStr;
use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::de::Error;

#[derive(Clone, Debug, Read, Write, NumBytes, Default, Encode, Decode, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct ActionReceipt {
    receiver: AccountName,
    pub act_digest: Checksum256,
    /// total number of actions dispatched since genesis
    global_sequence: u64,
    /// total number of actions with this receiver since genesis
    recv_sequence: u64,
    auth_sequence: FlatMap<AccountName, u64>,
    /// total number of setcodes
    code_sequence: UnsignedInt,
    /// total number of setabis
    abi_sequence: UnsignedInt,
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for ActionReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct VisitorRecdeipt;
        impl<'de> serde::de::Visitor<'de> for VisitorRecdeipt
        {
            type Value = ActionReceipt;
            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
                where D: serde::de::MapAccess<'de>,
            {
                let mut receiver = AccountName::default();
                let mut act_digest = Checksum256::default();
                let mut global_sequence = 0u64;
                let mut recv_sequence = 0u64;
                let mut auth_sequence = FlatMap::default();
                let mut code_sequence = UnsignedInt::default();
                let mut abi_sequence = UnsignedInt::default();
                while let Some(field) = map.next_key()? {
                    match field {
                        "receiver" => {
                            let val: String = map.next_value()?;
                            receiver = AccountName::from_str(&val).map_err(|_| D::Error::custom("failed to parse the filed receiver."))?;
                        }
                        "act_digest" => {
                            let val: String = map.next_value()?;
                            act_digest = Checksum256::from_str(&val).map_err(|_| D::Error::custom("failed to parse the filed act_digest."))?;
                        }
                        "global_sequence" => {
                            global_sequence= map.next_value()?;
                        }
                        "recv_sequence" => {
                            recv_sequence= map.next_value()?;
                        }
                        "auth_sequence" => {
                            let val: Vec<(AccountName, u64)> = map.next_value()?;
                            auth_sequence = FlatMap::assign(val);
                        }
                        "code_sequence" => {
                            let val: u32 = map.next_value()?;
                            code_sequence = UnsignedInt::from(val);
                        }
                        "abi_sequence" => {
                            let val: u32 = map.next_value()?;
                            abi_sequence = UnsignedInt::from(val);
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let receipt = ActionReceipt {
                    receiver,
                    act_digest,
                    global_sequence,
                    recv_sequence,
                    auth_sequence,
                    code_sequence,
                    abi_sequence,
                };
                Ok(receipt)
            }
        }
        deserializer.deserialize_any(VisitorRecdeipt)
    }
}

impl Digest for ActionReceipt {}
impl SerializeData for ActionReceipt {}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn action_receipt_digest_should_work() {
        let account = FromStr::from_str("eosio").unwrap();
        let action_receipt = ActionReceipt {
            receiver: account,
            act_digest: "765a9001f3648d8e4de93b4ac1fae775b6f6a6cc989f0b75060aacf9c1100b51".into(),
            global_sequence: 32u64,
            recv_sequence: 30u64,
            auth_sequence: FlatMap::new(account, 23u64),
            code_sequence: UnsignedInt::from(1u32),
            abi_sequence: UnsignedInt::from(1u32),
        };

        let hash = action_receipt.digest();
        assert_eq!(hash.unwrap(),  "af1558037fca2dda7e4dd00aac4a4c21a2bc9d801e6ecf635b58a87e928d1b09".into());
    }

    #[test]
    fn deserialization_action_receipt_should_be_ok() {
        // block 10775
        let receipt_str = r#"
            {
                "receiver":"eosio",
                "act_digest":"c64f119adfc7bd5a3001655ee8f91e68eecd629fb3d57d46e8a62410af1a86d4",
                "global_sequence":3040976,
                "recv_sequence":1323075,
                "auth_sequence":[["eosio",2959977]],
                "code_sequence":2,
                "abi_sequence":2
            }
        "#;

        let receipt: Result<ActionReceipt, _> = serde_json::from_str(receipt_str);
        assert!(receipt.is_ok());
        let digest = receipt.unwrap().digest();
        assert!(digest.is_ok());
        assert_eq!(digest.unwrap().to_string(),  "9f4d9c5a7fa93386e8b7dd3568b5f40a88f067fb984b9de8305a56ef333086a0");
    }
}
