use std::collections::HashMap;

use egui::Pos2;

use crate::{
    line::RoadSide, map::{MapNode, Visuals}, node::{Node, NodeVariant, Vehicle}, time::Time
};

#[derive(Debug)]
pub struct Graph {
    n: usize,
    vehicles: HashMap<(usize, RoadSide), Vehicle>,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, vehicles: HashMap<(usize, RoadSide), Vehicle>) -> Self {
        let n = nodes.len();
        Self { n, vehicles, nodes }
    }

    pub fn simulation_iter(&mut self, elapsed_time: f64, _sim_time: &Time) {
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

    pub fn generate_visuals(&self) -> Visuals {
        let mut map_nodes = Vec::with_capacity(self.n);

        for (i, node) in self.nodes.iter().enumerate() {
            let kl = &(i, RoadSide::Left);
            let kr = &(i, RoadSide::Right);

            let vehicle_line_left = if self.vehicles.contains_key(kl) {
                self.vehicles[kl].line_number()
            } else {
                0
            };
            let vehicle_line_right = if self.vehicles.contains_key(kr) {
                self.vehicles[kr].line_number()
            } else {
                0
            };

            let pos = Pos2 { x: 400.0 + (i as f32) * 80.0, y: 200.0 };

            let map_node = MapNode::new(pos, node.jammed, node.transport_variant, vehicle_line_left, vehicle_line_right);
            map_nodes.push(map_node);
        }

        Visuals { nodes: map_nodes }
    }
}
