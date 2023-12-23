use advent_of_code_2023::{read_input, Day};
use itertools::Itertools;
use regex::Regex;

struct Race {
    distance: u32,
    time: u32,
}

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY6)?;

    println!("Part 1: {}", part1(&input));

    Ok(())
}

fn part1(input: &str) -> u32 {
    let races = parse_races(input);

    races.iter().map(|race| count_winning_moves(race)).product()
}

fn count_winning_moves(race: &Race) -> u32 {
    let range = 1..race.time;
    let reversed = range.clone().rev();
    (range)
        .into_iter()
        .zip(reversed)
        .map(|(x, y)| x * y)
        .filter(|dist| dist > &race.distance)
        .count()
        .try_into()
        .unwrap()
}

fn parse_races(input: &str) -> Vec<Race> {
    let (times, distances) = input.lines().collect_tuple().unwrap();
    let times: Vec<u32> = split_line(times);
    let distances: Vec<u32> = split_line(distances);

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn split_line(line: &str) -> Vec<u32> {
    Regex::new(r"(?:([0-9]+) *)")
        .unwrap()
        .find_iter(line)
        .map(|m| m.as_str().trim())
        .map(|raw| raw.parse().unwrap())
        .collect()
}
