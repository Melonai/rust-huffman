use std::io;
use std::io::{prelude, Read};
use std::fs::File;
use std::collections::{HashMap, BinaryHeap};

use crate::bst::{Node, BranchNode, LeafNode};

pub fn encode(file_path: &str) -> Result<bool, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    let mut byte_count_table = HashMap::<u8, usize>::new();

    file.read_to_end(&mut buffer)?;
    for byte in &buffer {
        *byte_count_table.entry(*byte).or_insert(0) += 1;
    }

    let mut node_queue: BinaryHeap<Node> = byte_count_table
        .iter()
        .map(|n| LeafNode::new(*n.0, *n.1))
        .collect();

    while node_queue.len() != 1 {
        let first_node = node_queue.pop().unwrap();
        let second_node = node_queue.pop().unwrap();
        let new_node = BranchNode::new(first_node, second_node);
        node_queue.push(new_node);
    }

    return Ok(true);
}