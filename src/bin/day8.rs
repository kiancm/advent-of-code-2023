use std::{collections::HashMap, iter::repeat_with, str::FromStr};

use advent_of_code_2023::{read_input, Day};
use itertools::Itertools;
use strum::EnumString;

#[derive(EnumString)]
enum Instruction {
    #[strum(serialize = "L")]
    LEFT,
    #[strum(serialize = "R")]
    RIGHT,
}

const START_KEY: &str = "AAA";
const END_KEY: &str = "ZZZ";

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY8)?;

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> Option<u64> {
    let instructions = parse_instructions(input);
    let map = parse_map(input);
    get_path_length(START_KEY, &instructions, &map, |key| key == END_KEY)
}

fn get_path_length(
    start_key: &str,
    instructions: &Vec<Instruction>,
    map: &HashMap<&str, (&str, &str)>,
    predicate: impl Fn(&str) -> bool,
) -> Option<u64> {
    let instructions = repeat_with(|| instructions).into_iter().flatten();
    let mut key = start_key;
    for (i, ins) in (1u64..).zip(instructions) {
        let next = match ins {
            Instruction::LEFT => map.get(key).unwrap().0,
            Instruction::RIGHT => map.get(key).unwrap().1,
        };
        if predicate(next) {
            return Some(i);
        } else {
            key = next
        }
    }
    None
}

fn part2(input: &str) -> u64 {
    let instructions = parse_instructions(input);
    let map = parse_map(input);

    map.keys()
        .cloned()
        .filter(|key| key.ends_with("A"))
        .map(|key| get_path_length(key, &instructions, &map, |key| key.ends_with("Z")).unwrap())
        .fold(1, |a, b| lcm(a, b))
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd::euclid_u64(a, b)
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|char| Instruction::from_str(&char.to_string()).unwrap())
        .collect()
}

fn parse_map(input: &str) -> HashMap<&str, (&str, &str)> {
    input.lines().skip(2).map(parse_map_entry).collect()
}

fn parse_map_entry(line: &str) -> (&str, (&str, &str)) {
    let (raw_key, values) = line.split("=").collect_tuple().unwrap();
    let (raw_left, raw_right) = values.split(", ").collect_tuple().unwrap();

    (
        raw_key.trim(),
        (&raw_left.trim()[1..], &raw_right[0..raw_right.len() - 1]),
    )
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example1() {
        let input = r#"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "#
        .trim();

        assert_eq!(Some(6), part1(input));
    }

    #[test]
    fn test_example2() {
        let input = r#"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "#
        .trim();

        assert_eq!(6, part2(input));
    }
}
