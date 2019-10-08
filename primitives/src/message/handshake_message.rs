use crate::{Checksum256, Signature, PublicKey, TimePoint, Read, Write, NumBytes};

#[derive(Clone, Debug, Read, Write, NumBytes, Default, PartialEq)]
#[eosio_core_root_path = "crate"]
pub struct HandshakeMessage {
    network_version: u16,
    chain_id: Checksum256,
    node_id: Checksum256,
    key: PublicKey,
    time: TimePoint,
    token: Checksum256,
    sig: Signature,
    p2p_address: String,
    last_irreversible_block_num: u32,
    last_irreversible_block_id: Checksum256,
    head_num: u32,
    head_id: Checksum256,
    os: String,
    agent: String,
    generation: i16,
}

impl core::fmt::Display for HandshakeMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\nnetwork_version: {}\n\
            chain_id: {}\n\
            node_id: {}\n\
            key: {}\n\
            time: {}\n\
            token: {}\n\
            sig: {}\n\
            p2p_address: {}\n\
            last_irreversible_block_num: {}\n\
            last_irreversible_block_id: {}\n\
            head_num: {}\n\
            head_id: {}\n\
            os: {}\n\
            agent: {}\n\
            generation: {}\n",
            self.network_version,
            self.chain_id,
            self.node_id,
            self.key,
            self.time,
            self.token,
            self.sig,
            self.p2p_address,
            self.last_irreversible_block_num,
            self.last_irreversible_block_id,
            self.head_num,
            self.head_id,
            self.os,
            self.agent,
            self.generation,
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Read;
    use hex;

    // b604
    // cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f
    // 853a072780a4981523cfc6b0b887f2779d65f9d036b340c23ae1aa0e56e2c43a
    // 00000000000000000000000000000000000000000000000000000000000000000000
    // a86736d791c6c915
    // 0000000000000000000000000000000000000000000000000000000000000000
    // 0000000000000000000000000000000000000000000000000000000000000000
    // 00000000000000000000000000000000000000000000000000000000000000000000
    // 183132372e302e302e313a39383737202d2038353361303732
    // 5f000000
    // 0000005f6cb3522adf6fb9516a1459985acafac26d41ee7ee76eed77131d4e19
    // 60000000
    // 00000060588ec0a6f0b0acb5aa755886a9b0d8f2a55d9638b36715939826739e
    // 036f7378
    // 1022454f532054657374204167656e7422
    // 0300
    #[test]
    fn handshake_message_test1() {
        let data = hex::decode("b604cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f853a072780a4981523cfc6b0b887f2779d65f9d036b340c23ae1aa0e56e2c43a00000000000000000000000000000000000000000000000000000000000000000000a86736d791c6c9150000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000183132372e302e302e313a39383737202d20383533613037325f0000000000005f6cb3522adf6fb9516a1459985acafac26d41ee7ee76eed77131d4e196000000000000060588ec0a6f0b0acb5aa755886a9b0d8f2a55d9638b36715939826739e036f73781022454f532054657374204167656e74220300");
        let data = data.unwrap();
        let mut pos = 0usize;
        let msg = HandshakeMessage::read(&data.as_slice(), &mut pos);
        println!("{}", msg.unwrap());
        println!("pos: {}", pos);
    }

    #[test]
    fn handshake_message_test2() {
        let data = hex::decode("b604cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f9f3cd5460639393b216efdde31150b6420cfb06a962f8b7fe184d89857f84af200000000000000000000000000000000000000000000000000000000000000000000b06ae7d491c6c91500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002b57616e67456477696e64654d6163426f6f6b2d50726f2e6c6f63616c3a39383736202d20396633636435345f0000000000005f6cb3522adf6fb9516a1459985acafac26d41ee7ee76eed77131d4e196000000000000060588ec0a6f0b0acb5aa755886a9b0d8f2a55d9638b36715939826739e036f73781022454f532054657374204167656e74220100");
        let data = data.unwrap();
        let mut pos = 0usize;
        let msg = HandshakeMessage::read(&data.as_slice(), &mut pos);
        println!("{}", msg.unwrap());
        println!("pos: {}", pos);
    }
}
