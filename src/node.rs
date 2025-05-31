#![allow(dead_code)]

use crate::map::MapPosition;

#[derive(Clone, Copy)]
pub struct Line {
    pub next_node_index: usize
}

#[derive(Clone, Copy)]
pub enum NodeVariant {
    Road,
    Rail,
    BusStop,
    TramStop,
    Bus(Line),
    Tram(Line),
}

#[derive(Clone, Copy)]
pub struct Node {
    pub variant: NodeVariant,
    position: MapPosition,
}
