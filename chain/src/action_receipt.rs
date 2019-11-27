use crate::{AccountName, Checksum256, UnsignedInt, Digest, Read, Write, NumBytes, SerializeData};
use crate::utils::flat_map::FlatMap;
use codec::{Encode, Decode};

#[derive(Clone, Debug, Read, Write, NumBytes, Default, Encode, Decode)]
#[eosio_core_root_path = "crate"]
pub struct ActionReceipt {
    receiver: AccountName,
    act_digest: Checksum256,
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
}
