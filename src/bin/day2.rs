use advent_of_code_2023::{read_input, Day};
use regex::Regex;

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

struct Game {
    id: u8,
    draws: Vec<CubeSet>,
}

struct CubeSet {
    red: u8,
    green: u8,
    blue: u8,
}

impl CubeSet {
    fn power(self) -> i32 {
        self.red as i32 * self.green as i32 * self.blue as i32
    }
}

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY2)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &String) {
    let id_sum: i32 = input
        .lines()
        .flat_map(parse_line)
        .filter(|game| game.draws.iter().all(has_valid_cubes))
        .map(|game| game.id)
        .fold(0, |x, y| x as i32 + y as i32);

    println!("Part 1: {}", id_sum);
}

fn part2(input: &String) {
    let power_sum: i32 = input
        .lines()
        .flat_map(parse_line)
        .map(minimum_set)
        .map(CubeSet::power)
        .sum();

    println!("Part 2: {}", power_sum);
}

fn minimum_set(game: Game) -> CubeSet {
    let draws = game.draws;

    let max_red = draws.iter().map(|c| c.red).max().unwrap_or_default();
    let max_green = draws.iter().map(|c| c.green).max().unwrap_or_default();
    let max_blue = draws.iter().map(|c| c.blue).max().unwrap_or_default();

    CubeSet {
        red: max_red,
        green: max_green,
        blue: max_blue,
    }
}

fn has_valid_cubes(draw: &CubeSet) -> bool {
    draw.red <= MAX_RED && draw.green <= MAX_GREEN && draw.blue <= MAX_BLUE
}

fn parse_line(line: &str) -> Option<Game> {
    let parts: Vec<&str> = line.split(":").collect();
    let id = parts.get(0).and_then(|s| {
        let sp = s.split(" ").collect::<Vec<_>>();
        sp.into_iter().last()
    });
    let draws: Vec<CubeSet> = parts
        .get(1)
        .into_iter()
        .flat_map(|s| s.trim().split(";"))
        .map(parse_draw)
        .collect();

    id.and_then(|id| id.parse::<u8>().ok())
        .map(|id| Game { id, draws })
}

fn parse_draw(raw: &str) -> CubeSet {
    let re = Regex::new(r#"([0-9]+) (red|blue|green)"#).ok();

    let pairs: Vec<(u8, &str)> = raw
        .split(",")
        .filter_map(|s| re.as_ref().and_then(|re| re.captures(s)))
        .filter_map(|c| {
            c.get(1)
                .and_then(|num| c.get(2).map(|color| (num.as_str(), color.as_str())))
        })
        .filter_map(|(num, color)| num.parse::<u8>().map(|n| (n, color)).ok())
        .collect();

    let red = pairs
        .iter()
        .find(|(_, color)| color.to_owned() == "red")
        .map(|(n, _)| n)
        .unwrap_or(&0);
    let green = pairs
        .iter()
        .find(|(_, color)| color.to_owned() == "green")
        .map(|(n, _)| n)
        .unwrap_or(&0);
    let blue = pairs
        .iter()
        .find(|(_, color)| color.to_owned() == "blue")
        .map(|(n, _)| n)
        .unwrap_or(&0);

    CubeSet {
        red: *red,
        green: *green,
        blue: *blue,
    }
}
