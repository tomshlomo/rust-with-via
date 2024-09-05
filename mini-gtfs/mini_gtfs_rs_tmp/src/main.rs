use mini_gtfs_rs::{
    find_stop_times_within_time, find_stops_within_distance, read_from_csv, Stop, StopTime,
};
fn main() {
    let stops = read_from_csv("../small_data/stops.txt");
    let stop_times = read_from_csv("../small_data/stop_times.txt");

    println!(
        "There are {} stops and {} stop times",
        stops.len(),
        stop_times.len(),
    );

    let target_lat = 32.077_268;
    let target_lon = 34.793_313;
    let max_distance_m = 500;
    let target_time = "12:00:00";
    let time_window_minutes = 5;

    let relevant_stops = find_stops_within_distance(stops, target_lat, target_lon, max_distance_m);

    println!(
        "There are {} stops within {max_distance_m} meters",
        relevant_stops.len(),
    );
    println!(
        "There are {} stop times within {max_distance_m} meters within {time_window_minutes} minutes around {target_time}",
        find_stop_times_within_time(
            relevant_stops,
            stop_times,
            target_time,
            time_window_minutes
        )
        .len(),
    );
}
