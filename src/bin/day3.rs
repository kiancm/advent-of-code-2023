use std::collections::HashSet;

use advent_of_code_2023::{read_input, Day};

#[derive(Debug, PartialEq, Eq)]
struct PointValue {
    value: i32,
    start: (usize, usize),
    len: usize,
}

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY3)?;

    println!("Part ID Sum: {}", part1(input));

    Ok(())
}

fn part1(input: String) -> i32 {
    let symbol_locations: HashSet<(usize, usize)> = get_symbol_locations(&input);
    let nums = get_nums(&input);
    sum_part_ids(nums, &symbol_locations)
}

fn sum_part_ids(nums: Vec<PointValue>, symbol_locations: &HashSet<(usize, usize)>) -> i32 {
    nums.into_iter()
        .filter(|n| is_part(n, symbol_locations))
        .map(|pv| pv.value)
        .sum()
}

fn get_nums(input: &String) -> Vec<PointValue> {
    let mut nums: Vec<PointValue> = Vec::new();
    let mut num_builder: String = String::new();

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char.is_numeric() {
                num_builder.push(char);
            } else if let Ok(num) = num_builder.parse() {
                nums.push(PointValue {
                    value: num,
                    start: (i, j - num_builder.len()),
                    len: num_builder.len(),
                });
                num_builder.clear();
            }
        }
        if let Ok(num) = num_builder.parse() {
            nums.push(PointValue {
                value: num,
                start: (i, line.len() - num_builder.len()),
                len: num_builder.len(),
            });
            num_builder.clear();
        }
    }
    nums
}

fn get_symbol_locations(input: &String) -> HashSet<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, char)| (i, j, char)))
        .filter(|(_, _, char)| is_symbol(char))
        .map(|(i, j, _)| (i, j))
        .collect()
}

fn is_symbol(char: &char) -> bool {
    !char.is_numeric() && char.to_owned() != '.'
}

fn is_part(n: &PointValue, symbol_locations: &HashSet<(usize, usize)>) -> bool {
    let (x, y) = n.start;
    let x = x as isize;
    let y = y as isize;
    let len: isize = n.len as isize;

    let top: Vec<(isize, isize)> = (y - 1..y + len + 1).map(|y| (x - 1, y)).collect();
    let middle: Vec<(isize, isize)> = vec![(x, y - 1), (x, y + len)];
    let bottom: Vec<(isize, isize)> = (y - 1..y + len + 1).map(|y| (x + 1, y)).collect();

    let adj = top
        .iter()
        .chain(middle.iter())
        .chain(bottom.iter())
        .collect::<Vec<_>>();

    adj.iter()
        .map(|(x, y)| (*x as usize, *y as usize))
        .any(|point| symbol_locations.contains(&point))
}

#[cfg(test)]
mod tests {
    use crate::{get_nums, get_symbol_locations, part1, PointValue};

    #[test]
    fn test_parsing() {
        let input = "....769*148....".to_string();
        let nums = get_nums(&input);
        let symbol_locations = get_symbol_locations(&input);

        let expected_nums = vec![
            PointValue {
                value: 769,
                start: (0, 4),
                len: 3,
            },
            PointValue {
                value: 148,
                start: (0, 8),
                len: 3,
            },
        ];
        assert!(
            nums.iter().all(|n| expected_nums.contains(n)),
            "expected: {:?}, actual: {:?}",
            expected_nums,
            nums
        );

        assert!(
            symbol_locations.contains(&(0, 7)),
            "invalid locations: {:?}",
            symbol_locations
        );
    }

    #[test]
    fn test_adjacent() {
        let input = "....769*148....".to_string();
        test_valid_parts(input, 769 + 148);
    }

    #[test]
    fn test_corners() {
        let input = r#"
            123...234
            ....$....
            345...456
        "#
        .replace(" ", "")
        .to_string();
        test_valid_parts(input, 0);
    }

    #[test]
    fn test_extra_valid() {
        let input = r#"
            ..@@@@@..
            ..@123@..
            ..@@@@@..
        "#
        .replace(" ", "")
        .to_string();
        test_valid_parts(input, 123);
    }

    #[test]
    fn test_taha() {
        let input = r#"
            .........
            ...123...
            ......@..
        "#
        .replace(" ", "")
        .to_string();
        test_valid_parts(input, 123);
    }

    #[test]
    fn test_edges() {
        let input = r#"
            ...123..@
            234.$.345
            @..456...
        "#
        .replace(" ", "")
        .to_string();
        test_valid_parts(input, 123 + 234 + 345 + 456);
    }

    #[test]
    fn test_example() {
        let input = r#"
            ...467..114..
            ......*......
            .....35..633.
            .........#...
            ...617*......
            ........+.58.
            .....592.....
            .........755.
            ......$.*....
            ....664.598..
        "#
        .replace(" ", "")
        .to_string();
        test_valid_parts(input, 4361);
    }

    #[test]
    fn test_middle() {
        let input = r#"
            ..@@@@@..
            .@.....@.
            .@.123.@.
            .@.....@.
            ..@@@@@..
        "#
        .replace(" ", "")
        .to_string();
        test_valid_parts(input, 0);
    }

    fn test_valid_parts(input: String, sum: i32) {
        let actual = part1(input);
        assert_eq!(sum, actual, "expected: {} != actual: {}", sum, actual);
    }
}
