#![allow(dead_code)]
use crate::Checksum256;

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

struct IncrementalMerkle {
    _node_count: u64,
    _active_nodes: Vec<Checksum256>,
}

impl IncrementalMerkle {

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
    pub fn append(&mut self, digest: Checksum256) -> Checksum256 {
        let mut partial = false;
        let max_depth = calculate_max_depth(self._node_count + 1);
        let mut current_depth = max_depth - 1;
        let mut index = self._node_count;
        let mut top = digest;
        let mut active_iter = self._active_nodes.iter();
        let mut updated_active_nodes: Vec<Checksum256> = Vec::with_capacity(max_depth);

        while current_depth > 0 {
            if !((index & 0x1) != 0) {
                // we are collapsing from a "left" value and an implied "right" creating a partial node

                // we only need to append this node if it is fully-realized and by definition
                // if we have encountered a partial node during collapse this cannot be
                // fully-realized
                if !partial {
                    updated_active_nodes.push(top);
                }

                // calculate the partially realized node value by implying the "right" value is identical
                // to the "left" value
                top = Checksum256::hash((top, top)).unwrap();
                partial = true;
            } else {
                // we are collapsing from a "right" value and an fully-realized "left"

                // pull a "left" value from the previous active nodes
                let left_value = active_iter.next().unwrap();

                // if the "right" value is a partial node we will need to copy the "left" as future appends still need it
                // otherwise, it can be dropped from the set of active nodes as we are collapsing a fully-realized node
                if partial {
                    updated_active_nodes.push(*left_value);
                }

                // calculate the node
                top = Checksum256::hash((*left_value, top)).unwrap();
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

        return *self._active_nodes.last().unwrap();
    }

    // return the current root of the incremental merkle
    //
    // @return
    pub fn get_root(&self) -> Checksum256 {
        if self._node_count > 0 {
            return *self._active_nodes.last().unwrap();
        } else {
            return Default::default();
        }
    }
}
