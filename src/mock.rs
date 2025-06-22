use std::collections::BTreeMap;

use crate::{
    graph::Graph,
    node::{LineState, Node, NodeVariant, TransportVariant, Vehicle},
};

pub fn mock_one_line() -> Graph {
    let nodes = Vec::from([
        Node::new(TransportVariant::Bus, NodeVariant::Regular, true),
        Node::new(TransportVariant::Bus, NodeVariant::Regular, false),
        Node::new(TransportVariant::Bus, NodeVariant::Regular, false),
        Node::new(TransportVariant::Bus, NodeVariant::Regular, false),
        Node::new(TransportVariant::Bus, NodeVariant::Regular, false),
    ]);

    let line  = LineState {number: 1, next_node_index: 1};
    let vehicles = BTreeMap::from([(0, Vehicle::new(line))]);

    Graph::new(nodes, vehicles)
}
