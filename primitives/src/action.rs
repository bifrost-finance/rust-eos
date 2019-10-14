//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/contracts/eosio/action.hpp#L249-L274>
use crate::{AccountName, ActionName, NumBytes, PermissionLevel, Read, Write, Asset, SerializeData};
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use core::str::FromStr;

/// This is the packed representation of an action along with meta-data about
/// the authorization levels.
#[derive(Clone, Debug, Read, Write, NumBytes, Default)]
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

impl Action {
    pub fn new(account: AccountName, name: ActionName, authorization: Vec<PermissionLevel>, data: Vec<u8>) -> Self {
        Action { account, name, authorization, data }
    }

    pub fn from_str<T: AsRef<str>, S: SerializeData>(
        account: T,
        name: T,
        authorization: Vec<PermissionLevel>,
        action_data: S
    ) -> Result<Self, crate::Error> {
        let account = AccountName::from_str(account.as_ref()).map_err(crate::Error::from)?;
        let name =  ActionName::from_str(name.as_ref()).map_err(crate::Error::from)?;
        let data = action_data.to_serialize_data();

        Ok(Action { account, name, authorization, data })
    }
}

impl SerializeData for Action {}

#[derive(Clone, Debug, Read, Write, NumBytes, Default)]
#[eosio_core_root_path = "crate"]
pub struct ActionTransfer {
    from: AccountName,
    to: AccountName,
    amount: Asset,
    memo: String,
}

impl ActionTransfer {
    pub fn new(from: AccountName, to: AccountName, amount: Asset, memo: String) -> Self {
        ActionTransfer { from, to, amount, memo }
    }

    pub fn from_str<T: AsRef<str>>(from: T, to: T, amount: T, memo: T)
        -> Result<Self, crate::Error>
    {
        let from = AccountName::from_str(from.as_ref()).map_err(crate::Error::from)?;
        let to = AccountName::from_str(to.as_ref()).map_err(crate::Error::from)?;
        let amount = Asset::from_str(amount.as_ref()).map_err(crate::Error::from)?;
        let memo = memo.as_ref().to_string();

        Ok(ActionTransfer { from, to, amount, memo })
    }
}

impl core::fmt::Display for Action {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "account: {}\n\
            name: {}\n\
            authorization: {}\n\
            hex_data: {}",
            self.account,
            self.name,
            self.authorization.iter().map(|item| format!("{}", item)).collect::<String>(),
            // TODO display data,
            hex::encode(&self.data),
        )
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
    use super::*;
    use hex;

    #[test]
    fn action_should_work() {
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

        let data = action.to_serialize_data();
        assert_eq!(
            hex::encode(data),
            "00a6823403ea3055000000572d3ccdcd01000000000093b1ca00000000a8ed323227000000000093b1ca000000008093b1ca102700000000000004454f53000000000661206d656d6f"
        );
    }
}
