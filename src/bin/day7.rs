use std::{cmp::Ordering, convert::identity, hash::Hash, iter::zip, str::FromStr};

use advent_of_code_2023::{read_input, Day};
use itertools::Itertools;
use regex::Regex;
use strum::{Display, EnumString};

#[derive(EnumString, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Display)]
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

impl Card {
    fn cmp_part1(&self, other: &Self) -> Ordering {
        self.cmp(&other)
    }

    fn cmp_part2(&self, other: &Self) -> Ordering {
        match (&self, &other) {
            (Card::JACK, Card::JACK) => Ordering::Equal,
            (Card::JACK, _) => Ordering::Less,
            (_, Card::JACK) => Ordering::Greater,
            (_, _) => self.cmp(&other),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Clone)]
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
impl std::fmt::Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.iter().map(|card| card.to_string()).join(""))
    }
}
impl Cards {
    fn cmp(&self, other: &Self, card_cmp: impl Fn(&Card, &Card) -> Ordering) -> Ordering {
        for (c1, c2) in self.0.iter().zip(other.0.iter()) {
            match card_cmp(c1, c2) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                _ => {}
            }
        }
        Ordering::Equal
    }
    fn compute_hand_type_part1(&self) -> HandType {
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

    fn compute_hand_type_part2(&self) -> HandType {
        if self.is_five_of_a_kind_j() {
            HandType::FiveOfAKind
        } else if self.is_four_of_a_kind_j() {
            HandType::FourOfAKind
        } else if self.is_full_house_j() {
            HandType::FullHouse
        } else if self.is_three_of_a_kind_j() {
            HandType::ThreeOfAKind
        } else if self.is_two_pair_j() {
            HandType::TwoPair
        } else if self.is_pair_j() {
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

    fn is_five_of_a_kind_j(&self) -> bool {
        let joker_count = self.get_joker_count();
        joker_count == 5 || self.get_counts().contains(&(5 - self.get_joker_count()))
    }

    fn is_four_of_a_kind_j(&self) -> bool {
        let joker_count = self.get_joker_count();
        self.get_counts().contains(&(4 - self.get_joker_count()))
            && !(self.is_pair() && joker_count == 2)
    }

    fn is_full_house_j(&self) -> bool {
        (self.is_two_pair() && self.0.contains(&Card::JACK)) | self.is_full_house()
    }

    fn is_three_of_a_kind_j(&self) -> bool {
        self.get_counts().contains(&(3 - self.get_joker_count()))
    }

    fn is_two_pair_j(&self) -> bool {
        self.get_counts().into_iter().filter(|c| c == &2).count() == 2
    }

    fn is_pair_j(&self) -> bool {
        self.0.contains(&Card::JACK) | self.is_pair()
    }

    fn get_joker_count(&self) -> usize {
        self.0
            .iter()
            .counts_by(identity)
            .get(&Card::JACK)
            .map(|v| *v)
            .unwrap_or(0)
    }

    fn get_counts(&self) -> Vec<usize> {
        self.0.iter().counts_by(identity).into_values().collect()
    }
}

#[derive(Clone)]
struct Hand {
    cards: Cards,
    bid: u32,
}
impl Hand {
    fn cmp(
        &self,
        other: &Self,
        card_cmp: impl Fn(&Card, &Card) -> Ordering,
        hand_type_fn: impl Fn(&Cards) -> HandType,
    ) -> Ordering {
        hand_type_fn(&self.cards)
            .cmp(&hand_type_fn(&other.cards))
            .then(self.cards.cmp(&other.cards, card_cmp))
    }
}

fn main() -> std::io::Result<()> {
    let input = read_input(Day::DAY7)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u32 {
    compute_rank_product_sum(input, |h1, h2| {
        h1.cmp(
            h2,
            |c1, c2| c1.cmp_part1(c2),
            Cards::compute_hand_type_part1,
        )
    })
}

fn part2(input: &str) -> u32 {
    compute_rank_product_sum(input, |h1, h2| {
        h1.cmp(
            h2,
            |c1, c2| c1.cmp_part2(c2),
            Cards::compute_hand_type_part2,
        )
    })
}

fn compute_rank_product_sum(input: &str, cmp: impl Fn(&Hand, &Hand) -> Ordering) -> u32 {
    parse_hands(input)
        .into_iter()
        .sorted_by(cmp)
        .zip(0u32..)
        .map(|(hand, i)| hand.bid * (i + 1))
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

    Hand { cards, bid }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, str::FromStr};

    use crate::{part1, part2, Card, Cards, HandType};

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

        assert_eq!(6440, part1(&input));
        assert_eq!(5905, part2(&input));
    }

    #[test]
    fn test_cards_part1() {
        let cs1 = Cards::from_str("77888").unwrap();
        let cs2 = Cards::from_str("77788").unwrap();

        assert_eq!(Ordering::Greater, cs1.cmp(&cs2, |c1, c2| c1.cmp_part1(c2)));
    }

    #[test]
    fn test_hand_types_part2() {
        let cs1 = Cards::from_str("2233J").unwrap();
        let cs2 = Cards::from_str("2333J").unwrap();
        let cs3 = Cards::from_str("22JJ3").unwrap();
        let cs4 = Cards::from_str("JJJJJ").unwrap();
        let cs5 = Cards::from_str("JJ234").unwrap();

        assert_eq!(HandType::FullHouse, cs1.compute_hand_type_part2());
        assert_eq!(HandType::FourOfAKind, cs2.compute_hand_type_part2());
        assert_eq!(HandType::FourOfAKind, cs3.compute_hand_type_part2());
        assert_eq!(HandType::FiveOfAKind, cs4.compute_hand_type_part2());
        assert_eq!(HandType::ThreeOfAKind, cs5.compute_hand_type_part2());
    }

    #[test]
    fn test_joker_rank() {
        assert_eq!(Ordering::Equal, Card::JACK.cmp_part2(&Card::JACK));
        assert_eq!(Ordering::Greater, Card::TWO.cmp_part2(&Card::JACK));
        assert_eq!(Ordering::Less, Card::JACK.cmp_part2(&Card::TWO));
    }
}
