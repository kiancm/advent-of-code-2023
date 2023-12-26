use std::{borrow::Borrow, collections::HashMap, iter::repeat_with, str::FromStr};

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
    println!("Part 2: {:?}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> Option<u32> {
    let instructions = parse_instructions(input);
    let map = parse_map(input);
    let instructions = repeat_with(|| &instructions).into_iter().flatten();
    let mut key = START_KEY;
    for (i, ins) in (1u32..).zip(instructions) {
        let next = match ins {
            Instruction::LEFT => map.get(key).unwrap().0,
            Instruction::RIGHT => map.get(key).unwrap().1,
        };
        if next == END_KEY {
            return Some(i)
        } else {
            key = next
        }
    }
    None
}

fn part2(input: &str) -> Option<u32> {
    let instructions = parse_instructions(input);
    let map = parse_map(input);
    let instructions = repeat_with(|| &instructions).into_iter().flatten();
    let mut keys = map.keys().into_iter().filter(|key| key.ends_with("A")).map(|key| key.to_owned()).collect_vec();
    for (i, ins) in (1u32..).zip(instructions) {
        let nexts = keys.into_iter().map(|key| match ins {
            Instruction::LEFT => map.get(key).unwrap().0,
            Instruction::RIGHT => map.get(key).unwrap().1,
        });
        if nexts.clone().all(|n| n.ends_with("Z")) {
            return Some(i)
        } else {
            keys = nexts.collect_vec()
        }
    }
    None
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
    use crate::part1;


    #[test]
    fn test_example() {
        let input = r#"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "#.trim();

        assert_eq!(Some(6), part1(input));
    }
}
