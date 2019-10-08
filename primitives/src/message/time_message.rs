use crate::{TimePoint, Read, Write, NumBytes};

#[derive(Clone, Debug, Read, Write, NumBytes, Default, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct TimeMessage {
    /// origin timestamp
    org: TimePoint,
    /// receive timestamp
    rec: TimePoint,
    /// transmit timestamp
    xmt: TimePoint,
    /// destination timestamp
    dst: TimePoint,
}

impl core::fmt::Display for TimeMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\norigin timestamp: {}\n\
            receive timestamp: {}\n\
            transmit timestamp: {}\n\
            destination timestamp: {}\n",
            self.org,
            self.rec,
            self.xmt,
            self.dst,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Read;
    use hex;

    #[test]
    fn time_message_test() {
        let data = hex::decode("a86736d791c6c915a86736d791c6c915a86736d791c6c915a86736d791c6c915");
        let data = data.unwrap();
        let mut pos = 0usize;
        let msg = TimeMessage::read(&data.as_slice(), &mut pos);
        println!("{}", msg.unwrap());
        println!("Pos: {}", pos);
    }
}

