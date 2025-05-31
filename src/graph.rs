#![allow(dead_code)]

use crate::node::Node;

struct Graph {
    n: usize,
    node_progression: Vec<Node>,
    initial_nodes: Vec<Node>,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>) -> Self {
        let n = nodes.len();
        Self {
            n,
            node_progression: Vec::with_capacity(n),
            initial_nodes: nodes.clone(),
            nodes,
        }
    }
}
