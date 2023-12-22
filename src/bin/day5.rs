use advent_of_code_2023::{read_input, Day};
use regex::Regex;

struct Range {
    input: u32,
    output: u32,
    len: u32,
}
impl Range {
    fn get(&self, num: u32) -> Option<u32> {
        if self.input <= num && num < self.input + self.len {
            Some(self.output + (num - self.input))
        } else {
            None
        }
    }
}

struct Map {
    ranges: Vec<Range>,
}
impl Map {
    fn map(&self, seed: u32) -> u32 {
        self.ranges
            .iter()
            .find_map(|range| range.get(seed))
            .unwrap_or(seed)
    }
}

struct Maps(Vec<Map>);
impl Maps {
    fn map(&self, seed: u32) -> u32 {
        self.0
            .iter()
            .map(|map| |n| map.map(n))
            .fold(seed, |acc, f| f(acc))
    }
}

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY5)?;

    println!("Part 1: {}", part1(&input));

    Ok(())
}

fn part1(input: &String) -> u32 {
    let seeds = parse_seeds(&input);
    let maps = parse_maps(&input);

    seeds.into_iter().map(|seed| maps.map(seed)).min().unwrap()
}

fn parse_seeds(input: &str) -> Vec<u32> {
    let re = Regex::new(r"seeds: ([0-9 ]+)").unwrap();
    let captures = re.captures(&input).unwrap();
    let nums = captures.get(1).map(|m| m.as_str()).unwrap();

    nums.trim().split(" ").map(|n| n.parse().unwrap()).collect()
}

fn parse_maps(input: &str) -> Maps {
    let maps =
    Regex::new(r"\n([0-9 ]+\n)+").unwrap()
    .find_iter(&input)
    .map(|m| m.as_str())
    .map(|raw|  parse_map(raw))
    .collect();

    Maps(maps)
}

fn parse_map(raw: &str) -> Map {
    let ranges = raw
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| parse_range(line))
        .collect();

    Map { ranges }
}

fn parse_range(line: &str) -> Range {
    let re = Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap();
    let caps = re.captures(line).unwrap();

    Range {
        input: caps.get(2).unwrap().as_str().parse().unwrap(),
        output: caps.get(1).unwrap().as_str().parse().unwrap(),
        len: caps.get(3).unwrap().as_str().parse().unwrap(),
    }
}

fn create_regex(s: &str) -> Regex {
    Regex::new(&format!(r#"{} map:\n([0-9 ]+\n)+"#, s)).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_example() {
        let input = r#"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "#
        .trim()
        .to_string();

        assert_eq!(35, part1(&input))
    }
}
