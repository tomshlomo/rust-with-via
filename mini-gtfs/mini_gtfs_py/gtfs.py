from __future__ import annotations

import csv
import math
from typing import Iterable, Self

from pydantic import BaseModel, field_validator
from tqdm import tqdm


class FromCsv(BaseModel):
    @classmethod
    def from_csv(cls, file_path: str) -> list[Self]:
        with open(file_path, newline="", encoding="utf-8-sig") as csvfile:
            reader = csv.DictReader(csvfile)
            return [
                cls.model_validate(row)
                for row in tqdm(reader, desc=f"Reading {file_path}")
            ]


# Define Pydantic models
class Stop(FromCsv):
    stop_id: str
    stop_name: str
    stop_lat: float
    stop_lon: float
    stop_desc: str


class StopTime(FromCsv):
    trip_id: str
    departure_time: int
    stop_id: str

    @field_validator("departure_time", mode="before")
    def validate_time(cls, value: str) -> int:
        return parse_time(value)


def parse_time(value: str) -> int:
    try:
        hour, minute, second = map(int, value.split(":"))
        return hour * 3600 + minute * 60 + second
    except Exception:
        raise ValueError(value)


def haversine(lat1: float, lon1: float, lat2: float, lon2: float) -> float:
    earth_radius = 6_371_000  # Radius of the Earth in meters
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (
        math.sin(dlat / 2) ** 2
        + math.cos(math.radians(lat1))
        * math.cos(math.radians(lat2))
        * math.sin(dlon / 2) ** 2
    )
    c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))
    return earth_radius * c  # Distance in meters


def find_stops_within_distance(
    stops: Iterable[Stop],
    target_lat: float,
    target_lon: float,
    max_distance_m: int,
) -> list[Stop]:

    return [
        stop
        for stop in stops
        if haversine(target_lat, target_lon, stop.stop_lat, stop.stop_lon)
        <= max_distance_m
        and stop.stop_id is not None
    ]


def find_stop_times_within_time(
    stops: Iterable[Stop],
    stop_times: Iterable[StopTime],
    target_time: str,
    time_window_minutes: int,
) -> list[StopTime]:
    target_time_seconds = parse_time(target_time)
    time_window_seconds = time_window_minutes * 60
    valid_stop_ids = {stop.stop_id for stop in stops}
    return [
        stop_time
        for stop_time in stop_times
        if (
            stop_time.stop_id in valid_stop_ids
            and target_time_seconds - time_window_seconds
            <= stop_time.departure_time
            <= target_time_seconds + time_window_seconds
        )
    ]
