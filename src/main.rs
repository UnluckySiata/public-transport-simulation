#![allow(dead_code)]
mod graph;
mod line;
mod map;
mod mock;
mod node;
mod sim_consts;

mod data_aggregation;
mod structs;
mod exporting_to_json;

use std::{env, process};
use std::error::Error;
use std::ffi::OsString;
use exporting_to_json::creating_bus_stop_data;
use std::time::Instant;

fn get_args() -> Result<(OsString, OsString, OsString, OsString, OsString), Box<dyn Error>> {
    let args: Vec<OsString> = env::args_os().skip(1).collect();

    if args.len() < 5 {
        return Err(From::from(format!(
            "expected 5 arguments, got {}",
            args.len()
        )));
    }

    Ok((
        args[0].clone(),
        args[1].clone(),
        args[2].clone(),
        args[3].clone(),
        args[4].clone(),
    ))
}

use mock::mock_one_line;

fn simulation_loop() {
    let mut accumulator: f64 = 0.0;
    let mut debug_accumulator: f64 = 0.0;
    let mut previous = Instant::now();

    let mut graph = mock_one_line();

    let time_step = sim_consts::SIM_SPEED_MULTIPLIER * sim_consts::FIXED_DT;

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
            graph.simulation_iter(time_step);
            // println!("ft: {frame_time}");
        }

        if debug_accumulator >= sim_consts::DEBUG_PRINT_TIME {
            debug_accumulator = 0.0;
            println!("Graph state: {g}", g = graph.debug_repr());
        }
    }
}

fn main() {
    // simulation_loop();
    let args = match get_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Błąd: {}", e);
            process::exit(1);
        }
    };

    let (
        bus_stops_file_path,
        coordinates_file_path,
        stop_times_file_path,
        trips_file_path,
        routes_file_path,
    ) = args;

    if let Err(err) = creating_bus_stop_data(
        bus_stops_file_path,
        coordinates_file_path,
        stop_times_file_path,
        trips_file_path,
        routes_file_path,
    ) {
        eprintln!("Błąd: {}", err);
        process::exit(1);
    }
}
