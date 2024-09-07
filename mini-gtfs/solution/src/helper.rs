use std::{fs::File, num::ParseIntError};

use serde::{de::DeserializeOwned, Deserialize, Deserializer};

pub fn deserialize_time<'a, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'a>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(parse_time(s).unwrap())
}

pub fn parse_time(time: &str) -> Result<u32, ParseIntError> {
    let parts: Vec<&str> = time.split(':').collect();

    let hours: u32 = parts[0].parse()?;
    let minutes: u32 = parts[1].parse()?;
    let seconds: u32 = parts[2].parse()?;
    Ok(hours * 3600 + minutes * 60 + seconds)
}

pub fn read_from_csv<T: DeserializeOwned>(file_name: &str) -> Vec<T> {
    let file = File::open(file_name).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    rdr.deserialize().filter_map(Result::ok).collect()
}
