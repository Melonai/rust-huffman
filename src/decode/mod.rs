use std::collections::BTreeMap;
use std::convert::TryInto;
use crate::bst::{Node, BranchNode, LeafNode};

pub fn decode(mut buffer: Vec<u8>) {
    let (map_length, payload) = buffer.split_first_mut().unwrap();
    let indices = read_map(*map_length, payload);
    let tree_head = indices_to_tree(indices);
}

fn read_map(map_length: u8, mut payload: &mut [u8]) -> BTreeMap<u8, Vec<bool>> {
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
            index.push(index_bytes[byte_index as usize] & (1 << bit_index) != 0);
        }

        indices.insert(character, index);
        payload = new_payload;
    }
    indices
}

fn indices_to_tree(indices: BTreeMap<u8, Vec<bool>>) -> Node {
    let mut tree_head = BranchNode::new(None, None);
    for (value, path) in indices {
        let mut current = tree_head.unwrap_branch_mut();
        let leaf_right = path.last().unwrap();
        for choice_right in path.iter().take(path.len()-1) {
            let next_node = current.choose_branch_mut(*choice_right);
            if let Some(node) = next_node {
                current = node.unwrap_branch_mut();
            } else {
                current = next_node.get_or_insert(Box::new(BranchNode::new(None, None))).unwrap_branch_mut();
            }
        }
        current.choose_branch_mut(*leaf_right).replace(Box::new(LeafNode::new(value, 0)));
    }
    tree_head
}