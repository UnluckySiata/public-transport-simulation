use std::collections::HashMap;

use crate::{
    graph::Graph,
    line::{Line, LineState, RoadSide},
    node::{Node, NodeVariant, TransportVariant, Vehicle},
    sim_consts,
};

pub fn mock_empty() -> Graph {
    let nodes = Vec::new();
    let vehicles = HashMap::new();
    Graph::new(nodes, vehicles)
}

pub fn mock_one_line() -> Graph {
    let nodes = Vec::from([
        Node::new(
            TransportVariant::Bus,
            NodeVariant::Regular,
            true,
            false,
            sim_consts::JAM_PROBABILITY,
        ),
        Node::new(
            TransportVariant::Bus,
            NodeVariant::Regular,
            false,
            false,
            sim_consts::JAM_PROBABILITY,
        ),
        Node::new(
            TransportVariant::Bus,
            NodeVariant::Regular,
            false,
            false,
            sim_consts::JAM_PROBABILITY,
        ),
        Node::new(
            TransportVariant::Bus,
            NodeVariant::Regular,
            false,
            false,
            sim_consts::JAM_PROBABILITY,
        ),
        Node::new(
            TransportVariant::Bus,
            NodeVariant::Regular,
            false,
            false,
            sim_consts::JAM_PROBABILITY,
        ),
    ]);

    let line = Line::new(1, vec![0, 1, 2, 3, 4]);
    let line_state = LineState::new(1, false, line.into());
    let vehicles = HashMap::from([((0, RoadSide::Left), Vehicle::new(line_state))]);

    Graph::new(nodes, vehicles)
}
