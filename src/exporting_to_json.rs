use crate::structs;
use crate::structs::{BusStop, NeighbourStop};
use chrono::NaiveTime;
use serde_json;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::io::{BufRead, BufReader};
use std::{error::Error, ffi::OsString, fs::File};
use serde::{Deserialize, Serialize};

pub fn creating_bus_stop_data(
    bus_stops_file_path: OsString,
    coordinates_file_path: OsString,
    stop_times_file_path: OsString,
    trips_file_path: OsString,
    routes_file_path: OsString,
) -> Result<(), Box<dyn Error>> {
    let bus_stops = reading_bus_stops_csv(bus_stops_file_path, coordinates_file_path)?;
    let bus_schedules = create_initial_bus_schedules(&bus_stops)?;
    let (stop_times, trips, routes) = read_schedule_dependencies(
        &bus_schedules,
        stop_times_file_path,
        trips_file_path,
        routes_file_path,
    )?;

    let enriched_schedules = enrich_schedules(bus_schedules, &stop_times, &trips, &routes)?;
    save_bus_schedule(&enriched_schedules)?;

    let enriched_stops = enrich_bus_stops(bus_stops, &enriched_schedules, &stop_times, &routes);
    save_enriched_stops(&enriched_stops)?;

    Ok(())
}

fn enrich_bus_stops(
    mut bus_stops: HashMap<String, BusStop>,
    schedules: &Vec<structs::BusStopSchedule>,
    stop_times: &Vec<structs::StopTime>,
    routes: &HashMap<String, structs::Route>,
) -> Vec<BusStop> {
    let stop_id_to_name: HashMap<_, _> = schedules
        .iter()
        .map(|s| (s.stop_id.clone(), s.stop_name.clone()))
        .collect();

    // zbiera wszystkie linie dla danego przystanku, patrzy na schedule danego bus_stopu i robi mapę
    // gdzie kluczem jest bus_id, a wartością lista lini zatrzymujących się przy danym przystanku
    let stop_id_to_lines: HashMap<String, HashSet<String>> = schedules
        .iter()
        .filter_map(|s| {
            s.schedules
                .as_ref()
                .map(|schedules| (s.stop_id.clone(), schedules.keys().cloned().collect()))
        })
        .collect();

    // mapa przystanków (bus_id) jakich dany autobus odwiedza podczas przejazdy (tripu)
    // stop_times ma listę nieposortowanych przystanków które odwiedza,
    // mogę zaagregować dla każdego trip_id wszystkie przystanki w odpowiedniej kolejności po stop_sequence
    let mut trip_stop_sequences: HashMap<String, Vec<(u16, String)>> = HashMap::new();
    for stop_time in stop_times {
        trip_stop_sequences
            .entry(stop_time.trip_id.clone())
            .or_default()
            .push((stop_time.stop_sequence.clone(), stop_time.stop_id.clone()));
    }

    for trip in trip_stop_sequences.values_mut() {
        trip.sort_by_key(|(seq, _)| *seq);
    }

    let mut stop_neighbours: HashMap<String, HashSet<String>> = HashMap::new();
    for seq in trip_stop_sequences.values() {
        for window in seq.windows(2) {
            if let [a, b] = &window[..] {
                stop_neighbours
                    .entry(a.1.clone())
                    .or_default()
                    .insert(b.1.clone());
                stop_neighbours
                    .entry(b.1.clone())
                    .or_default()
                    .insert(a.1.clone());
            }
        }
    }

    for stop in schedules {
        if let Some(bus_stop) = bus_stops.get_mut(&stop.stop_id) {
            let neighbours = stop_neighbours
                .get(&stop.stop_id)
                .cloned()
                .unwrap_or_default();

            let mut neighbour_stops = Vec::new();
            let mut reachable_stops = Vec::new();

            for neighbour_id in neighbours {
                if let Some(name) = stop_id_to_name.get(&neighbour_id) {
                    let lines = stop_id_to_lines
                        .get(&neighbour_id)
                        .cloned()
                        .unwrap_or_default()
                        .into_iter()
                        .collect::<Vec<_>>();

                    let transport_types: HashSet<_> = lines
                        .iter()
                        .filter_map(|line| {
                            routes
                                .values()
                                .find(|r| &r.route_short_name == line)
                                .map(|r| transport_type_from_route_type(r.route_type))
                        })
                        .collect();

                    let transport_type = match transport_types.len() {
                        1 => transport_types.into_iter().next().unwrap(),
                        0 => "unknown",
                        _ => "mixed",
                    };

                    neighbour_stops.push(structs::NeighbourStop {
                        stop_id: neighbour_id.clone(),
                        stop_name: name.clone(),
                        lines,
                        transport_type: transport_type.parse().unwrap(),
                    });

                    reachable_stops.push(name.clone());
                }
            }

            bus_stop.neighbour_stops = Option::from(neighbour_stops);
            bus_stop.reachable_stops = Option::from(reachable_stops);
        }
    }

    bus_stops.into_values().collect()
}

fn create_initial_bus_schedules(
    mut bus_stops: &HashMap<String, BusStop>,
) -> Result<Vec<structs::BusStopSchedule>, Box<dyn Error>> {
    let schedules = bus_stops
        .values()
        .map(|bus_stop| structs::BusStopSchedule {
            stop_id: bus_stop.stop_id.clone(),
            stop_name: bus_stop.stop_name.clone(),
            schedules: None,
        })
        .collect();
    Ok(schedules)
}

fn read_schedule_dependencies(
    bus_schedules: &Vec<structs::BusStopSchedule>,
    stop_times_path: OsString,
    trips_path: OsString,
    routes_path: OsString,
) -> Result<
    (
        Vec<structs::StopTime>,
        HashMap<String, structs::Trip>,
        HashMap<String, structs::Route>,
    ),
    Box<dyn Error>,
> {
    let stop_ids: HashSet<String> = bus_schedules.iter().map(|s| s.stop_id.clone()).collect();
    let stop_times = reading_stop_times_csv(stop_times_path, stop_ids)?;

    let trip_ids: HashSet<String> = stop_times.iter().map(|s| s.trip_id.clone()).collect();
    let trips = reading_trips_csv(trips_path, trip_ids)?;

    let route_ids: HashSet<String> = trips.values().map(|t| t.route_id.clone()).collect();
    let routes = reading_routes_csv(routes_path, route_ids)?;

    Ok((stop_times, trips, routes))
}

fn enrich_schedules(
    mut schedules: Vec<structs::BusStopSchedule>,
    stop_times: &Vec<structs::StopTime>,
    trips: &HashMap<String, structs::Trip>,
    routes: &HashMap<String, structs::Route>,
) -> Result<Vec<structs::BusStopSchedule>, Box<dyn Error>> {
    for schedule in &mut schedules {
        let times: Vec<_> = stop_times
            .iter()
            .filter(|t| t.stop_id == schedule.stop_id)
            .collect();

        let mut schedule_map: HashMap<String, structs::Schedule> = HashMap::new();

        for stop_time in times {
            if let Some(trip) = trips.get(&stop_time.trip_id) {
                if let Some(route) = routes.get(&trip.route_id) {
                    let line = route.route_short_name.clone();
                    let entry = schedule_map
                        .entry(line.clone())
                        .or_insert(structs::Schedule {
                            line,
                            arrivals: BTreeSet::new(),
                        });
                    if let Ok(time) = NaiveTime::parse_from_str(&stop_time.arrival_time, "%H:%M:%S")
                    {
                        entry.arrivals.insert(time);
                    }
                }
            }
        }

        schedule.schedules = Some(schedule_map);
    }
    Ok(schedules)
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
) -> Result<HashMap<String, BusStop>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(bus_stops_file_path)?;
    let (start_lat, start_lon, end_lat, end_lon) = reading_coordinates(coordinates_file_path)?;

    let mut bus_stops: HashMap<String, BusStop> = HashMap::new();
    for result in rdr.deserialize() {
        let mut bus_stop: BusStop = result?;

        if bus_stop.stop_lat <= start_lat
            && bus_stop.stop_lat >= end_lat
            && bus_stop.stop_lon >= start_lon
            && bus_stop.stop_lon <= end_lon
        {
            let (x, y) = scale_coordinates_to_gui(
                bus_stop.stop_lat,
                bus_stop.stop_lon,
                start_lat,
                start_lon,
                end_lat,
                end_lon,
                1280.0,
                720.0,
            );

            bus_stop.x = Some(x);
            bus_stop.y = Some(y);

            bus_stops.insert(bus_stop.stop_id.clone(), bus_stop);
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

fn save_enriched_stops(stops: &Vec<BusStop>) -> Result<(), Box<dyn Error>> {
    let file = File::create("outputs/enriched_stops.json")?;
    serde_json::to_writer_pretty(file, stops)?;
    Ok(())
}

fn save_bus_schedule(schedules: &Vec<structs::BusStopSchedule>) -> Result<(), Box<dyn Error>> {
    let file = File::create("outputs/bus_schedule_output.json")?;
    serde_json::to_writer_pretty(file, schedules)?;
    Ok(())
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

fn scale_coordinates_to_gui(
    lat: f64,
    lon: f64,
    start_lat: f64,
    start_lon: f64,
    end_lat: f64,
    end_lon: f64,
    width: f64,
    height: f64,
) -> (f64, f64) {
    let x = ((lon - start_lon) / (end_lon - start_lon)) * width;
    let y = ((start_lat - lat) / (start_lat - end_lat)) * height;
    (x, y)
}

