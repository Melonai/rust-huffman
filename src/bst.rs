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

    pub fn unwrap_branch_mut(&mut self) -> &mut BranchNode {
        if let Node::Branch(branch) = self {
            branch
        } else {
            panic!("Could not unwrap non branch node as branch.")
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
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub path: Option<Vec<bool>>,
    weight: usize,
}

pub struct LeafNode {
    pub symbol: u8,
    pub path: Option<Vec<bool>>,
    weight: usize,
}

impl BranchNode {
    pub fn new(first: Option<Node>, second: Option<Node>) -> Node {
        let first_node = first.map(Box::new);
        let second_node = second.map(Box::new);
        let (weight, change_order) = BranchNode::calculate_weights(&first_node, &second_node);
        if !change_order {
            Node::Branch(BranchNode {
                left: first_node,
                right: second_node,
                weight: weight as usize,
                path: None,
            })
        } else {
            Node::Branch(BranchNode {
                left: second_node,
                right: first_node,
                weight: weight as usize,
                path: None,
            })
        }
    }

    pub fn choose_branch_mut(&mut self, right: bool) -> &mut Option<Box<Node>> {
        if right {
            &mut self.right
        } else {
            &mut self.left
        }
    }

    pub fn choose_branch(&self, right: bool) -> &Option<Box<Node>> {
        if right {
            &self.right
        } else {
            &self.left
        }
    }

    fn calculate_weights(first: &Option<Box<Node>>, second: &Option<Box<Node>>) -> (isize, bool) {
        let first_weight = first.as_ref().map_or(0, |n| n.get_weight() as isize);
        let second_weight = second.as_ref().map_or(0, |n| n.get_weight() as isize);
        (
            first_weight + second_weight,
            first_weight - second_weight > 0,
        )
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
