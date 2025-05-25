#![allow(dead_code)]

use crate::common::{CellType, Direction, Position};

pub enum VehicleType {
    Bus,
    Tram,
}

impl VehicleType {
    pub fn size_in_cells(&self) -> i32 {
        match self {
            VehicleType::Bus => 2,
            VehicleType::Tram => 4,
        }
    }

    pub fn cell_type(&self) -> CellType {
        match self {
            VehicleType::Bus => CellType::Bus,
            VehicleType::Tram => CellType::Tram,
        }
    }
}

pub struct Vehicle {
    position: Position,
    v_type: VehicleType,
    direction: Direction,
}

impl Vehicle {
    pub fn new(position: Position, v_type: VehicleType, direction: Direction) -> Self {
        Self {
            position,
            v_type,
            direction,
        }
    }
}
