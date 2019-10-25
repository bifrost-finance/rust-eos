//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/contracts/eosio/action.hpp#L180-L217>
use crate::{AccountName, NumBytes, PermissionName, Read, Write};
use core::str::FromStr;
use serde::{Deserialize, Serialize};

/// A permission
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Read, Write, NumBytes, Hash, PartialOrd, Ord, Deserialize, Serialize)]
#[eosio_core_root_path = "crate"]
pub struct PermissionLevel {
    pub actor: AccountName,
    pub permission: PermissionName,
}

impl PermissionLevel {
    pub fn new(actor: AccountName, permission: PermissionName, ) -> Self {
        PermissionLevel { actor, permission }
    }

    pub fn from_str<T: AsRef<str>>(actor: T, permission: T) -> crate::Result<Self> {
        let actor = AccountName::from_str(actor.as_ref()).map_err(crate::Error::from)?;
        let permission = PermissionName::from_str(permission.as_ref()).map_err(crate::Error::from)?;

        Ok(PermissionLevel { actor, permission })
    }
}

impl core::fmt::Display for PermissionLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "actor: {}\n\
            permission: {}",
            self.actor,
            self.permission,
        )
    }
}
