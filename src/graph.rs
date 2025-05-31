#![allow(dead_code)]

use std::mem::swap;

use crate::node::{NodeVariant, Node};

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

    pub fn simulation_iter(&mut self) {
        for (index, node) in self.nodes.iter_mut().enumerate() {
            match node.variant {
                v @ NodeVariant::Bus(line) => {
                    self.node_progression[line.next_node_index].variant = v;
                },
                v @ NodeVariant::Tram(line) => {
                    self.node_progression[line.next_node_index].variant = v;
                },
                // todo
                v => {
                    self.node_progression[index].variant = v;
                }
            }
        }
        swap(&mut self.nodes, &mut self.node_progression);
    }
}
