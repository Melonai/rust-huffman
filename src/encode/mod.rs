use crate::bst::{BranchNode, LeafNode, Node};
use std::collections::{BinaryHeap, HashMap, VecDeque, BTreeMap};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;
use std::iter::FromIterator;

pub fn encode(file_path: &str) -> Result<(), std::io::Error> {
    let start = Instant::now();

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    let mut byte_count_table = HashMap::<u8, usize>::new();

    file.read_to_end(&mut buffer)?;
    for byte in &buffer {
        *byte_count_table.entry(*byte).or_insert(0) += 1;
    }
    let byte_count_table = BTreeMap::from_iter(byte_count_table);
    log_time(start, "Counted byte occurrence");

    let mut node_heap: BinaryHeap<Node> = byte_count_table
        .iter()
        .map(|n| LeafNode::new(*n.0, *n.1))
        .collect();

    log_time(start, "Made leaf nodes");

    while node_heap.len() != 1 {
        let first_node = node_heap.pop().unwrap();
        let second_node = node_heap.pop().unwrap();
        let new_node = BranchNode::new(first_node, second_node);
        node_heap.push(new_node);
    }

    log_time(start, "Made binary tree");

    let mut queue = VecDeque::<Node>::new();
    queue.push_front(node_heap.pop().unwrap());

    let mut indices = BTreeMap::new();

    while let Some(node) = queue.pop_front() {
        match node {
            Node::Branch(branch) => {
                let BranchNode {
                    mut left,
                    mut right,
                    path,
                    ..
                } = branch;
                let path = path.unwrap_or_default();
                left.set_path(path.clone(), false);
                right.set_path(path, true);
                queue.push_front(*left);
                queue.push_front(*right);
            }
            Node::Leaf(leaf) => {
                indices.insert(leaf.symbol, leaf.path.unwrap());
            }
        }
    }

    log_time(start, "Got indices from binary tree");

    let mut output: Vec<bool> = vec![];
    for byte in &buffer {
        output.append(&mut indices.get(byte).unwrap().clone());
    }

    log_time(start, "Created output for file");
    let output = booleans_to_u8(output);
    log_time(start, "Converted output to bytes");

    let mut map: Vec<u8> = vec![indices.len() as u8];
    for (value, boolean_index) in indices.into_iter() {
        let bit_amount = boolean_index.len() as u8;
        let index = booleans_to_u8(boolean_index);
        map.push(value);
        map.push(bit_amount);
        map.extend(index);
    }

    log_time(start, "Converted indices to usable map");

    let mut output_file = File::create(format!(
        "{}.huff",
        Path::new(&file_path).file_stem().unwrap().to_str().unwrap()
    ))?;

    output_file.write_all(map.as_slice())?;
    output_file.write_all(output.as_slice())?;

    log_time(start, "Finished");

    Ok(())
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
