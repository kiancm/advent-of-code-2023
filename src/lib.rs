use std::{path::PathBuf, fs};

use strum::{EnumString, Display};

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Day {
    DAY1,
    DAY2,
    DAY3,
    DAY4,
    DAY5,
}

pub fn read_input(day: Day) -> std::io::Result<String> {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = format!("data/{}/input.txt", day);
    base.push("data/");

    fs::read_to_string(path)
}
