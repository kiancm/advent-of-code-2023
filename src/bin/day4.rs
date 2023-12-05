use std::collections::{HashMap, HashSet};

use advent_of_code_2023::{read_input, Day};

struct Card {
    nums: Vec<usize>,
    winning_nums: HashSet<usize>,
}

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY4)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &String) -> i32 {
    let cards: Vec<Card> = parse_cards(&input);

    cards
        .iter()
        .map(num_matches)
        .filter(|matches| matches > &0)
        .map(|matches| i32::pow(2, (matches - 1) as u32))
        .sum()
}

fn part2(input: &String) -> usize {
    let cards: Vec<Card> = parse_cards(&input);

    let cards: Vec<usize> = cards.iter().map(num_matches).collect();

    let len = cards.len();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut count = 0;

    for (ri, card) in cards.into_iter().rev().enumerate() {
        let i = len - ri - 1;
        if card == 0 {
            map.insert(i, 1);
            count += 1;
        } else {
            let sum: usize = (i + 1..i + 1 + card).map(|j| map.get(&j).unwrap()).sum();
            let sum = sum + 1;
            map.insert(i, sum);
            count += sum;
        }
    }
    count
}

fn num_matches(c: &Card) -> usize {
    c.nums.iter().filter(|n| c.winning_nums.contains(n)).count()
}

fn parse_cards(input: &String) -> Vec<Card> {
    input
        .lines()
        .flat_map(|line| line.split(":").nth(1))
        .flat_map(|rest| parse_card(rest))
        .collect()
}

fn parse_card(rest: &str) -> Option<Card> {
    if let [winning_nums, nums] = rest.split("|").collect::<Vec<_>>().as_slice() {
        let nums: Vec<usize> = nums
            .trim()
            .split(" ")
            .flat_map(|n| n.trim().parse::<usize>())
            .collect();
        let winning_nums: HashSet<usize> = winning_nums
            .trim()
            .split(" ")
            .flat_map(|n| n.trim().parse::<usize>())
            .collect();
        Some(Card { nums, winning_nums })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test() {
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#
        .trim()
        .to_string();

        assert_eq!(30, part2(&input));
    }
}
