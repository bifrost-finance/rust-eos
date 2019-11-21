#![allow(dead_code)]
use crate::{Checksum256, make_canonical_pair};

// given an unsigned integral number return the smallest
// power-of-2 which is greater than or equal to the given number
//
// @param value - an unsigned integral
// @return - the minimum power-of-2 which is >= value
fn next_power_of_2(mut value: u64) -> u64 {
    value -= 1;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    value |= value >> 32;
    value += 1;
    value
}

// Given a power-of-2 (assumed correct) return the number of leading zeros
//
// This is a classic count-leading-zeros in parallel without the necessary
// math to make it safe for anything that is not already a power-of-2
//
// @param value - and integral power-of-2
// @return the number of leading zeros
fn clz_power_2(value: u64) -> usize {
    let mut lz: usize = 64;

    if value != 0 { lz -= 1; }
    if (value & 0x00000000FFFFFFFF_u64) != 0 { lz -= 32; }
    if (value & 0x0000FFFF0000FFFF_u64) != 0 { lz -= 16; }
    if (value & 0x00FF00FF00FF00FF_u64) != 0 { lz -= 8; }
    if (value & 0x0F0F0F0F0F0F0F0F_u64) != 0 { lz -= 4; }
    if (value & 0x3333333333333333_u64) != 0 { lz -= 2; }
    if (value & 0x5555555555555555_u64) != 0 { lz -= 1; }

    lz
}

// Given a number of nodes return the depth required to store them
// in a fully balanced binary tree.
//
// @param node_count - the number of nodes in the implied tree
// @return the max depth of the minimal tree that stores them
fn calculate_max_depth(node_count: u64) -> usize {
    if node_count == 0 {
        return 0;
    }
    let implied_count = next_power_of_2(node_count);
    clz_power_2(implied_count) + 1
}

#[derive(Default, Debug)]
struct IncrementalMerkle {
    _node_count: u64,
    _active_nodes: Vec<Checksum256>,
}

impl IncrementalMerkle {

    fn new(node_count: u64, active_nodes: Vec<Checksum256>) -> Self {
        IncrementalMerkle {
            _node_count: node_count,
            _active_nodes: active_nodes,
        }
    }

    // Add a node to the incremental tree and recalculate the _active_nodes so they
    // are prepared for the next append.
    //
    // The algorithm for this is to start at the new node and retreat through the tree
    // for any node that is the concatenation of a fully-realized node and a partially
    // realized node we must record the value of the fully-realized node in the new
    // _active_nodes so that the next append can fetch it.   Fully realized nodes and
    // Fully implied nodes do not have an effect on the _active_nodes.
    //
    // For convention _AND_ to allow appends when the _node_count is a power-of-2, the
    // current root of the incremental tree is always appended to the end of the new
    // _active_nodes.
    //
    // In practice, this can be done iteratively by recording any "left" value that
    // is to be combined with an implied node.
    //
    // If the appended node is a "left" node in its pair, it will immediately push itself
    // into the new active nodes list.
    //
    // If the new node is a "right" node it will begin collapsing upward in the tree,
    // reading and discarding the "left" node data from the old active nodes list, until
    // it becomes a "left" node.  It must then push the "top" of its current collapsed
    // sub-tree into the new active nodes list.
    //
    // Once any value has been added to the new active nodes, all remaining "left" nodes
    // should be present in the order they are needed in the previous active nodes as an
    // artifact of the previous append.  As they are read from the old active nodes, they
    // will need to be copied in to the new active nodes list as they are still needed
    // for future appends.
    //
    // As a result, if an append collapses the entire tree while always being the "right"
    // node, the new list of active nodes will be empty and by definition the tree contains
    // a power-of-2 number of nodes.
    //
    // Regardless of the contents of the new active nodes list, the top "collapsed" value
    // is appended.  If this tree is _not_ a power-of-2 number of nodes, this node will
    // not be used in the next append but still serves as a conventional place to access
    // the root of the current tree. If this _is_ a power-of-2 number of nodes, this node
    // will be needed during then collapse phase of the next append so, it serves double
    // duty as a legitimate active node and the conventional storage location of the root.
    //
    //
    // @param digest - the node to add
    // @return - the new root
    pub fn append(&mut self, digest: Checksum256) -> crate::Result<Checksum256> {
        let mut partial = false;
        let max_depth = calculate_max_depth(self._node_count + 1);
        let mut current_depth = max_depth - 1;
        let mut index = self._node_count;
        let mut top = digest;
        let mut active_iter = self._active_nodes.iter();
        let mut updated_active_nodes: Vec<Checksum256> = Vec::with_capacity(max_depth);

        while current_depth > 0 {
            if (index & 0x1) == 0 {
                // we are collapsing from a "left" value and an implied "right" creating a partial node

                // we only need to append this node if it is fully-realized and by definition
                // if we have encountered a partial node during collapse this cannot be
                // fully-realized
                if !partial {
                    updated_active_nodes.push(top);
                }

                // calculate the partially realized node value by implying the "right" value is identical
                // to the "left" value
                top = Checksum256::hash(make_canonical_pair(&top, &top))?;
                partial = true;
            } else {
                // we are collapsing from a "right" value and an fully-realized "left"

                // pull a "left" value from the previous active nodes
                let left_value = active_iter.next().ok_or(crate::Error::IncreMerkleError)?;

                // if the "right" value is a partial node we will need to copy the "left" as future appends still need it
                // otherwise, it can be dropped from the set of active nodes as we are collapsing a fully-realized node
                if partial {
                    updated_active_nodes.push(*left_value);
                }

                // calculate the node
                top = Checksum256::hash(make_canonical_pair(left_value, &top))?;
            }

            // move up a level in the tree
            current_depth -= 1;
            index = index >> 1;
        }

        // append the top of the collapsed tree (aka the root of the merkle)
        updated_active_nodes.push(top);

        // store the new active_nodes
        self._active_nodes = updated_active_nodes;

        // update the node count
        self._node_count += 1;

        return Ok(self._active_nodes[self._active_nodes.len() - 1]);
    }

    // return the current root of the incremental merkle
    pub fn get_root(&self) -> Checksum256 {
        if self._node_count > 0 {
            return self._active_nodes[self._active_nodes.len() - 1];
        } else {
            return Default::default();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_merkle(incr_merkle: &mut IncrementalMerkle, leaf: &str, root: &str, node_count: u64) {
        let ret = incr_merkle.append(leaf.into());
        assert!(ret.is_ok());
        assert_eq!(incr_merkle.get_root(), root.into());
        assert_eq!(ret.unwrap(), root.into());
        assert_eq!(incr_merkle._node_count, node_count);
    }

    #[test]
    fn incre_merkle_next_power_of_2_should_work() {
        assert_eq!(next_power_of_2(1), 1);
        assert_eq!(next_power_of_2(4), 4);
        assert_eq!(next_power_of_2(30), 32);
        assert_eq!(next_power_of_2(100), 128);
        assert_eq!(next_power_of_2(1000), 1024);
    }

    #[test]
    fn incre_merkle_clz_power_2_should_work() {
        assert_eq!(clz_power_2(2), 1);
        assert_eq!(clz_power_2(4), 2);
        assert_eq!(clz_power_2(8), 3);
        assert_eq!(clz_power_2(1024), 10);
    }

    #[test]
    fn incre_merkle_calculate_max_depth_should_work() {
        assert_eq!(calculate_max_depth(2), 2);
        assert_eq!(calculate_max_depth(4), 3);
        assert_eq!(calculate_max_depth(8), 4);
        assert_eq!(calculate_max_depth(1024), 11);
    }

    #[test]
    fn incre_merkle_append_default_should_work() {
        let mut im = IncrementalMerkle::default();
        assert_merkle(&mut im, "00000001bcf2f448225d099685f14da76803028926af04d2607eafcf609c265c", "00000001bcf2f448225d099685f14da76803028926af04d2607eafcf609c265c", 1);
        assert_merkle(&mut im, "000000025e8d459281b82824f627a65c99ce538c0b7b5077640810834ad29550", "9e417ac4c5add67b79e648e58b64b0a18f04292421944ada38372bdf5e5caa44", 2);
        assert_merkle(&mut im, "00000003a833fc0d4af3d9eefa4d84b6ec6557686a38b198512dad758d95c83a", "47fcbe87388fdb0f535051d79d25960550317dca4727984a3e63262058bf6949", 3);
        assert_merkle(&mut im, "0000000464986ad3b90e9d19cb1f82cc39acc6c2029dc21d6d6b2c94cd7eda7b", "3aad2785f1ae142b4828a8d2682932aeca978c2a75a4ee62089b4bc2ae25c65c", 4);
        assert_merkle(&mut im, "00000005d5dc1fddb0e7dc5282f43279d02df642a14d2e901201486c3647da02", "1a12c87cb6dd0a01f2971c8044b7a5ff81e974ec2db50eb4050d8a8ccbca4097", 5);
        assert_merkle(&mut im, "000000062026cf3652bfc3c9e6314b0d3249213e7d7245f674d482b113eb642e", "1328f9dd84a29ef56a2cd5db6f5451971fd48a53a41666e01a90c95de9af54b0", 6);
        assert_merkle(&mut im, "00000007c5219f27464f8358f21bf191256b957a6fc9e75ac95f86cf01ae5948", "3c28744e912e5189a38c106fdc28e8403aab5dfaaaca7652cd328db3c0702a7b", 7);
        assert_merkle(&mut im, "00000008cc389079fc4e18bc5a641bd9329854c7cd893dad169246810138a635", "e20e5ecbf7aab7e13f501de1c4d53099b7bd18b8ccf643152cdacda62ff0bf23", 8);
        assert_merkle(&mut im, "00000009fce66aff37a769a36547855918ad2f9eb736c8f2db9f87d16dc68664", "b84465f3ee5303ad5cb0eea0fc774f3a6f94e981cc9107f66cad449262876a42", 9);
        assert_merkle(&mut im, "0000000ab821097409af57c5e26316f0651f538a060501d11806b7d8d865d03d", "3a31e9490733a26739908caebe542a75763bbaaac8fc685ffd19397cf86f3449", 10);
    }

    #[test]
    fn incre_merkle_append_new_should_work() {
        let node_count: u64 = 1000;
        let active_nodes: Vec<Checksum256> = vec![
            "a07c9d677e8b78fbcf6a39006e32f7267136720852970c98fa0a17906db904ed".into(),
            "d42448facefac0365ffa3352f5a884de5a7767617613ed603c0bf8ead497746b".into(),
            "ce13bb37d1e3a0447a8dedac09912d6dd580ef6f4287eace80b793faa6f1350b".into(),
            "71dc5f0f981fd938aa8a0645c7a4489a1566f6d17a1690bb8c1b0abee059fc1d".into(),
            "e4241c10dba7472bad31cccf204f8cc8a9c30ea409edc1f0bc96b8235107f252".into(),
            "eaeae53ae87fda5f525b6e133a40d3cebf9c2ea00c097212fbdda66f24a947f6".into(),
            "054fc7abfa49a458adb7974587b395e1d7b703470049bcbd869b7877bc5a2c95".into(),
        ];
        let mut im = IncrementalMerkle::new(node_count, active_nodes);
        assert_merkle(&mut im, "000003e9fa0a5fe4c0dc18d84df95f07327da95ab1560ad11c8e22518b667eb2", "573e977a68dea3e50e563d4899eec42526ea1b10202b1ad241532af117993257", 1001);

        let node_count: u64 = 1100;
        let active_nodes: Vec<Checksum256> = vec![
            "ce111f8110efac9d153fa0663bb19b5d42c065b873ef19c9cf566524b727f7ef".into(),
            "7f581fe7b91d8075e1d502a5d0cf71bca496181ee541d06d9c76acc12fd2bbf8".into(),
            "cc9b4d01d9c9072b3d86fd8e42a6490b907b244988107b1fd7729c1c8b358ab7".into(),
            "dd6738c0b5ae245b1695817a24025834cfb932910608b1ae3fa1385ba693c1d4".into(),
            "b07eb9ef49d7be02bfddedeb0915139d6a4390335eb029b5f74b36c767a6ab8e".into(),
        ];
        let mut im = IncrementalMerkle::new(node_count, active_nodes);
        assert_merkle(&mut im, "0000044dd8410fd57e1f7f3c39a943018dd08e091236c8eb3c273da17261878a", "6dc059f54d0f6d025cded68766b9a4a57bb3c4c8413398e7267f43ab9f57b8a2", 1101);

        let node_count: u64 = 1200;
        let active_nodes: Vec<Checksum256> = vec![
            "8719a9d241d81ff50cb16de0e6b1b2da7e056031205524306a775ae027a3c75d".into(),
            "e4717866b581101dc5f5c2462c2742cd36c7ab13ca06c6391881308ddcc77839".into(),
            "e7e4747a6e3e2422f9a5d9588fdba0ebf08e9b34b19d3dc2361298ca99b184a9".into(),
            "dd6738c0b5ae245b1695817a24025834cfb932910608b1ae3fa1385ba693c1d4".into(),
            "b912082d9576279c662435eac4241274d7c643e3e770d91f17abf8ddb419e542".into(),
        ];
        let mut im = IncrementalMerkle::new(node_count, active_nodes);
        assert_merkle(&mut im, "000004b1053d966afe4d6da6f6469e33bb227268b42eddc5a9d52b5e3517b8dd", "18d7e8fbc7c971283cf21964054b2ee29f74e57633d1c2499fb8765d25057d4a", 1201);
        assert_merkle(&mut im, "000004b2790d7cc9b1eaa01389bd7dd37878c3d834607b4a8dc806172b2d7c96", "6bbfbde16d994fe2bbc110e804b544a78999f0c7a8db9ed17075f35429522a8f", 1202);
        assert_merkle(&mut im, "000004b391178dca57423d2a6b7d50bce57c96da226ce601e9415a11116ed577", "c8202455a65bbb3d03af70792a4874ef064adb4205aa70c9d439f65fdbb8f879", 1203);
        assert_merkle(&mut im, "000004b41b410ae71e5b2d27ed28304a342103448e8543284cf72de034ddaa1d", "43a5440ae97b9d5058364ca0657f61378d4ed0e76d84ca424714b45a9ac6b83e", 1204);
        assert_merkle(&mut im, "000004b54d7733aecae89fe83bd0fdebc32c9f73046b86272f1a55da565261be", "f4a05bb8b4402a8a486720bbfb3c9db3a599ab52268eb846e1d404b2bf63323a", 1205);
    }
}
