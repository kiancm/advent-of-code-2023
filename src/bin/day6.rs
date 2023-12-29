use advent_of_code_2023::{read_input, Day};
use itertools::Itertools;
use regex::Regex;

struct Race {
    distance: u64,
    time: u64,
}

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY6)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u64 {
    let races = parse_races(input);

    races.iter().map(|race| count_winning_moves(race)).product()
}

fn part2(input: &str) -> u64 {
    let race = parse_race(input);

    count_winning_moves(&race)
}

fn count_winning_moves(race: &Race) -> u64 {
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

fn parse_race(input: &str) -> Race {
    let (times, distances) = input.lines().collect_tuple().unwrap();
    let times = tokenize(times);
    let distances = tokenize(distances);

    let time: u64 = times.into_iter().join("").parse().unwrap();
    let distance: u64 = distances.into_iter().join("").parse().unwrap();

    Race { time, distance }
}

fn parse_races(input: &str) -> Vec<Race> {
    let (times, distances) = input.lines().collect_tuple().unwrap();
    let times: Vec<u64> = split_line(times);
    let distances: Vec<u64> = split_line(distances);

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn tokenize(line: &str) -> Vec<&str> {
    Regex::new(r"(?:([0-9]+) *)")
        .unwrap()
        .find_iter(line)
        .map(|m| m.as_str().trim())
        .collect()
}

fn split_line(line: &str) -> Vec<u64> {
    tokenize(line)
        .iter()
        .map(|raw| raw.parse().unwrap())
        .collect()
}
