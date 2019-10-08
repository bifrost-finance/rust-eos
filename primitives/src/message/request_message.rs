use crate::{Read, Write, NumBytes, SelectIds};

#[derive(Clone, Debug, Read, Write, NumBytes, Default, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct RequestMessage {
    pub known_trx: SelectIds,
    pub known_blocks: SelectIds,
}

impl core::fmt::Display for RequestMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\nknown_trx: {}\n\
            known_blocks: {}\n",
            self.known_trx,
            self.known_blocks,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Read;
    use hex;

    #[test]
    fn request_message_test() {
        let data = hex::decode("020000005f00000000020000006000000000");
        let data = data.unwrap();
        let mut pos = 0usize;
        let msg = RequestMessage::read(&data.as_slice(), &mut pos);
        println!("{}", msg.unwrap());
        println!("Pos: {}", pos);
    }
}

