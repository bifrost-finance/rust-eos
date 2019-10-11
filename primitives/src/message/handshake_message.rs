use crate::{Checksum256, Signature, PublicKey, TimePoint, Read, Write, NumBytes};

///
/// For a while, network version was a 16 bit value equal to the second set of 16 bits
/// of the current build's git commit id. We are now replacing that with an integer protocol
/// identifier. Based on historical analysis of all git commit identifiers, the larges gap
/// between ajacent commit id values is shown below.
/// these numbers were found with the following commands on the master branch:
///
/// git log | grep "^commit" | awk '{print substr($2,5,4)}' | sort -u > sorted.txt
/// rm -f gap.txt; prev=0; for a in $(cat sorted.txt); do echo $prev $((0x$a - 0x$prev)) $a >> gap.txt; prev=$a; done; sort -k2 -n gap.txt | tail
///
/// DO NOT EDIT net_version_base OR net_version_range!
///
pub const NET_VERSION_BASE: u16 = 0x04b5;
pub const NET_VERSION_RANGE: u16 = 106;
///
/// If there is a change to network protocol or behavior, increment net version to identify
/// the need for compatibility hooks
///
pub const PROTO_BASE: u16 = 0;
pub const PROTO_EXPLICIT_SYNC: u16 = 1;

pub const NET_VERSION: u16 = PROTO_EXPLICIT_SYNC;

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
    pub generation: i16,
}

impl HandshakeMessage {
    pub fn populate(hello: &mut HandshakeMessage) {
        hello.network_version = NET_VERSION_BASE + NET_VERSION;
        hello.chain_id = [0xcf, 0x05, 0x7b, 0xbf, 0xb7, 0x26, 0x40, 0x47, 0x1f, 0xd9, 0x10, 0xbc, 0xb6, 0x76, 0x39, 0xc2, 0x2d, 0xf9, 0xf9, 0x24, 0x70, 0x93, 0x6c, 0xdd, 0xc1, 0xad, 0xe0, 0xe2, 0xf2, 0xe7, 0xdc, 0x4f].into();
        hello.node_id  = [0x0d, 0x67, 0xb6, 0xd8, 0xdc, 0xcf, 0x22, 0xb7, 0xcf, 0xb6, 0xd3, 0x86, 0xaa, 0xe2, 0x1d, 0x85, 0x21, 0x92, 0x7f, 0x9a, 0xbf, 0x2f, 0x6e, 0xa6, 0x4f, 0x4c, 0xb0, 0x08, 0xa7, 0x60, 0xb4, 0x54].into();
        hello.key = Default::default();
        hello.time = Default::default();
        hello.token = Default::default();
        hello.sig = Default::default();
        hello.p2p_address = format!("127.0.0.1:9877 - {}", hello.node_id.to_string()[..7].to_string());
        hello.os = "osx".to_owned();
        hello.agent = "agent".to_owned();
        hello.head_id = [0x00, 0x00, 0x00, 0x01, 0xbc, 0xf2, 0xf4, 0x48, 0x22, 0x5d, 0x09, 0x96, 0x85, 0xf1, 0x4d, 0xa7, 0x68, 0x03, 0x02, 0x89, 0x26, 0xaf, 0x04, 0xd2, 0x60, 0x7e, 0xaf, 0xcf, 0x60, 0x9c, 0x26, 0x5c].into();
        hello.head_num = 1;
        hello.last_irreversible_block_id = [0x00, 0x00, 0x00, 0x01, 0xbc, 0xf2, 0xf4, 0x48, 0x22, 0x5d, 0x09, 0x96, 0x85, 0xf1, 0x4d, 0xa7, 0x68, 0x03, 0x02, 0x89, 0x26, 0xaf, 0x04, 0xd2, 0x60, 0x7e, 0xaf, 0xcf, 0x60, 0x9c, 0x26, 0x5c].into();
        hello.last_irreversible_block_num = 1;
    }
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
