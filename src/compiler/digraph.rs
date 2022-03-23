use crate::compiler::lexanalysis::Token;

pub enum Direction {
    To,
    From,
    Nil
}

pub struct DiGraph {
    root: Option<Node>
}

impl DiGraph {
    pub fn new() -> Self {
        DiGraph { 
            root: None 
        }
    }

    pub fn transverse_closure() {
        unimplemented!()
    }

    pub fn output_as_matrix() {
        unimplemented!()
    }
}

pub struct Node {
    content: Option<Token>,
    path: Option<Vec<NodePath>> 
}

impl Node {
    pub fn new() -> Self {
        Node { 
            content: None, 
            path: None
        }
    }
}

pub struct NodePath {
    dir: Direction,
    node: Box<Node>
}

impl NodePath {
    pub fn new() -> Self {
        NodePath { 
            dir: Direction::Nil,
            node: Box::new(Node::new())
        }
    }
}
