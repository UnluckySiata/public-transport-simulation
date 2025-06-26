use crate::line::{LineState, RoadSide};
use crate::sim_consts;

#[derive(Clone, Debug)]
pub struct Vehicle {
    line: LineState,
    curr_dist_traveled: f64,
    pub to_move: bool,
}

impl Vehicle {
    pub fn new(line: LineState) -> Self {
        Self {
            line,
            curr_dist_traveled: 0.0,
            to_move: false,
        }
    }
    pub fn progress(&mut self, elapsed_time: f64) -> bool {
        self.curr_dist_traveled += sim_consts::VEHICLE_SPEED_MS * elapsed_time;

        if self.curr_dist_traveled >= self.line.dist_to_next_node {
            self.curr_dist_traveled = 0.0;
            self.line.progress();
            true
        } else {
            false
        }
    }
    pub fn next_node_index(&mut self) -> usize {
        self.line.curr_node_index
    }
    pub fn line_number(&self) -> u32 {
        self.line.number
    }
    pub fn road_side(&self) -> RoadSide {
        self.line.current_road_side()
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
    light_cycle_time: f64,
    time_until_change: f64,
}

impl TrafficLights {
    pub fn new(variant: LightsVariant, time_until_change: f64) -> Self {
        Self {
            variant,
            light_cycle_time: time_until_change,
            time_until_change,
        }
    }

    pub fn iter_and_change(&mut self, elapsed_time: f64) -> bool {
        if self.time_until_change <= 0.0 {
            self.time_until_change = self.light_cycle_time;

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
    pub occupied_left: bool,
    pub occupied_right: bool,
    jammed: bool,
    jam_probability: f64,
    remaining_jam_time: f64,
}

impl Node {
    pub fn new(
        transport_variant: TransportVariant,
        node_variant: NodeVariant,
        occupied_left: bool,
        occupied_right: bool,
        jam_probability: f64,
    ) -> Self {
        Self {
            transport_variant,
            node_variant,
            occupied_left,
            occupied_right,
            jammed: false,
            jam_probability,
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

        if !rand::random_bool(self.jam_probability) {
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

    pub fn can_move_into(&self, road_side: RoadSide) -> bool {
        let no_traffic_restriction = match self.node_variant {
            NodeVariant::Regular => true,
            NodeVariant::Stop => true,

            NodeVariant::TrafficLights(traffic_lights) => {
                traffic_lights.variant == LightsVariant::Green
            }
        };

        let occupied = match road_side {
            RoadSide::Left => self.occupied_left,
            RoadSide::Right => self.occupied_right,
        };

        no_traffic_restriction && !self.jammed && !occupied
    }
}
