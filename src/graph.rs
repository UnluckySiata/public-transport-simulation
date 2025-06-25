use std::collections::HashMap;

use crate::node::{Node, NodeVariant, Vehicle};

#[derive(Debug)]
pub struct Graph {
    n: usize,
    vehicles: HashMap<usize, Vehicle>,
    initial_nodes: Vec<Node>,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, vehicles: HashMap<usize, Vehicle>) -> Self {
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
            if !vehicle.progress(elapsed_time) {
                continue;
            }

            let dest_node_index = vehicle.next_node_index();
            let to_node = &self.nodes[dest_node_index];

            if to_node.can_move_into() {
                vehicle.to_move = true;
            }
        }

        // update state
        let vehicle_progression: Vec<(usize, usize)> = self
            .vehicles
            .iter_mut()
            .filter_map(|(source_node_index, vehicle)| {
                if vehicle.to_move {
                    let dest_node_index = vehicle.next_node_index();
                    Some((*source_node_index, dest_node_index))
                } else {
                    None
                }
            })
            .collect();

        for (source_node_index, dest_node_index) in vehicle_progression {
            if let Some(mut vehicle) = self.vehicles.remove(&source_node_index) {
                self.nodes[source_node_index].occupied = false;
                self.nodes[dest_node_index].occupied = true;

                vehicle.to_move = false;

                self.vehicles.insert(dest_node_index, vehicle);
            }
        }
    }

    // TODO: improve or just create gui
    pub fn debug_repr(&self) -> String {
        let repr = (0..self.n)
            .map(|i| {
                if self.vehicles.contains_key(&i) {
                    format!("{}", self.vehicles[&i].line_number())
                } else {
                    "o".to_owned()
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        repr
    }
}
