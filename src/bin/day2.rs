use advent_of_code_2023::{read_input, Day};
use regex::Regex;

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY2)?;

    let id_sum: i32 = input.lines().flat_map(parse_line)
        .filter(|game| game.draws.iter().all(has_valid_cubes))
        .map(|game| game.id)
        .fold(0, |x, y| x as i32 + y as i32);

    println!("ID Sum: {}", id_sum);

    Ok(())
}

fn has_valid_cubes(draw: &Draw) -> bool {
    draw.red <= MAX_RED && draw.green <= MAX_GREEN && draw.blue <= MAX_BLUE
}

struct Game {
    id: u8,
    draws: Vec<Draw>,
}

struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}

fn parse_line(line: &str) -> Option<Game> {
    let parts: Vec<&str> = line.split(":").collect();
    let id = parts.get(0).and_then(|s| {
        let sp = s.split(" ").collect::<Vec<_>>();
        sp.into_iter().last()});
    let draws: Vec<Draw> = parts
        .get(1)
        .into_iter()
        .flat_map(|s| s.trim().split(";"))
        .map(parse_draw)
        .collect();

    id.and_then(|id| id.parse::<u8>().ok())
        .map(|id| Game { id, draws })
}

fn parse_draw(raw: &str) -> Draw {
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

    Draw {
        red: *red,
        green: *green,
        blue: *blue
    }
}
