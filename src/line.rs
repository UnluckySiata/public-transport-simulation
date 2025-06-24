use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct LineState {
    pub number: u32,
    pub curr_node_index: usize,
    reversed: bool,
    line: Rc<Line>,
}

impl LineState {
    pub fn new(number: u32, reversed: bool, line: Rc<Line>) -> Self {
        let curr_node_index = if reversed { line.n_stops - 1 } else { 0 };

        Self {
            number,
            curr_node_index,
            reversed,
            line,
        }
    }
    pub fn progress(&mut self) {
        let (next_node_index, reversed) = self.line.progress(self.curr_node_index, self.reversed);

        self.curr_node_index = next_node_index;
        self.reversed = reversed;
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
    fn progress(&self, curr_node_index: usize, reversed: bool) -> (usize, bool) {
        match (curr_node_index, reversed) {
            (0, true) => (1, false),
            (_, true) => (curr_node_index - 1, true),
            (val, false) if val == self.n_stops - 1 => (val - 1, true),
            (_, false) => (curr_node_index + 1, false),
        }
    }
}
