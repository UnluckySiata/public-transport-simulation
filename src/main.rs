#![allow(dead_code)]
mod graph;
mod line;
mod map;
mod mock;
mod time;
mod node;
mod sim_consts;

use std::time::Instant;

use mock::mock_one_line;
use time::Time;

fn simulation_loop() {
    let mut accumulator: f64 = 0.0;
    let mut debug_accumulator: f64 = 0.0;
    let mut previous = Instant::now();

    let mut graph = mock_one_line();

    let time_step = sim_consts::SIM_SPEED_MULTIPLIER * sim_consts::FIXED_DT;
    let sim_time = Time::new(4, 30, 0);

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
            graph.simulation_iter(time_step, &sim_time);
            // println!("ft: {frame_time}");
        }

        if debug_accumulator >= sim_consts::DEBUG_PRINT_TIME {
            debug_accumulator = 0.0;
            println!("Graph state: {g}", g = graph.debug_repr());
        }
    }
}

fn main() {
    simulation_loop();
}
