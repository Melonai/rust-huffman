use crate::bst::{BranchNode, LeafNode, Node};
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};
use std::iter::FromIterator;
use std::time::Instant;

pub fn encode(buffer: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let start = Instant::now();

    let byte_count_table = count_occurrence(buffer);
    log_time(start, "Counted byte occurrence");

    let tree_head = create_bst(&byte_count_table);
    log_time(start, "Made binary tree");

    let indices = unwrap_bst_to_indices(tree_head);
    log_time(start, "Got indices from binary tree");

    let payload = booleans_to_u8(create_payload(buffer, &indices));
    log_time(start, "Created payload for file");

    let map = create_map(indices);
    log_time(start, "Finished encoding");

    Ok([map.as_slice(), payload.as_slice()].concat())
}

fn log_time(start: Instant, message: &str) {
    println!("{}: {}", message, start.elapsed().as_secs_f32());
}

fn booleans_to_u8(booleans: Vec<bool>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for byte in booleans.chunks(8) {
        result.push(byte.iter().fold(0, |acc, b| acc << 1 | (*b as u8)));
    }
    result
}

fn count_occurrence(buffer: &[u8]) -> BTreeMap<u8, usize> {
    let mut count = HashMap::<u8, usize>::new();

    for byte in buffer {
        *count.entry(*byte).or_insert(0) += 1;
    }
    BTreeMap::from_iter(count)
}

fn create_bst(count: &BTreeMap<u8, usize>) -> Node {
    let mut node_heap: BinaryHeap<Node> = count.iter().map(|n| LeafNode::new(*n.0, *n.1)).collect();
    while node_heap.len() != 1 {
        let first_node = node_heap.pop().unwrap();
        let second_node = node_heap.pop().unwrap();
        let new_node = BranchNode::new(Some(first_node), Some(second_node));
        node_heap.push(new_node);
    }
    node_heap.pop().unwrap()
}

fn unwrap_bst_to_indices(tree_head: Node) -> BTreeMap<u8, Vec<bool>> {
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

fn create_payload(buffer: &[u8], indices: &BTreeMap<u8, Vec<bool>>) -> Vec<bool> {
    let mut payload: Vec<bool> = vec![];
    for byte in buffer {
        payload.append(&mut indices.get(byte).unwrap().clone());
    }
    payload
}

fn create_map(indices: BTreeMap<u8, Vec<bool>>) -> Vec<u8> {
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
