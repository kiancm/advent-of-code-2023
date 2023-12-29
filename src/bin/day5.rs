use advent_of_code_2023::{read_input, Day};
use itertools::Itertools;
use regex::Regex;

struct Range {
    input: u64,
    output: u64,
    len: u64,
}
impl Range {
    fn get(&self, num: u64) -> Option<u64> {
        if self.input <= num && num < self.input + self.len {
            Some(self.output + (num - self.input))
        } else {
            None
        }
    }

    fn input_end(&self) -> u64 {
        return self.input + self.len - 1;
    }

    fn get_overlap(&self, range: (u64, u64)) -> Overlap {
        let (start, len) = range;
        let end = start + len - 1;
        if start <= self.input && end >= self.input && end <= self.input_end() {
            let mut outside = Vec::new();
            if start < self.input {
                outside.push((start, self.input - start));
            }
            Overlap {
                inside: vec![(self.input, end - self.input + 1)],
                outside,
            }
        } else if start >= self.input && end <= self.input_end() {
            Overlap {
                inside: vec![range],
                outside: vec![],
            }
        } else if start < self.input && self.input_end() < end {
            Overlap {
                inside: vec![(self.input, self.len)],
                outside: vec![
                    (start, self.input - start),
                    (self.input_end() + 1, end - self.input_end()),
                ],
            }
        } else if start <= self.input_end() && self.input_end() < end {
            Overlap {
                inside: vec![(start, self.input_end() - start + 1)],
                outside: vec![(self.input_end() + 1, end - self.input_end())],
            }
        } else {
            Overlap {
                inside: vec![],
                outside: vec![range],
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Overlap {
    inside: Vec<(u64, u64)>,
    outside: Vec<(u64, u64)>,
}

struct Map {
    ranges: Vec<Range>,
}
impl Map {
    fn map(&self, seed: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| range.get(seed))
            .unwrap_or(seed)
    }

    fn map_range(&self, range: (u64, u64)) -> Vec<(u64, u64)> {
        let (matching, out) =
            self.ranges
                .iter()
                .fold((vec![], vec![range]), |(matches, out), range| {
                    let (new_matches, new_out) = out
                        .into_iter()
                        .map(|r| range.get_overlap(r))
                        .fold((vec![], vec![]), |(m, o), ov| {
                            ([ov.inside, m].concat(), [ov.outside, o].concat())
                        });
                    (
                        [
                            new_matches
                                .into_iter()
                                .map(|(start, len)| (self.map(start), len))
                                .collect_vec(),
                            matches,
                        ]
                        .concat(),
                        new_out,
                    )
                });
        [matching, out].concat()
    }
}

struct Maps(Vec<Map>);
impl Maps {
    fn map(&self, seed: u64) -> u64 {
        self.0
            .iter()
            .map(|map| |n| map.map(n))
            .fold(seed, |acc, f| f(acc))
    }

    fn map_range(&self, range: (u64, u64)) -> Vec<(u64, u64)> {
        self.0.iter().fold(vec![range], |ranges, map| {
            ranges
                .into_iter()
                .flat_map(|range| map.map_range(range))
                .collect()
        })
    }
}

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY5)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &String) -> u64 {
    let seeds = parse_seeds(&input);
    let maps = parse_maps(&input);

    seeds.into_iter().map(|seed| maps.map(seed)).min().unwrap()
}

fn part2(input: &String) -> u64 {
    let seeds = parse_seeds(&input);
    let maps = parse_maps(&input);

    let seed_ranges: Vec<(u64, u64)> = seeds
        .iter()
        .enumerate()
        .group_by(|(i, _)| i / 2)
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<_>>())
        .map(|group| (*group[0].1, *group[1].1))
        .collect();

    seed_ranges
        .into_iter()
        .flat_map(|range| maps.map_range(range))
        .map(|(start, _)| start)
        .min()
        .unwrap()
}

fn parse_seeds(input: &str) -> Vec<u64> {
    let re = Regex::new(r"seeds: ([0-9 ]+)").unwrap();
    let captures = re.captures(&input).unwrap();
    let nums = captures.get(1).map(|m| m.as_str()).unwrap();

    nums.trim().split(" ").map(|n| n.parse().unwrap()).collect()
}

fn parse_maps(input: &str) -> Maps {
    let maps = Regex::new(r"\n([0-9 ]+\n)+")
        .unwrap()
        .find_iter(&input)
        .map(|m| m.as_str())
        .map(|raw| parse_map(raw))
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

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Overlap, Range};

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

        assert_eq!(35, part1(&input));
        assert_eq!(46, part2(&input));
    }

    #[test]
    fn test_overlap() {
        let range = Range {
            input: 3,
            output: 5,
            len: 2,
        };

        assert_eq!(
            Overlap {
                inside: vec![(3, 2)],
                outside: vec![(1, 2), (5, 1)]
            },
            range.get_overlap((1, 5))
        );
        assert_eq!(
            Overlap {
                inside: vec![(3, 1)],
                outside: vec![(1, 2)]
            },
            range.get_overlap((1, 3))
        );
        assert_eq!(
            Overlap {
                inside: vec![(4, 1)],
                outside: vec![(5, 1)]
            },
            range.get_overlap((4, 2))
        );
        assert_eq!(
            Overlap {
                inside: vec![],
                outside: vec![(1, 2)]
            },
            range.get_overlap((1, 2))
        );
    }
}
