use super::Day;
use std::{cmp::Ordering, collections::HashMap};

pub struct Day7 {}

impl Day for Day7 {
    fn name(&self) -> String {
        String::from("Day 7: Camel Cards")
    }

    fn part_one(&self, input: &str) -> u64 {
        let mut hands = parse_hands(input);
        hands.sort_by(compare_hands);

        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) as u64 * hand.bid)
            .sum()
    }

    fn part_two(&self, _input: &str) -> u64 {
        0
    }
}

// all cards sorted in ascending order based on their strength
const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Combination {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: u64,
}

impl Hand {
    fn get_combination(&self) -> Combination {
        let mut card_counts: HashMap<char, u8> = HashMap::new();
        self.hand
            .chars()
            .for_each(|char| match card_counts.get(&char) {
                Some(val) => {
                    card_counts.insert(char, val + 1);
                }
                None => {
                    card_counts.insert(char, 1);
                }
            });

        let card_counts: Vec<u8> = card_counts.values().map(|v| *v).collect();

        if card_counts.len() == 1 {
            Combination::FiveOfKind
        } else if card_counts.len() == 2 {
            let has_three_of_same = card_counts.contains(&3);

            // if there are two counts, and one of them
            // is 3, then the other one has to be 2 -> full house
            if has_three_of_same {
                Combination::FullHouse
            } else {
                Combination::FourOfKind
            }
        } else {
            let pair_count = card_counts
                .iter()
                .filter(|c| **c == 2)
                .collect::<Vec<&u8>>()
                .len();

            if pair_count == 1 {
                Combination::OnePair
            } else if pair_count == 2 {
                Combination::TwoPair
            } else {
                let has_three = card_counts.contains(&3);

                if has_three {
                    Combination::ThreeOfKind
                } else {
                    Combination::HighCard
                }
            }
        }
    }
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    let a_combination = a.get_combination();
    let b_combination = b.get_combination();

    let cmp = a_combination.cmp(&b_combination);

    match cmp {
        Ordering::Equal => {
            let a_hand = a.hand.chars();
            let b_hand = b.hand.chars();
            let mut order = Ordering::Equal;

            for (card1, card2) in a_hand.zip(b_hand) {
                let card_order = compare_cards(&card2, &card1);
                if card_order != Ordering::Equal {
                    order = card_order;
                    break;
                }
            }

            order
        }
        order => order,
    }
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let parsed: Vec<&str> = line.split_ascii_whitespace().collect();
            let hand = parsed[0];

            Hand {
                hand: sort_hand(hand),
                bid: parsed[1].parse::<u64>().unwrap(),
            }
        })
        .collect()
}

fn sort_hand(hand: &str) -> String {
    let mut cards: Vec<char> = hand.chars().collect();
    cards.sort_by(compare_cards);
    cards.into_iter().collect::<String>()
}

fn compare_cards(a: &char, b: &char) -> Ordering {
    let a_index = CARDS.iter().position(|c| c == a).unwrap() as i32;
    let b_index = CARDS.iter().position(|c| c == b).unwrap() as i32;

    let diff = b_index - a_index;

    if diff < 0 {
        Ordering::Less
    } else if diff > 0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
