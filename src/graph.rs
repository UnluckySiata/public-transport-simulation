#![allow(dead_code)]

use std::collections::BTreeMap;

use crate::node::{Node, NodeVariant, Vehicle};

#[derive(Debug)]
pub struct Graph {
    n: usize,
    vehicles: BTreeMap<usize, Vehicle>,
    initial_nodes: Vec<Node>,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, vehicles: BTreeMap<usize, Vehicle>) -> Self {
        let n = nodes.len();
        Self {
            n,
            vehicles,
            initial_nodes: nodes.clone(),
            nodes,
        }
    }

    pub fn simulation_iter(&mut self, elapsed_time: f64) {
        for node in self.nodes.iter_mut() {
            if let NodeVariant::TrafficLights(traffic_lights) = &mut node.node_variant {
                traffic_lights.iter_and_change(elapsed_time);
            }
            node.update_state(elapsed_time);
        }

        // update vehicles for current state
        for (_, vehicle) in self.vehicles.iter_mut() {
            let dest_node_index = vehicle.line.next_node_index;
            let to_node = &self.nodes[dest_node_index];

            if to_node.can_move_into() {
                vehicle.to_move = true;
            }
        }

        // update state
        for (source_node_index, vehicle) in self.vehicles.iter_mut() {
            if !vehicle.to_move {
                continue;
            }
            let dest_node_index = vehicle.line.next_node_index;

            self.nodes[*source_node_index].occupied = false;
            self.nodes[dest_node_index].occupied = true;

            vehicle.to_move = false;
        }
    }
}
