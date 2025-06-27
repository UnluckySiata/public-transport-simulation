use chrono::NaiveTime;
use serde_json;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::io::{BufRead, BufReader};
use std::{error::Error, ffi::OsString, fs::File, process};
use crate::{get_args, structs};


pub fn creating_bus_stop_data() -> Result<(), Box<dyn Error>> {
    let (
        bus_stops_file_path,
        coordinates_file_path,
        stop_times_file_path,
        trips_file_path,
        routes_file_path,
    ) = get_args()?;
    let bus_stops = reading_bus_stops_csv(bus_stops_file_path, coordinates_file_path);

    let mut bus_schedules: Vec<structs::BusStopSchedule> = bus_stops?
        .iter()
        .map(|bus_stop: &structs::BusStop| structs::BusStopSchedule {
            stop_id: bus_stop.stop_id.clone(),
            stop_name: bus_stop.stop_name.clone(),
            schedules: None,
        })
        .collect();

    let stop_ids: HashSet<String> = bus_schedules
        .iter()
        .map(|schedule: &structs::BusStopSchedule| schedule.stop_id.clone())
        .collect();
    let stop_times = reading_stop_times_csv(stop_times_file_path, stop_ids)?;

    let trip_ids: HashSet<String> = stop_times
        .iter()
        .map(|stop_time: &structs::StopTime| stop_time.trip_id.clone())
        .collect();
    let trips: HashMap<String, structs::Trip> = reading_trips_csv(trips_file_path, trip_ids)?;

    let route_ids: HashSet<String> = trips
        .iter()
        .map(|trip: (&String, &structs::Trip)| trip.1.route_id.clone())
        .collect();
    let routes: HashMap<String, structs::Route> = reading_routes_csv(routes_file_path, route_ids)?;

    for bus_schedule in &mut bus_schedules {
        let current_stop_times: Vec<&structs::StopTime> = stop_times
            .iter()
            .filter(|stop_time: &&structs::StopTime| stop_time.stop_id == bus_schedule.stop_id)
            .collect();

        let mut schedule_map: HashMap<String, structs::Schedule> = HashMap::new();

        for stop_time in current_stop_times {
            if let Some(trip) = trips.get(&stop_time.trip_id) {
                if let Some(route) = routes.get(&trip.route_id) {
                    // println!("{}: found {} ", trip.route_id, route.route_id);
                    let line_name: String = route.route_short_name.clone();

                    let entry = schedule_map.entry(line_name.clone()).or_insert(structs::Schedule {
                        line: line_name,
                        arrivals: BTreeSet::new(),
                    });

                    match NaiveTime::parse_from_str(&stop_time.arrival_time, "%H:%M:%S") {
                        Ok(parsed_arrival) => {
                            entry.arrivals.insert(parsed_arrival);
                        }
                        Err(e) => {
                            // eprintln!("Błąd parsowania arrival_time: {} ({}).", stop_time.arrival_time, e);
                        }
                    }
                }
            }
        }
        bus_schedule.schedules = Some(schedule_map);
    }
    let file = File::create("../outputs/bus_schedule_output.json")?;
    serde_json::to_writer_pretty(file, &bus_schedules)?;

    let mut stop_id_to_stop_name: HashMap<String, String> = HashMap::new();
    for schedule in &bus_schedules {
        stop_id_to_stop_name.insert(schedule.stop_id.clone(), schedule.stop_name.clone());
    }

    let mut stop_id_to_lines: HashMap<String, HashSet<String>> = HashMap::new();
    for schedule in &bus_schedules {
        if let Some(schedules) = &schedule.schedules {
            for (line_name, _) in schedules {
                stop_id_to_lines
                    .entry(schedule.stop_id.clone())
                    .or_default()
                    .insert(line_name.clone());
            }
        }
    }

    let mut trip_stop_sequences: HashMap<String, Vec<String>> = HashMap::new();
    for stop_time in &stop_times {
        trip_stop_sequences
            .entry(stop_time.trip_id.clone())
            .or_default()
            .push(stop_time.stop_id.clone());
    }

    let mut stop_neighbours: HashMap<String, HashSet<String>> = HashMap::new();
    for stop_list in trip_stop_sequences.values() {
        for window in stop_list.windows(2) {
            if let [a, b] = &window[..] {
                stop_neighbours
                    .entry(a.clone())
                    .or_default()
                    .insert(b.clone());
                stop_neighbours
                    .entry(b.clone())
                    .or_default()
                    .insert(a.clone());
            }
        }
    }

    let mut enriched_stops: Vec<structs::EnrichedStop> = Vec::new();

    for stop in &bus_schedules {
        let neighbours = stop_neighbours
            .get(&stop.stop_id)
            .cloned()
            .unwrap_or_default();

        let mut neighbour_stops = Vec::new();
        let mut reachable_stops = Vec::new();

        for neighbour_id in neighbours {
            if let Some(name) = stop_id_to_stop_name.get(&neighbour_id) {
                let lines_set = stop_id_to_lines
                    .get(&neighbour_id)
                    .cloned()
                    .unwrap_or_default();

                let lines: Vec<String> = lines_set.iter().cloned().collect();

                let mut transport_types = HashSet::new();

                for line in &lines {
                    if let Some(route) = routes.values().find(|r| &r.route_short_name == line) {
                        transport_types.insert(transport_type_from_route_type(route.route_type));
                    }
                }

                let transport_type = if transport_types.len() == 1 {
                    transport_types.into_iter().next().unwrap().to_string()
                } else if transport_types.is_empty() {
                    "unknown".to_string()
                } else {
                    "mixed".to_string()
                };

                neighbour_stops.push(structs::NeighbourStop {
                    stop_id: neighbour_id.clone(),
                    stop_name: name.clone(),
                    lines,
                    transport_type,
                });

                reachable_stops.push(name.clone());
            }
        }

        enriched_stops.push(structs::EnrichedStop {
            stop_name: stop.stop_name.clone(),
            neighbour_stops,
            reachable_stops,
        });
    }

    let file = File::create("../outputs/enriched_stops.json")?;
    serde_json::to_writer_pretty(file, &enriched_stops)?;
    Ok(())
}

fn reading_buses_schedule_json(buses_schedule_file_path: String) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(buses_schedule_file_path)?;
    let mut vehicles_paths: HashMap<String, Vec<structs::BusStopMinimal>> = HashMap::new();

    let json: Vec<structs::BusStopSchedule> = serde_json::from_str(&data)?;
    for bus_stop in json {
        for schedule_map in bus_stop.schedules {
            for (line_name, _) in schedule_map {
                let entry = vehicles_paths
                    .entry(line_name.clone())
                    .or_insert(Vec::new());
                let bus_stop_minimal: structs::BusStopMinimal = structs::BusStopMinimal {
                    stop_name: bus_stop.stop_name.clone(),
                    stop_id: bus_stop.stop_id.clone(),
                };
                entry.push(bus_stop_minimal);
            }
        }
    }
    println!("vehicle paths: {:?}", &vehicles_paths);

    Ok(())
}

fn reading_routes_csv(
    routes_file_path: OsString,
    route_ids: HashSet<String>,
) -> Result<HashMap<String, structs::Route>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(routes_file_path)?;
    let mut routes: HashMap<String, structs::Route> = HashMap::new();

    for result in rdr.deserialize() {
        let route: structs::Route = result?;
        if route_ids.contains(&route.route_id) {
            routes.insert(route.route_id.clone(), route);
        }
    }

    Ok(routes)
}

fn reading_trips_csv(
    trips_file_path: OsString,
    trip_ids: HashSet<String>,
) -> Result<HashMap<String, structs::Trip>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(trips_file_path)?;
    let mut trips: HashMap<String, structs::Trip> = HashMap::new();

    for result in rdr.deserialize() {
        let trip: structs::Trip = result?;
        if trip_ids.contains(&trip.trip_id) {
            trips.insert(trip.trip_id.clone(), trip);
        }
    }

    Ok(trips)
}

fn reading_bus_stops_csv(
    bus_stops_file_path: OsString,
    coordinates_file_path: OsString,
) -> Result<Vec<structs::BusStop>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(bus_stops_file_path)?;
    let (start_lat, start_lon, end_lat, end_lon) = reading_coordinates(coordinates_file_path)?;

    let mut bus_stops: Vec<structs::BusStop> = Vec::new();
    for result in rdr.deserialize() {
        let bus_stop: structs::BusStop = result?;

        if bus_stop.stop_lat <= start_lat
            && bus_stop.stop_lat >= end_lat
            && bus_stop.stop_lon >= start_lon
            && bus_stop.stop_lon <= end_lon
        {
            bus_stops.push(bus_stop);
        }
    }

    Ok(bus_stops)
}

fn reading_stop_times_csv(
    stop_times_file_path: OsString,
    stop_ids: HashSet<String>,
) -> Result<Vec<structs::StopTime>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(stop_times_file_path)?;
    let mut stop_times: Vec<structs::StopTime> = Vec::new();

    for result in rdr.deserialize() {
        let stop_time: structs::StopTime = result?;
        if stop_ids.contains(&stop_time.stop_id) {
            stop_times.push(stop_time);
        }
    }

    Ok(stop_times)
}

fn reading_coordinates(file_path: OsString) -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = BufReader::new(file);
    let mut line = String::new();

    rdr.read_line(&mut line)?;
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.len() < 4 {
        return Err("expected 4 values".into());
    }

    Ok((
        parts[0].parse()?,
        parts[1].parse()?,
        parts[2].parse()?,
        parts[3].parse()?,
    ))
}


fn transport_type_from_route_type(route_type: u8) -> &'static str {
    match route_type {
        0 => "tram",
        1 => "metro",
        2 => "rail",
        3 => "bus",
        4 => "ferry",
        5 => "cable_car",
        6 => "gondola",
        7 => "monorail",
        _ => "unknown",
    }
}
