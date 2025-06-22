#![allow(dead_code)]

use crate::sim_consts;

#[derive(Clone, Copy, Debug)]
pub struct LineState {
    pub number: u32,
    pub next_node_index: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Vehicle {
    pub line: LineState,
    pub to_move: bool,
}

impl Vehicle {
    pub fn new(line: LineState) -> Self {
        Self {
            line,
            to_move: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TransportVariant {
    Bus,
    Tram,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LightsVariant {
    Red,
    Green,
}

#[derive(Clone, Copy, Debug)]
pub struct TrafficLights {
    pub variant: LightsVariant,
    time_until_change: f64,
}

impl TrafficLights {
    pub fn new(variant: LightsVariant) -> Self {
        Self {
            variant,
            time_until_change: sim_consts::LIGHT_CYCLE_SECONDS,
        }
    }
    
    pub fn iter_and_change(&mut self, elapsed_time: f64) -> bool {
        if self.time_until_change <= 0.0 {
            self.time_until_change = sim_consts::LIGHT_CYCLE_SECONDS;

            return true;
        }

        self.time_until_change -= elapsed_time;

        false
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NodeVariant {
    Regular,
    TrafficLights(TrafficLights),
    Stop,
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub transport_variant: TransportVariant,
    pub node_variant: NodeVariant,
    pub occupied: bool,
    jammed: bool,
    remaining_jam_time: f64,
}

impl Node {
    pub fn new(
        transport_variant: TransportVariant,
        node_variant: NodeVariant,
        occupied: bool,
    ) -> Self {
        Self {
            transport_variant,
            node_variant,
            occupied,
            jammed: false,
            remaining_jam_time: 0.0,
        }
    }
    pub fn update_state(&mut self, elapsed_time: f64) {
        if self.jammed {
            self.remaining_jam_time -= elapsed_time;

            if self.remaining_jam_time <= 0.0 {
                self.jammed = false;
                self.remaining_jam_time = 0.0;
            }
            return;
        }

        if !rand::random_bool(sim_consts::JAM_PROBABILITY) {
            return;
        }

        let jam_time =
            rand::random_range(0.0..sim_consts::JAM_MAX_TIME - sim_consts::JAM_BASE_TIME);

        if jam_time < sim_consts::JAM_BASE_TIME {
            return;
        }

        self.jammed = true;
        self.remaining_jam_time = jam_time + sim_consts::JAM_BASE_TIME;
    }
    pub fn can_move_into(&self) -> bool {
        let no_traffic_restriction = match self.node_variant {
            NodeVariant::Regular => true,
            NodeVariant::Stop => true,

            NodeVariant::TrafficLights(traffic_lights) => {
                traffic_lights.variant == LightsVariant::Green
            }
        };

        no_traffic_restriction && !self.jammed && !self.occupied
    }
}
