use advent_of_code_2023::{read_input, Day};

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY9)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> i32 {
    let histories = parse_histories(input);

    histories.iter().map(|h| extrapolate_forward(h)).sum()
}

fn part2(input: &str) -> i32 {
    let histories = parse_histories(input);

    histories.iter().map(|h| extrapolate_backward(h)).sum()
}

fn extrapolate_forward(history: &[i32]) -> i32 {
    diffs(history)
        .iter()
        .rev()
        .fold(0, |acc, x| acc + x.last().unwrap())
}

fn extrapolate_backward(history: &[i32]) -> i32 {
    diffs(history)
        .iter()
        .rev()
        .fold(0, |acc, x| x.first().unwrap() - acc)
}

fn diffs(history: &[i32]) -> Vec<Vec<i32>> {
    let mut diffs = vec![history.to_vec()];
    while diffs.last().unwrap().iter().sum::<i32>() != 0 {
        diffs.push(diff(diffs.last().unwrap()));
    }
    diffs
}

fn diff(history: &[i32]) -> Vec<i32> {
    history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

fn parse_histories(input: &str) -> Vec<Vec<i32>> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.trim()
        .split(" ")
        .map(|raw| raw.trim().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example1() {
        let input = r#"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "#
        .trim();

        assert_eq!(114, part1(input));
        assert_eq!(2, part2(input));
    }
}
