use std::cmp::Ordering;
use std::collections::HashMap;

use crate::day07::WinType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl From<(char, bool)> for Card {
    fn from(value: (char, bool)) -> Self {
        let (card, j_is_joker) = value;
        match card {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => if j_is_joker { Card::Joker } else { Card::Jack },
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            other => panic!("Don't know what a '{}' card is", other),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum WinType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
    win_type: WinType,
}

impl Hand {
    fn new(cards: [Card; 5], bid: i64) -> Self {
        Hand {
            cards,
            bid,
            win_type: Self::determine_win_type(&cards),
        }
    }

    fn determine_win_type(cards: &[Card; 5]) -> WinType {
        let mut counts = HashMap::new();
        for card in cards {
            *counts.entry(*card).or_insert_with(|| 0u8) += 1;
        }
        let wilds = counts.remove(&Card::Joker).unwrap_or(0);
        match counts.len() {
            0 | 1 => FiveOfAKind,
            2 => {
                if counts.iter().any(|(_, &count)| (count + wilds) == 4)  {
                    FourOfAKind
                } else {
                    FullHouse
                }
            }
            3 => {
                if counts.iter().any(|(_, &count)| (count + wilds) == 3) {
                    ThreeOfAKind
                } else {
                    TwoPair
                }
            }
            4 => OnePair,
            5 => HighCard,
            _ => panic!("More cards than expected! {:?}", cards),
        }
    }
}

pub fn total_winnings(input: &str, jokers_wild: bool) -> i64 {
    let mut hands = load_hands(input, jokers_wild);
    hands.sort_unstable_by(|a, b| match a.win_type.cmp(&b.win_type) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            a.cards.cmp(&b.cards)
        }
    });
    hands.iter().enumerate().map(|(i, h)| (i as i64 + 1) * h.bid).sum()
}

fn load_hands(input: &str, jokers_wild: bool) -> Vec<Hand> {
    input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(cards, bid)| Hand::new(cards.chars().map(|c| Card::from((c, jokers_wild))).collect::<Vec<Card>>().try_into().unwrap(),
                                      bid.parse::<i64>().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn example_1() {
        assert_eq!(total_winnings(EXAMPLE, false), 6440);
    }

    #[test]
    fn example_2() {
        assert_eq!(total_winnings(EXAMPLE, true), 5905);
    }

    #[test]
    fn part_1() {
        println!("Part 1: {}", total_winnings(include_str!("../res/day07.txt"), false));
    }

    #[test]
    fn part_2() {
        println!("Part 2: {}", total_winnings(include_str!("../res/day07.txt"), true));
    }
}
