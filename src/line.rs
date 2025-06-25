use std::rc::Rc;

use crate::sim_consts;

#[derive(Clone, Debug)]
pub struct LineState {
    pub number: u32,
    pub curr_node_index: usize,
    pub dist_to_next_node: f64,
    reversed: bool,
    line: Rc<Line>,
}

impl LineState {
    pub fn new(number: u32, reversed: bool, line: Rc<Line>) -> Self {
        let curr_node_index = if reversed { line.n_stops - 1 } else { 0 };
        let dist_to_next_node = line.start_distance(reversed);

        Self {
            number,
            curr_node_index,
            dist_to_next_node,
            reversed,
            line,
        }
    }
    pub fn progress(&mut self) {
        let line_update = self.line.progress(self.curr_node_index, self.reversed);

        self.curr_node_index = line_update.curr_node_index;
        self.reversed = line_update.reversed;
        self.dist_to_next_node = line_update.dist_to_next_node;
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    number: u32,
    n_stops: usize,
    stops: Vec<u32>,
    // struct for stops instead of vec or just line schedule
}

impl Line {
    pub fn new(number: u32, stops: Vec<u32>) -> Self {
        Self {
            number,
            n_stops: stops.len(),
            stops,
        }
    }

    fn start_distance(&self, _reversed: bool) -> f64 {
        // placeholder
        sim_consts::METERS_BETWEEN_NODES
    }

    fn dist_between_nodes(&self, _from_node: usize, _to_node: usize) -> f64 {
        // placeholder
        sim_consts::METERS_BETWEEN_NODES
    }

    fn progress(&self, curr_node_index: usize, reversed: bool) -> LineUpdate {
        let (from_index, to_index, reversed) = match (curr_node_index, reversed) {
            (0, true) => (0, 1, false),
            (_, true) => (curr_node_index, curr_node_index - 1, true),
            (val, false) if val == self.n_stops - 1 => (val, val - 1, true),
            (_, false) => (curr_node_index, curr_node_index + 1, false),
        };
        let dist = self.dist_between_nodes(from_index, to_index);

        LineUpdate::new(to_index, reversed, dist)
    }
}

pub struct LineUpdate {
    pub curr_node_index: usize,
    pub reversed: bool,
    pub dist_to_next_node: f64,
}

impl LineUpdate {
    fn new(curr_node_index: usize, reversed: bool, dist_to_next_node: f64) -> Self {
        Self {
            curr_node_index,
            reversed,
            dist_to_next_node,
        }
    }
}
