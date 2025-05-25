#![allow(dead_code)]

use std::collections::BTreeMap;

use crate::{
    common::{CellType, Direction, Position},
    vehicle::{Vehicle, VehicleType},
};

pub struct Grid {
    n: usize,
    grid: Vec<Vec<CellType>>,
    vehicle_map: BTreeMap<Position, Vehicle>,
}

impl Grid {
    pub fn new(n: usize) -> Self {
        let grid = vec![vec![CellType::Empty; n]; n];
        let vehicle_map = BTreeMap::new();

        Self {
            n,
            grid,
            vehicle_map,
        }
    }

    pub fn create_vehicle(
        &mut self,
        position: Position,
        v_type: VehicleType,
        direction: Direction,
    ) {
        let x = position.x;
        let y = position.y;

        let (x_delta, y_delta) = direction.position_delta();
        let cell_type = v_type.cell_type();

        for i in 0..v_type.size_in_cells() {
            self.grid[(x + i * x_delta) as usize][(y + i * y_delta) as usize] = cell_type;
        }

        self.vehicle_map
            .insert(position, Vehicle::new(position, v_type, direction));
    }
}
