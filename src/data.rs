use std::{collections::HashMap, fs::File};

use egui::Pos2;
use serde::Deserialize;

use crate::{
    graph::Graph,
    node::{Node, NodeVariant, TransportVariant},
    sim_consts,
};

#[derive(Deserialize)]
pub struct ParsedNode {
    pub id: u32,
    pub label: String,
    pub variant: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize)]
pub struct ParsedNodes {
    pub nodes: Vec<ParsedNode>,
}

impl ParsedNodes {
    pub fn to_regular_nodes(self) -> Vec<Node> {
        self.nodes
            .into_iter()
            .map(|n| {
                let transport_variant = if n.variant == "bus" {
                    TransportVariant::Bus
                } else {
                    TransportVariant::Tram
                };

                let node_variant = if n.label == "" {
                    NodeVariant::Regular
                } else {
                    NodeVariant::Stop
                };

                Node::new(
                    transport_variant,
                    node_variant,
                    false,
                    false,
                    n.label,
                    Pos2 { x: n.x, y: n.y },
                    sim_consts::JAM_PROBABILITY,
                )
            })
            .collect()
    }
}

pub fn create_graph() -> Graph {
    let f = File::open("data/nodes.json").unwrap();
    let parsed: ParsedNodes = serde_json::from_reader(f).unwrap();

    let vehicles = HashMap::new();

    Graph::new(parsed.to_regular_nodes(), vehicles)
}
