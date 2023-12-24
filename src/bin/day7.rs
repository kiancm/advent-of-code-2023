use std::{cmp::Ordering, convert::identity, str::FromStr};

use advent_of_code_2023::{read_input, Day};
use itertools::Itertools;
use regex::Regex;
use strum::EnumString;

#[derive(EnumString, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    #[strum(serialize = "2")]
    TWO,
    #[strum(serialize = "3")]
    THREE,
    #[strum(serialize = "4")]
    FOUR,
    #[strum(serialize = "5")]
    FIVE,
    #[strum(serialize = "6")]
    SIX,
    #[strum(serialize = "7")]
    SEVEN,
    #[strum(serialize = "8")]
    EIGHT,
    #[strum(serialize = "9")]
    NINE,
    #[strum(serialize = "T")]
    TEN,
    #[strum(serialize = "J")]
    JACK,
    #[strum(serialize = "Q")]
    QUEEN,
    #[strum(serialize = "K")]
    KING,
    #[strum(serialize = "A")]
    ACE,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct ParseCardsError;

#[derive(PartialEq, Eq)]
struct Cards(Vec<Card>);
impl FromIterator<Card> for Cards {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        Cards(iter.into_iter().collect())
    }
}
impl FromStr for Cards {
    type Err = ParseCardsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|c| Card::from_str(&c.to_string()))
            .collect::<Result<Cards, _>>()
            .map_err(|_| ParseCardsError)
    }
}
impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        for (c1, c2) in self.0.iter().zip(other.0.iter()) {
            match c1.cmp(c2) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                _ => {}
            }
        }
        Ordering::Equal
    }
}
impl Cards {
    fn compute_hand_type(&self) -> HandType {
        if self.is_five_of_a_kind() {
            HandType::FiveOfAKind
        } else if self.is_four_of_a_kind() {
            HandType::FourOfAKind
        } else if self.is_full_house() {
            HandType::FullHouse
        } else if self.is_three_of_a_kind() {
            HandType::ThreeOfAKind
        } else if self.is_two_pair() {
            HandType::TwoPair
        } else if self.is_pair() {
            HandType::Pair
        } else {
            HandType::HighCard
        }
    }

    fn is_five_of_a_kind(&self) -> bool {
        self.0.iter().unique().count() == 1
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.get_counts().contains(&4)
    }

    fn is_full_house(&self) -> bool {
        self.is_three_of_a_kind() && self.is_pair()
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.get_counts().contains(&3)
    }

    fn is_two_pair(&self) -> bool {
        self.get_counts().into_iter().filter(|c| c == &2).count() == 2
    }

    fn is_pair(&self) -> bool {
        self.get_counts().into_iter().filter(|c| c == &2).count() == 1
    }

    fn get_counts(&self) -> Vec<usize> {
        self.0.iter().counts_by(identity).into_values().collect()
    }
}

struct Hand {
    hand_type: HandType,
    cards: Cards,
    bid: u32,
}
impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(|o| o == Ordering::Equal)
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then(self.cards.cmp(&other.cards))
    }
}

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY7)?;

    println!("Part 1: {}", part1(&input));
    // part2(&input);

    Ok(())
}

fn part1(input: &str) -> u32 {
    parse_hands(&input)
        .into_iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

fn parse_hands(input: &str) -> Vec<Hand> {
    let re = Regex::new(r"([0-9TJQKA]{5}) ([0-9]+)").unwrap();
    input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
        .map(|(raw_hand, raw_bid)| parse_hand(raw_hand, raw_bid))
        .collect()
}

fn parse_hand(raw_hand: &str, raw_bid: &str) -> Hand {
    let cards = Cards::from_str(raw_hand).unwrap();
    let bid = raw_bid.parse().unwrap();
    let hand_type = cards.compute_hand_type();

    Hand {
        hand_type,
        cards,
        bid,
    }
}

fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, str::FromStr};

    use crate::{part1, Cards};

    #[test]
    fn test_example() {
        let input = r#"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "#
        .trim();

        assert_eq!(6440, part1(&input))
    }

    #[test]
    fn test_cards() {
        let c1 = Cards::from_str("77888").unwrap();
        let c2 = Cards::from_str("77788").unwrap();

        assert_eq!(Ordering::Greater, c1.cmp(&c2));
    }
}
