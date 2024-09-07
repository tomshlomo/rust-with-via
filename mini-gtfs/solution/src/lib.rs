use pyo3::prelude::*;

use std::collections::HashSet;

use serde::Deserialize;

mod helper;
pub use helper::read_from_csv;
use helper::{deserialize_time, parse_time};

#[derive(Debug, Deserialize, Clone)]
#[pyclass(get_all)]
pub struct Stop {
    stop_id: u32,
    stop_name: String,
    stop_lat: f64,
    stop_lon: f64,
    stop_desc: String,
}

#[pymethods]
impl Stop {
    #[staticmethod]
    pub fn from_csv(path: &str) -> Vec<Stop> {
        read_from_csv(path)
    }
}

#[derive(Debug, Deserialize, Clone)]
#[pyclass(get_all)]
pub struct StopTime {
    trip_id: String,
    #[serde(deserialize_with = "deserialize_time")]
    departure_time: u32,
    stop_id: u32,
}

#[pymethods]
impl StopTime {
    #[staticmethod]
    pub fn from_csv(path: &str) -> Vec<StopTime> {
        read_from_csv(path)
    }
}

fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS: f64 = 6_371_000.0; // Radius of the Earth in meters

    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();

    let a =
        (dlat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);

    let c = 2.0 * (a.sqrt()).atan2((1.0 - a).sqrt());

    EARTH_RADIUS * c // Distance in meters
}

#[pyfunction]
pub fn find_stops_within_distance(
    stops: Vec<Stop>,
    target_lat: f64,
    target_lon: f64,
    max_distance_m: u32,
) -> Vec<Stop> {
    stops
        .into_iter()
        .filter(|stop| {
            haversine(stop.stop_lat, stop.stop_lon, target_lat, target_lon) <= max_distance_m as f64
        })
        .collect()
}

#[pyfunction]
pub fn find_stop_times_within_time(
    stops: Vec<Stop>,
    stops_times: Vec<StopTime>,
    target_time: &str,
    time_window_minutes: u32,
) -> Vec<StopTime> {
    let valid_stop_ids: HashSet<_> = stops.into_iter().map(|stop| stop.stop_id).collect();
    let target_time = parse_time(target_time).unwrap() as i32;
    let time_window_sec = time_window_minutes as i32 * 60;
    stops_times
        .into_iter()
        .filter(|stop_time| {
            valid_stop_ids.contains(&stop_time.stop_id)
                && (stop_time.departure_time as i32 - target_time).abs() <= time_window_sec
        })
        .collect()
}

#[pymodule]
fn mini_gtfs_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Stop>()?;
    m.add_class::<StopTime>()?;
    m.add_function(wrap_pyfunction!(find_stop_times_within_time, m)?)?;
    m.add_function(wrap_pyfunction!(find_stops_within_distance, m)?)?;
    Ok(())
}
