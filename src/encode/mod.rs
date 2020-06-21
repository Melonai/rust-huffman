use crate::bst::{BranchNode, LeafNode, Node};
use crate::utils::{booleans_to_u8, log_time};
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};
use std::iter::FromIterator;
use std::time::Instant;

pub fn encode(buffer: &[u8]) -> Result<Vec<u8>, ()> {
    let start = Instant::now();

    let byte_count_table = count_occurrence(buffer);
    log_time(start, "Counted byte occurrence");

    let tree_head = create_bst(&byte_count_table);
    log_time(start, "Made binary tree");

    let indices = unwrap_bst_to_indices(tree_head);
    log_time(start, "Got indices from binary tree");
    let payload = create_payload(buffer, &indices);
    log_time(start, "Created payload for file");

    let map = create_map(indices);
    log_time(start, "Finished encoding");

    Ok([map.as_slice(), payload.as_slice()].concat())
}

pub fn count_occurrence(buffer: &[u8]) -> BTreeMap<u8, usize> {
    let mut count = HashMap::<u8, usize>::new();

    for byte in buffer {
        *count.entry(*byte).or_insert(0) += 1;
    }
    BTreeMap::from_iter(count)
}

pub fn create_bst(count: &BTreeMap<u8, usize>) -> Node {
    let mut node_heap: BinaryHeap<Node> = count.iter().map(|n| LeafNode::new(*n.0, *n.1)).collect();
    while node_heap.len() != 1 {
        let first_node = node_heap.pop().unwrap();
        let second_node = node_heap.pop().unwrap();
        let new_node = BranchNode::new(Some(first_node), Some(second_node));
        node_heap.push(new_node);
    }
    node_heap.pop().unwrap()
}

pub fn unwrap_bst_to_indices(tree_head: Node) -> BTreeMap<u8, Vec<bool>> {
    let mut queue = VecDeque::<Node>::new();
    queue.push_front(tree_head);
    let mut indices = BTreeMap::new();

    while let Some(node) = queue.pop_front() {
        match node {
            Node::Branch(branch) => {
                let BranchNode {
                    left, right, path, ..
                } = branch;
                let path = path.unwrap_or_default();
                if let Some(mut node) = left {
                    node.set_path(path.clone(), false);
                    queue.push_front(*node);
                }
                if let Some(mut node) = right {
                    node.set_path(path.clone(), true);
                    queue.push_front(*node);
                }
            }
            Node::Leaf(leaf) => {
                indices.insert(leaf.symbol, leaf.path.unwrap());
            }
        }
    }
    indices
}

fn create_payload(buffer: &[u8], indices: &BTreeMap<u8, Vec<bool>>) -> Vec<u8> {
    let mut payload: Vec<u8> = vec![0];
    let mut current_bit = 0;
    for byte in buffer {
        let to_write = indices.get(byte).unwrap().clone();
        for bit in to_write {
            if current_bit == 8 {
                current_bit = 0;
                payload.push(0);
            }
            let current = payload.last_mut().unwrap();
            *current |= (bit as u8) << (7 - current_bit);
            current_bit += 1;
        }
    }
    payload.insert(0, current_bit);
    payload
}

pub fn create_map(indices: BTreeMap<u8, Vec<bool>>) -> Vec<u8> {
    let mut map: Vec<u8> = vec![indices.len() as u8];
    for (value, boolean_index) in indices.into_iter() {
        let bit_amount = boolean_index.len() as u8;
        let index = booleans_to_u8(boolean_index);
        map.push(value);
        map.push(bit_amount);
        map.extend(index);
    }
    map
}
