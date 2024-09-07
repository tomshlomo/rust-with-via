#![allow(dead_code)]
use serde::Deserialize;

mod helper;
pub use helper::read_from_csv;
use helper::{deserialize_time, parse_time};

#[derive(Debug, Deserialize, Clone)]
pub struct Stop {
    stop_id: u32,
    stop_name: String,
    stop_lat: f64,
    stop_lon: f64,
    stop_desc: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StopTime {
    trip_id: String,
    #[serde(deserialize_with = "deserialize_time")]
    departure_time: u32,
    stop_id: u32,
}

pub fn find_stops_within_distance(
    stops: Vec<Stop>,
    target_lat: f64,
    target_lon: f64,
    max_distance_m: u32,
) -> Vec<Stop> {
    todo!();
}

pub fn find_stop_times_within_time(
    stops: Vec<Stop>,
    stops_times: Vec<StopTime>,
    target_time: &str,
    time_window_minutes: u32,
) -> Vec<StopTime> {
    todo!();
}
