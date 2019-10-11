use crate::{Read, Write, NumBytes};

#[derive(Clone, Debug, Read, Write, NumBytes, Default, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct SyncRequestMessage {
    pub start_block: u32,
    pub end_block: u32,
}

impl SyncRequestMessage {
    pub fn new(start_block: u32, end_block: u32) -> Self {
        Self {
            start_block,
            end_block,
        }
    }
}

impl core::fmt::Display for SyncRequestMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\nstart_block: {}, end_block: {}\n",
            self.start_block,
            self.end_block
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Read;
    use hex;

    #[test]
    fn sync_request_message_test() {
        let data = hex::decode("540000005f000000");
        let data = data.unwrap();
        let mut pos = 0usize;
        let msg = SyncRequestMessage::read(&data.as_slice(), &mut pos);
        println!("{}", msg.unwrap());
        println!("Pos: {}", pos);
    }
}
