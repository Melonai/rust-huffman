use std::cmp::Ordering;

pub enum Node {
    Branch(BranchNode),
    Leaf(LeafNode),
}

impl Node {
    pub fn get_weight(&self) -> usize {
        match self {
            Node::Branch(branch) => branch.weight,
            Node::Leaf(leaf) => leaf.weight,
        }
    }

    pub fn set_path(&mut self, mut new_path: Vec<bool>, right: bool) {
        new_path.push(right);
        match self {
            Node::Branch(branch) => branch.path = Some(new_path),
            Node::Leaf(leaf) => leaf.path = Some(new_path),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.get_weight().cmp(&self.get_weight())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.get_weight() == other.get_weight()
    }
}

pub struct BranchNode {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub path: Option<Vec<bool>>,
    weight: usize,
}

pub struct LeafNode {
    pub symbol: u8,
    pub path: Option<Vec<bool>>,
    weight: usize,
}

impl BranchNode {
    pub fn new(first: Node, second: Node) -> Node {
        let weight = first.get_weight() + second.get_weight();
        let weight_difference = first.get_weight() as isize - second.get_weight() as isize;
        let first_node = Box::new(first);
        let second_node = Box::new(second);
        if weight_difference < 0 {
            Node::Branch(BranchNode {
                left: first_node,
                right: second_node,
                weight,
                path: None,
            })
        } else {
            Node::Branch(BranchNode {
                left: second_node,
                right: first_node,
                weight,
                path: None,
            })
        }
    }
}

impl LeafNode {
    pub fn new(symbol: u8, weight: usize) -> Node {
        Node::Leaf(LeafNode {
            symbol,
            weight,
            path: None,
        })
    }
}
