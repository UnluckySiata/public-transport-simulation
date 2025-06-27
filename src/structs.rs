use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Deserialize, Serialize)]
pub struct NeighbourStop {
    pub(crate) stop_id: String,
    pub(crate) stop_name: String,
    pub(crate) lines: Vec<String>,
    pub(crate) transport_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BusStop {
    pub(crate) stop_id: String,
    pub(crate) stop_code: String,
    pub(crate) stop_name: String,
    pub(crate) stop_desc: String,
    pub(crate) stop_lat: f64,
    pub(crate) stop_lon: f64,
    pub(crate) stop_type: Option<String>,
    pub(crate) neighbour_stops: Option<Vec<NeighbourStop>>,
    pub(crate) reachable_stops: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BusStopSchedule {
    pub(crate) stop_id: String,
    pub(crate) stop_name: String,
    pub(crate) schedules: Option<HashMap<String, Schedule>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Schedule {
    pub(crate) line: String,
    pub(crate) arrivals: BTreeSet<NaiveTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StopTime {
    pub(crate) trip_id: String,
    pub(crate) arrival_time: String,
    pub(crate) departure_time: String,
    pub(crate) stop_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Route {
    pub(crate) route_id: String,
    pub(crate) route_short_name: String,
    pub(crate) route_type: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Trip {
    pub(crate) trip_id: String,
    pub(crate) route_id: String,
}

#[derive(Debug, Serialize)]
pub struct EnrichedStop {
    pub(crate) stop_name: String,
    pub(crate) neighbour_stops: Vec<NeighbourStop>,
    pub(crate) reachable_stops: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BusStopMinimal {
    pub(crate) stop_name: String,
    pub(crate) stop_id: String,
}
