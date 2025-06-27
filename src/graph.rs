use std::collections::HashMap;

use crate::{
    line::RoadSide,
    node::{Node, NodeVariant, Vehicle},
};

#[derive(Debug)]
pub struct Graph {
    n: usize,
    vehicles: HashMap<(usize, RoadSide), Vehicle>,
    initial_nodes: Vec<Node>,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, vehicles: HashMap<(usize, RoadSide), Vehicle>) -> Self {
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

            if to_node.can_move_into(vehicle.road_side()) {
                vehicle.to_move = true;
            }
        }

        // update state
        let vehicle_progression: Vec<(usize, usize, RoadSide, RoadSide)> = self
            .vehicles
            .iter_mut()
            .filter_map(|((source_node_index, previous_road_side), vehicle)| {
                if vehicle.to_move {
                    let dest_node_index = vehicle.next_node_index();
                    let road_side = vehicle.road_side();
                    Some((
                        *source_node_index,
                        dest_node_index,
                        *previous_road_side,
                        road_side,
                    ))
                } else {
                    None
                }
            })
            .collect();

        for (source_node_index, dest_node_index, previous_road_side, road_side) in
            vehicle_progression
        {
            if let Some(mut vehicle) = self
                .vehicles
                .remove(&(source_node_index, previous_road_side))
            {
                match previous_road_side {
                    RoadSide::Left => self.nodes[source_node_index].occupied_left = false,
                    RoadSide::Right => self.nodes[source_node_index].occupied_right = false,
                };

                match road_side {
                    RoadSide::Left => self.nodes[dest_node_index].occupied_left = true,
                    RoadSide::Right => self.nodes[dest_node_index].occupied_right = true,
                };

                vehicle.to_move = false;

                self.vehicles.insert((dest_node_index, road_side), vehicle);
            }
        }
    }

    // TODO: improve or just create gui
    pub fn debug_repr(&self) -> String {
        let repr = (0..self.n)
            .map(|i| {
                let lk = &(i, RoadSide::Left);
                let rk = &(i, RoadSide::Right);

                let left = if self.vehicles.contains_key(lk) {
                    format!("{}", self.vehicles[lk].line_number())
                } else {
                    "o".to_owned()
                };

                let right = if self.vehicles.contains_key(rk) {
                    format!("{}", self.vehicles[rk].line_number())
                } else {
                    "o".to_owned()
                };

                format!("{left}|{right}")
            })
            .collect::<Vec<String>>()
            .join(" ");

        repr
    }
}
