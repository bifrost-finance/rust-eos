//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/contracts/eosio/action.hpp#L249-L274>
use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use core::str::FromStr;
use codec::{Encode, Decode};
use crate::{
    AccountName, ActionName, Asset, Digest, NumBytes,
    PermissionLevel, Read, SerializeData, Write
};
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize, de::Error, ser::{Serializer, SerializeStruct}};


/// This is the packed representation of an action along with meta-data about
/// the authorization levels.
#[derive(Clone, Debug, Read, Write, NumBytes, PartialEq, Default, Encode, Decode)]
#[eosio_core_root_path = "crate"]
pub struct Action {
    /// Name of the account the action is intended for
    pub account: AccountName,
    /// Name of the action
    pub name: ActionName,
    /// List of permissions that authorize this action
    pub authorization: Vec<PermissionLevel>,
    /// Payload data
    pub data: Vec<u8>,
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::de::Deserializer<'de>
    {
        #[derive(Debug)]
        struct VisitorAction;
        impl<'de> serde::de::Visitor<'de> for VisitorAction
        {
            type Value = Action;
            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "string or a struct, but this is: {:?}", self)
            }

            fn visit_map<D>(self, mut map: D) -> Result<Self::Value, D::Error>
                where D: serde::de::MapAccess<'de>,
            {
                let mut account = AccountName::default();
                let mut name = ActionName::default();
                let mut authorization: Vec<PermissionLevel> = vec![];
                let mut data: Vec<u8> = vec![];
                while let Some(field) = map.next_key()? {
                    match field {
                        "account" => {
                            account = map.next_value()?;
//                            account = AccountName::from_str(&val).map_err(|_| D::Error::custom("failed to parse the filed account."))?;
                        }
                        "name" => {
                            name = map.next_value()?;
//                            act_digest = ActionName::from_str(&val).map_err(|_| D::Error::custom("failed to parse the filed name."))?;
                        }
                        "authorization" => {
                            authorization= map.next_value()?;
                        }
                        "hex_data" => {
                            let val: String= map.next_value()?;
                            data = hex::decode(val).map_err(D::Error::custom)?;
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                            continue;
                        }
                    }
                }
                let action = Action {
                    account,
                    name,
                    authorization,
                    data,
                };
                Ok(action)
            }
        }
        deserializer.deserialize_any(VisitorAction)
    }
}

impl Action {
    pub fn new(account: AccountName, name: ActionName, authorization: Vec<PermissionLevel>, data: Vec<u8>) -> Self {
        Action { account, name, authorization, data }
    }

    pub fn from_str<T: AsRef<str>, S: SerializeData>(
        account: T,
        name: T,
        authorization: Vec<PermissionLevel>,
        action_data: S
    ) -> crate::Result<Self> {
        let account = FromStr::from_str(account.as_ref()).map_err(crate::Error::from)?;
        let name =  FromStr::from_str(name.as_ref()).map_err(crate::Error::from)?;
        let data = action_data.to_serialize_data();

        Ok(Action { account, name, authorization, data })
    }

    pub fn transfer<T: AsRef<str>>(from: T, to: T, quantity: T, memo: T) -> crate::Result<Action> {
        let permission_level = PermissionLevel::from_str(from.as_ref(), "active")?;
        let action_transfer = ActionTransfer::from_str(from, to, quantity, memo)?;
        Action::from_str(
            "eosio.token",
            "transfer",
            vec![permission_level],
            action_transfer
        )
    }
}

impl Digest for Action {}
impl SerializeData for Action {}

impl core::fmt::Display for Action {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "account: {}\n\
            name: {}\n\
            authorization: {}\n\
            hex_data: {}",
            self.account,
            self.name,
            self.authorization.iter().map(|item| format!("{}", item)).collect::<String>(),
            hex::encode(&self.data),
        )
    }
}

#[cfg(feature = "std")]
impl serde::ser::Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Action", 5)?;
        state.serialize_field("account", &self.account)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("authorization", &self.authorization)?;
        state.serialize_field("hex_data", &hex::encode(&self.data))?;
        match (self.account.to_string().as_str(), self.name.to_string().as_str()) {
            ("eosio.token", "transfer") => {
                let data = ActionTransfer::read(&self.data, &mut 0).expect("Action read from data failed.");
                state.serialize_field("data", &data)?;
            },
            _ => {}
        }
        state.end()
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Read, Write, NumBytes, Default)]
#[eosio_core_root_path = "crate"]
pub struct ActionTransfer {
    pub from: AccountName,
    pub to: AccountName,
    pub quantity: Asset,
    pub memo: String,
}

impl ActionTransfer {
    pub fn new(from: AccountName, to: AccountName, quantity: Asset, memo: String) -> Self {
        ActionTransfer { from, to, quantity, memo }
    }

    pub fn from_str<T: AsRef<str>>(from: T, to: T, quantity: T, memo: T) -> crate::Result<Self> {
        let from = FromStr::from_str(from.as_ref()).map_err(crate::Error::from)?;
        let to = FromStr::from_str(to.as_ref()).map_err(crate::Error::from)?;
        let quantity = FromStr::from_str(quantity.as_ref()).map_err(crate::Error::from)?;
        let memo = memo.as_ref().to_string();

        Ok(ActionTransfer { from, to, quantity, memo })
    }
}

impl SerializeData for ActionTransfer {}

pub trait ToAction: Write + NumBytes {
    const NAME: u64;

    #[inline]
    fn to_action(
        &self,
        account: AccountName,
        authorization: Vec<PermissionLevel>,
    ) -> Action {
        let mut data = vec![0_u8; self.num_bytes()];
        self.write(&mut data, &mut 0).expect("write");

        Action {
            account,
            name: Self::NAME.into(),
            authorization,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use hex;

    use super::*;

    #[test]
    fn action_hash_should_work() {
        let action = Action {
            account: FromStr::from_str("eosio.token").unwrap(),
            name: FromStr::from_str("issue").unwrap(),
            authorization: vec![PermissionLevel {
                actor: FromStr::from_str("eosio").unwrap(),
                permission: FromStr::from_str("active").unwrap(),
            }],
            data: hex::decode("0000000000ea305500625e5a1809000004454f530000000004696e6974").unwrap(),
        };

        let hash = action.digest().unwrap();
        assert_eq!(hash, "0221f3da945a3de738cdb744f7963a6a3486097ab42436d1f4e13a1ade502bb9".into());
    }

    #[test]
    fn action_transfer_serialize_should_work() {
        let action = Action::transfer("testa", "testb", "1.0000 EOS", "a memo").ok().unwrap();
        let data = action.to_serialize_data();
        assert_eq!(
            hex::encode(data),
            "00a6823403ea3055000000572d3ccdcd01000000000093b1ca00000000a8ed323227000000000093b1ca000000008093b1ca102700000000000004454f53000000000661206d656d6f"
        );
    }

    #[test]
    fn action_deserialize_should_be_ok() {
        let action_str = r#"
        {
            "account": "eosio.token",
            "name": "transfer",
            "authorization": [
                {
                    "actor": "junglefaucet",
                    "permission": "active"
                }
            ],
            "data": {
                "from": "junglefaucet",
                "receiver": "megasuper333",
                "stake_net_quantity": "1.0000 EOS",
                "stake_cpu_quantity": "1.0000 EOS",
                "transfer": 1
            },
            "hex_data": "9015d266a9c8a67e30c6b8aa6a6c989240420f000000000004454f5300000000134e657720425020526567697374726174696f6e"
        }"#;

        let action: Result<Action, _> = serde_json::from_str(action_str);
        assert!(action.is_ok());
        let hash = action.unwrap().digest().unwrap();
        assert_eq!(hash, "eaa3b4bf845a1b41668ab7ca49fb5644fc91a6c0156dfd33911b4ec69d2e41d6".into())
    }
}
