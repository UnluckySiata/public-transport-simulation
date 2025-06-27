mod data_aggregation;
mod structs;
mod exporting_to_json;

use std::{env, process};
use std::error::Error;
use std::ffi::OsString;
use std::time::Instant;
use exporting_to_json::creating_bus_stop_data;

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

fn simulation_loop() {
    const FIXED_DT: f32 = 1.0 / 2.0;
    const MAX_FRAME_TIME: f32 = 0.25;

    let mut accumulator: f32 = 0.0;
    let mut previous = Instant::now();

    loop {
        let now = Instant::now();
        let mut frame_time = (now - previous).as_secs_f32();
        previous = now;

        if frame_time > MAX_FRAME_TIME {
            frame_time = MAX_FRAME_TIME;
        }
        accumulator += frame_time;

        while accumulator >= FIXED_DT {
            accumulator -= FIXED_DT;
            // println!("ft: {frame_time}");
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
    }}
