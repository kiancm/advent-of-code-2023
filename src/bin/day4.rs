use std::collections::HashSet;

use advent_of_code_2023::{read_input, Day};


struct Card {
    nums: Vec<usize>,
    winning_nums: HashSet<usize>
}

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY4)?;

    let result = part1(&input);

    println!("{}", result);

    Ok(())
}

fn part1(input: &String) -> i32 {
    let cards: Vec<Card> = parse_cards(&input);

    cards.iter()
        .map(num_matches)
        .filter(|matches| matches > &0)
        .map(|matches| i32::pow(2, (matches - 1) as u32))
        .sum()
}

fn num_matches(c: &Card) -> usize {
    c.nums.iter().filter(|n| c.winning_nums.contains(n)).count()
}

fn parse_cards(input: &String) -> Vec<Card> {
    input.lines()
        .flat_map(|line| line.split(":").nth(1))
        .flat_map(|rest| parse_card(rest))
        .collect()
}

fn parse_card(rest: &str) -> Option<Card> {
    if let [winning_nums, nums] = rest.split("|").collect::<Vec<_>>().as_slice() {
        let nums: Vec<usize> = nums.trim().split(" ").flat_map(|n| n.trim().parse::<usize>()).collect();
        let winning_nums: HashSet<usize> = winning_nums.trim().split(" ").flat_map(|n| n.trim().parse::<usize>()).collect();
        Some(Card { nums, winning_nums })
    } else {
        None
    }
}
