use crate::{Checksum256, Read, Write, NumBytes, ReadError, WriteError};

#[derive(Clone, Debug, Read, Write, NumBytes, Default, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct GoAwayMessage {
    reason: u32,
    node_id: Checksum256,
}

impl core::fmt::Display for GoAwayMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "reason: {}, node_id: {}", self.reason, self.node_id)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GoAwayReason {
    NoReason,
    Myself,
    Duplicate,
    WrongChain,
    WrongVersion,
    Forked,
    Unlinkable,
    BadTransaction,
    Validation,
    BenignOther,
    FatalOther,
    Authentication,
}

impl core::fmt::Display for GoAwayReason {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let reason_str = match self {
            GoAwayReason::NoReason => "no reason",
            GoAwayReason::Myself => "self connect",
            GoAwayReason::Duplicate => "duplicate",
            GoAwayReason::WrongChain => "wrong chain",
            GoAwayReason::WrongVersion => "wrong version",
            GoAwayReason::Forked => "chain is forked",
            GoAwayReason::Unlinkable => "unlinkable block received",
            GoAwayReason::BadTransaction => "bad transaction",
            GoAwayReason::Validation => "invalid block",
            GoAwayReason::BenignOther => "some other non-fatal condition",
            GoAwayReason::FatalOther => "some other failure",
            GoAwayReason::Authentication => "authentication failure",
        };
        write!(f, "{}", reason_str)
    }
}

impl Default for GoAwayReason {
    fn default() -> Self {
        Self::NoReason
    }
}

impl NumBytes for GoAwayReason {
    fn num_bytes(&self) -> usize {
        4
    }
}

impl From<u32> for GoAwayReason {
    fn from(mode: u32) -> Self {
        match mode {
            0  => GoAwayReason::NoReason,
            1  => GoAwayReason::Myself,
            2  => GoAwayReason::Duplicate,
            3  => GoAwayReason::WrongChain,
            4  => GoAwayReason::WrongVersion,
            5  => GoAwayReason::Forked,
            6  => GoAwayReason::Unlinkable,
            7  => GoAwayReason::BadTransaction,
            8  => GoAwayReason::Validation,
            9  => GoAwayReason::BenignOther,
            10 => GoAwayReason::FatalOther,
            11 => GoAwayReason::Authentication,
            _  => GoAwayReason::NoReason,
        }
    }
}

impl From<GoAwayReason> for u32 {
    fn from(mode: GoAwayReason) -> Self {
        match mode {
            GoAwayReason::NoReason      => 0,
            GoAwayReason::Myself        => 1,
            GoAwayReason::Duplicate     => 2,
            GoAwayReason::WrongChain    => 3,
            GoAwayReason::WrongVersion  => 4,
            GoAwayReason::Forked        => 5,
            GoAwayReason::Unlinkable    => 6,
            GoAwayReason::BadTransaction=> 7,
            GoAwayReason::Validation    => 8,
            GoAwayReason::BenignOther   => 9,
            GoAwayReason::FatalOther    => 10,
            GoAwayReason::Authentication=> 11,
        }
    }
}

impl Read for GoAwayReason {
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u32::read(bytes, pos).map(|res| GoAwayReason::from(res))
    }
}

impl Write for GoAwayReason {
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        u32::from(self.clone()).write(bytes, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Read;
    use hex;

    #[test]
    fn go_away_message_test() {
        let data = hex::decode("020000005f00000000020000006000000000");
        let data = data.unwrap();
        let mut pos = 0usize;
        let msg = GoAwayMessage::read(&data.as_slice(), &mut pos);
        dbg!(&msg);
        dbg!(&pos);
    }
}

