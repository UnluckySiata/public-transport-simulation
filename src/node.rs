#![allow(dead_code)]

use crate::map::MapPosition;

#[derive(Clone, Copy)]
struct LineState {
    number: u32,
    next_node_index: usize,
}

#[derive(Clone, Copy)]
struct Bus {
    line: LineState,
}

#[derive(Clone, Copy)]
struct Tram {
    line: LineState,
}

#[derive(Clone, Copy)]
enum NodeVariant {
    Bus(Option<Bus>),
    Tram(Option<Tram>),
}

#[derive(Clone, Copy)]
enum TrafficLights {
    Red,
    Green,
}

#[derive(Clone, Copy)]
pub struct Node {
    variant: NodeVariant,
    traffic_lights: Option<TrafficLights>,
    position: MapPosition,
}
