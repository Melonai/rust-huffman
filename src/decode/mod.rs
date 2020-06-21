use crate::bst::{BranchNode, LeafNode, Node};
use crate::utils::{bit_at, log_time};
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::time::Instant;

pub fn decode(mut buffer: Vec<u8>) -> Result<Vec<u8>, ()> {
    let start = Instant::now();
    let (map_length, payload) = buffer.split_first_mut().unwrap();
    let (indices, payload) = read_map_and_separate(*map_length, payload);
    log_time(start, "Read map");

    let (payload_pad, payload) = payload.split_first_mut().unwrap();
    let tree_head = indices_to_tree(&indices);
    log_time(start, "Created tree from indices");

    let mut decoded = Vec::new();
    let mut current = &tree_head;
    let mut bit = *payload_pad;
    for byte in payload {
        while bit < 8 {
            if let Node::Branch(branch) = current {
                current = branch.choose_branch(bit_at(*byte, bit)).as_ref().unwrap();
                if let Node::Leaf(leaf) = current {
                    decoded.push(leaf.symbol);
                    current = &tree_head;
                }
                bit += 1;
            }
        }
        bit = 0;
    }
    log_time(start, "Finished");
    Ok(decoded)
}

pub fn read_map_and_separate(
    map_length: u8,
    mut payload: &mut [u8],
) -> (BTreeMap<u8, Vec<bool>>, &mut [u8]) {
    let mut indices = BTreeMap::new();
    for _ in 0..map_length {
        let (info, payload_no_info) = payload.split_at_mut(2);
        let &mut [character, bit_length]: &mut [u8; 2] = info.try_into().unwrap();
        let byte_length = (bit_length as f32 / 8.0).ceil() as usize;

        let (index_bytes, new_payload) = payload_no_info.split_at_mut(byte_length);

        let mut index = Vec::new();
        for bit in 0..bit_length {
            let byte_index = bit / 8;
            let bit_index = (bit_length - 1 - byte_index * 8).min(7) - bit % 8;
            index.push(bit_at(index_bytes[byte_index as usize], 7 - bit_index));
        }

        indices.insert(character, index);
        payload = new_payload;
    }
    (indices, payload)
}

pub fn indices_to_tree(indices: &BTreeMap<u8, Vec<bool>>) -> Node {
    let mut tree_head = BranchNode::new(None, None);
    for (value, path) in indices {
        let mut current = tree_head.unwrap_branch_mut();
        let leaf_right = path.last().unwrap();
        for choice_right in path.iter().take(path.len() - 1) {
            let next_node = current.choose_branch_mut(*choice_right);
            if let Some(node) = next_node {
                current = node.unwrap_branch_mut();
            } else {
                current = next_node
                    .get_or_insert(Box::new(BranchNode::new(None, None)))
                    .unwrap_branch_mut();
            }
        }
        current
            .choose_branch_mut(*leaf_right)
            .replace(Box::new(LeafNode::new(*value, 0)));
    }
    tree_head
}
