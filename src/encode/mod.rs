use std::io;
use std::io::{prelude, Read};
use std::fs::File;
use std::collections::{HashMap, BinaryHeap, VecDeque};

use crate::bst::{Node, BranchNode, LeafNode};

pub fn encode(file_path: &str) -> Result<HashMap<u8, String>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    let mut byte_count_table = HashMap::<u8, usize>::new();

    file.read_to_end(&mut buffer)?;
    for byte in &buffer {
        *byte_count_table.entry(*byte).or_insert(0) += 1;
    }

    let mut node_heap: BinaryHeap<Node> = byte_count_table
        .iter()
        .map(|n| LeafNode::new(*n.0, *n.1))
        .collect();

    while node_heap.len() != 1 {
        let first_node = node_heap.pop().unwrap();
        let second_node = node_heap.pop().unwrap();
        let new_node = BranchNode::new(first_node, second_node);
        node_heap.push(new_node);
    }

    let mut queue = VecDeque::<Node>::new();
    queue.push_front(node_heap.pop().unwrap());

    let mut indices = HashMap::new();

    while let Some(node) = queue.pop_front() {
        match node {
            Node::Branch(branch) => {
                let BranchNode { mut left, mut right, path, ..} = branch;
                let path = path.unwrap_or(Vec::new());
                left.set_path(path.clone(), true);
                right.set_path(path, false);
                queue.push_front(*left);
                queue.push_front(*right);
            }
            Node::Leaf(leaf) => {
                let path_name = leaf.path
                    .unwrap()
                    .iter()
                    .fold(String::new(), |a, &l| a + (l as u8).to_string().as_ref());
                indices.insert(leaf.symbol, path_name);
            }
        }
    }

    return Ok(indices);
}