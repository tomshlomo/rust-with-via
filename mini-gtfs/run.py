import argparse
from pathlib import Path
import time

parser = argparse.ArgumentParser()

# Add the arguments
parser.add_argument(
    "--use_large_dataset",
    action="store_true",
    default=False,
    help="Use large dataset (default: False)",
)
parser.add_argument(
    "--use_python",
    action="store_true",
    default=False,
    help="Use Python implementation (default: False)",
)
# Parse the arguments
args = parser.parse_args()
folder = Path(__file__).parent / (
    "large_data" if args.use_large_dataset else "small_data"
)

if args.use_python:
    from mini_gtfs_py.gtfs import (
        Stop,
        StopTime,
        find_stop_times_within_time,
        find_stops_within_distance,
    )
else:
    from mini_gtfs_rs import (
        Stop,
        StopTime,
        find_stop_times_within_time,
        find_stops_within_distance,
    )


def main() -> None:
    target_lat = 32.077268
    target_lon = 34.793313
    max_distance_m = 500
    target_time = "12:00:00"
    time_window_minutes = 5

    stops = Stop.from_csv(str(folder / "stops.txt"))
    stop_times = StopTime.from_csv(str(folder / "stop_times.txt"))

    valid_stops = find_stops_within_distance(
        stops=stops,
        target_lat=target_lat,
        target_lon=target_lon,
        max_distance_m=max_distance_m,
    )

    valid_stop_times = find_stop_times_within_time(
        valid_stops,
        stop_times,
        target_time,
        time_window_minutes,
    )

    print(
        f"There are {len(valid_stop_times)} stop times at {len(valid_stops)} stops within {max_distance_m} meters within {time_window_minutes} minutes around {target_time}",
    )


if __name__ == "__main__":
    start_time = time.time()
    main()
    print(f"The run took {(time.time() - start_time):0.02} seconds")
