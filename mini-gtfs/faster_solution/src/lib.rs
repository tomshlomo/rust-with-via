use pyo3::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
    // Wrapping the return value to avoid the creation of a Python list
    #[staticmethod]
    pub fn from_csv(path: &str) -> StopsContainer {
        StopsContainer(read_from_csv(path))
    }
}

#[pyclass]
pub struct StopsContainer(Vec<Stop>);

#[pymethods]
impl StopsContainer {
    pub fn __len__(&self) -> usize {
        self.0.len()
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
    // Wrapping the return value to avoid the creation of a Python list
    #[staticmethod]
    pub fn from_csv(path: &str) -> StopTimesContainer {
        StopTimesContainer(read_from_csv(path))
    }
}

#[pyclass]
pub struct StopTimesContainer(Vec<StopTime>);

#[pymethods]
impl StopTimesContainer {
    pub fn __len__(&self) -> usize {
        self.0.len()
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
    // Using &StopsContainer instead of a Vec<Stop> avoids the creation of a Python list,
    // and the cloning of all the Stops.
    stops: &StopsContainer,
    target_lat: f64,
    target_lon: f64,
    max_distance_m: u32,
) -> StopsContainer {
    // Return StopsContainer to avoid the creation of a Python list.
    StopsContainer(
        stops
            .0
            // par_iter uses the Rayon crate to parallelize the iteration
            .par_iter()
            .filter_map(|stop| {
                (haversine(stop.stop_lat, stop.stop_lon, target_lat, target_lon)
                    <= max_distance_m as f64)
                    // Clone only the stops that are within the max_distance_m
                    .then(|| stop.clone())
            })
            .collect(),
    )
}

#[pyfunction]
pub fn find_stop_times_within_time(
    // Using &StopsContainer instead of a Vec<Stop> avoids the creation of a Python list,
    // and the cloning of all the Stops
    stops: &StopsContainer,
    // Using &StopTimesContainer instead of a Vec<StopTimes> avoids the creation of a Python list,
    // and the cloning of all the StopTimes
    stops_times: &StopTimesContainer,
    target_time: &str,
    time_window_minutes: u32,
) -> StopTimesContainer {
    let valid_stop_ids: HashSet<_> = stops.0.iter().map(|stop| stop.stop_id).collect();
    let target_time = parse_time(target_time).unwrap() as i32;
    let time_window_sec = time_window_minutes as i32 * 60;
    // Return StopTimesContainer to avoid the creation of a Python list
    StopTimesContainer(
        stops_times
            .0
            // par_iter uses the Rayon crate to parallelize the iteration
            .par_iter()
            .filter_map(|stop_time| {
                (valid_stop_ids.contains(&stop_time.stop_id)
                    && (stop_time.departure_time as i32 - target_time).abs() <= time_window_sec)
                    // Clone only the required stop_times
                    .then(|| stop_time.clone())
            })
            .collect(),
    )
}

#[pymodule]
fn mini_gtfs_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Stop>()?;
    m.add_class::<StopTime>()?;
    m.add_class::<StopTimesContainer>()?;
    m.add_class::<StopsContainer>()?;
    m.add_function(wrap_pyfunction!(find_stop_times_within_time, m)?)?;
    m.add_function(wrap_pyfunction!(find_stops_within_distance, m)?)?;
    Ok(())
}
