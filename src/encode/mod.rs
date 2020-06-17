use crate::bst::{BranchNode, LeafNode, Node};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;

pub fn encode(file_path: &str) -> Result<(), std::io::Error> {
    let start = Instant::now();
    let mut current = Instant::now();

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    let mut byte_count_table = HashMap::<u8, usize>::new();

    file.read_to_end(&mut buffer)?;
    for byte in &buffer {
        *byte_count_table.entry(*byte).or_insert(0) += 1;
    }

    current = log_time(current, "Counted byte occurrence");

    let mut node_heap: BinaryHeap<Node> = byte_count_table
        .iter()
        .map(|n| LeafNode::new(*n.0, *n.1))
        .collect();

    current = log_time(current, "Made leaf nodes");

    while node_heap.len() != 1 {
        let first_node = node_heap.pop().unwrap();
        let second_node = node_heap.pop().unwrap();
        let new_node = BranchNode::new(first_node, second_node);
        node_heap.push(new_node);
    }

    current = log_time(current, "Made binary tree");

    let mut queue = VecDeque::<Node>::new();
    queue.push_front(node_heap.pop().unwrap());

    let mut indices = HashMap::new();

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

    current = log_time(current, "Got indices from binary tree");

    let mut output: Vec<bool> = vec![];
    for byte in &buffer {
        output.append(&mut indices.get(byte).unwrap().clone());
    }
    current = log_time(current, "Created output for file");
    let mut output = booleans_to_u8(output);
    current = log_time(current, "Converted output to bytes");

    let mut map: Vec<u8> = vec![];
    for (value, index) in indices.into_iter() {
        map.push(value);
        map.push((index.len() as f32 / 8.0).ceil() as u8);
        map.extend(booleans_to_u8(index));
    }
    current = log_time(current, "Converted indices to usable map");

    let mut output_file = File::create(format!(
        "{}.huff",
        Path::new(&file_path).file_stem().unwrap().to_str().unwrap()
    ))?;

    output_file.write_all(map.as_slice())?;
    output_file.write_all(vec![10, 10, 10, 10].as_slice())?;
    output_file.write_all(output.as_slice())?;

    current = log_time(current, "Wrote to file");
    log_time(start, "Full time");

    Ok(())
}

fn log_time(current: Instant, message: &str) -> Instant {
    println!("{}: {}", message, current.elapsed().as_secs_f32());
    Instant::now()
}

fn booleans_to_u8(booleans: Vec<bool>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for byte in booleans.chunks(8) {
        result.push(byte.iter().fold(0, |acc, b| acc << 1 | (*b as u8)));
    }
    result
}
