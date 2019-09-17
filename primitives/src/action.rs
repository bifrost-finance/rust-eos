//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/contracts/eosio/action.hpp#L249-L274>
use crate::{AccountName, ActionName, NumBytes, PermissionLevel, Read, Write, Asset};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// This is the packed representation of an action along with meta-data about
/// the authorization levels.
#[derive(Clone, Debug, Serialize, Deserialize, Read, Write, NumBytes, Default)]
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

    pub fn from_str<T: AsRef<str>>(account: T, name: T, authorization: Vec<PermissionLevel>, data: Vec<u8>)
        -> Result<Self, crate::Error>
    {
        let account = AccountName::from_str(account.as_ref()).map_err(|err| crate::Error::from(err) )?;
        let name =  ActionName::from_str(name.as_ref()).map_err(|err| crate::Error::from(err) )?;

        Ok(Action { account, name, authorization, data })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Read, Write, NumBytes, Default)]
#[eosio_core_root_path = "crate"]
struct ActionTransfer {
    from: AccountName,
    to: AccountName,
    amount: Asset,
    memo: String,
}

impl ActionTransfer {
    fn new(from: AccountName, to: AccountName, amount: Asset, memo: String) -> Self {
        ActionTransfer { from, to, amount, memo }
    }

    fn from_str<T: AsRef<str>>(from: T, to: T, amount: T, memo: T)
        -> Result<Self, crate::Error>
    {
        let from = AccountName::from_str(from.as_ref()).map_err(|err| crate::Error::from(err) )?;
        let to = AccountName::from_str(to.as_ref()).map_err(|err| crate::Error::from(err) )?;
        let amount = Asset::from_str(amount.as_ref()).map_err(|err| crate::Error::from(err) )?;
        let memo = memo.as_ref().to_string();

        Ok(ActionTransfer { from, to, amount, memo })
    }

    fn to_action_data(&self) -> Vec<u8> {
        let mut data = vec![0u8; 1024];
        let mut pos = 0;
        self.write(&mut data, &mut pos).expect("write");
        data[..pos].to_vec()
    }
}

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

    #[test]
    fn action_should_work() {
        let permission_level = PermissionLevel::from_str("testa", "active").ok().unwrap();
        let authorization: Vec<PermissionLevel> = vec![permission_level];

        let from = "testa";
        let to = "testb";
        let amount = "1.0000 EOS";
        let memo = "a memo";
        let action_transfer = ActionTransfer::from_str(from, to, amount, memo).ok().unwrap();
        let data = action_transfer.to_action_data();
        let account = "eosio.token";
        let name = "transfer";
        let action = Action::from_str(account, name, authorization, data).ok().unwrap();

        let mut data1 = vec![0u8; 1024];
        let mut pos = 0;
        action.write(&mut data1, &mut pos).expect("write");;
        dbg!(format!("{:02x?})", &data1[..pos]));
        dbg!(format!("{:02x?})", pos));
    }
}
