#![allow(dead_code)]
mod data;
mod graph;
mod gui;
mod line;
mod map;
mod node;
mod sim_consts;
mod time;

use crate::{data::create_graph, graph::Graph, time::Time};

pub struct Simulation {
    pub accumulator: f64,
    pub debug_accumulator: f64,
    pub speed: f64,
    pub paused: bool,
    pub done: bool,
    pub time: Time,
    pub graph: Graph,
}

fn run_simulation() -> eframe::Result<()> {
    let graph = create_graph();

    let sim = Simulation {
        accumulator: 0.0,
        debug_accumulator: 0.0,
        speed: 1.0,
        paused: false,
        done: false,
        time: Time::new(4, 30, 0),
        graph,
    };

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Public transport simulation",
        native_options,
        Box::new(move |_| Ok(Box::new(sim))),
    )
}

fn main() {
    let _ = run_simulation();
}
