use std::io;
use std::io::{prelude, Read};
use std::fs::File;
use std::collections::HashMap;

mod bst;

pub fn encode(file_path: &str) -> Result<bool, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    let mut byte_count_table = HashMap::<u8, usize>::new();

    file.read_to_end(&mut buffer)?;
    for byte in &buffer {
        *byte_count_table.entry(*byte).or_insert(0) += 1;
    }

    let mut node_queue: Vec<_> = byte_count_table.iter().collect();

    return Ok(true);
}

fn insert_into_queue(queue: Vec<BST>, node: ) {

}

enum NodeKind {
    Parent,
    Leaf
}

struct Node {
    left: Link,
    right: Link,
    kind: NodeKind
}

type Link = Option<Box<Node>>;