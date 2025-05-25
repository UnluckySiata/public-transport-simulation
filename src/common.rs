#![allow(dead_code)]

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn position_delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Empty,
    RouteNorth,
    RouteSouth,
    RouteEast,
    RouteWest,
    BusStop,
    TramStop,
    Bus,
    Tram,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
