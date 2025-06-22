#![allow(dead_code)]
mod graph;
mod map;
mod mock;
mod node;
mod sim_consts;

use std::collections::BTreeMap;
use std::time::Instant;

use graph::Graph;

fn simulation_loop() {
    let mut accumulator: f64 = 0.0;
    let mut debug_accumulator: f64 = 0.0;
    let mut previous = Instant::now();

    let nodes = Vec::new();
    let vehicles = BTreeMap::new();

    let mut graph = Graph::new(nodes, vehicles);

    loop {
        let now = Instant::now();
        let mut frame_time = (now - previous).as_secs_f64();
        previous = now;

        if frame_time > sim_consts::MAX_FRAME_TIME {
            frame_time = sim_consts::MAX_FRAME_TIME;
        }
        accumulator += frame_time;
        debug_accumulator += frame_time;

        while accumulator >= sim_consts::FIXED_DT {
            accumulator -= sim_consts::FIXED_DT;
            graph.simulation_iter(frame_time);
            // println!("ft: {frame_time}");
        }

        if debug_accumulator >= sim_consts::DEBUG_PRINT_TIME {
            debug_accumulator = 0.0;
            println!("Graph state: {graph:#?}");
        }
    }
}

fn main() {
    simulation_loop();
}
