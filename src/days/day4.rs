use super::Day;
use std::collections::{HashMap, HashSet};

pub struct Day4 {}

impl Day for Day4 {
    fn name(&self) -> String {
        String::from("Day 4: Scratchcards")
    }

    fn part_one(&self, input: &str) -> u64 {
        input.lines().fold(0, |acc, line| {
            let match_count = get_card_matches(line);

            if match_count > 0 {
                acc + u64::pow(2, (match_count - 1).try_into().unwrap())
            } else {
                acc
            }
        })
    }

    fn part_two(&self, input: &str) -> u64 {
        let cards: Vec<&str> = input.lines().collect();
        let matches: Vec<u64> = cards.iter().map(|card| get_card_matches(card)).collect();

        // a map that will store the card indices as keys and amounts as values
        let mut card_amounts: HashMap<u64, u64> = (0..cards.len()).map(|i| (i as u64, 1)).collect();

        for (i, matches) in matches.iter().enumerate() {
            let i: u64 = i as u64;
            let card_num = i + 1;

            for index in card_num..=i + matches {
                let cur_amount = *card_amounts.get(&i).unwrap();

                if let Some(amount) = card_amounts.get_mut(&index) {
                    *amount += cur_amount;
                }
            }
        }

        card_amounts.values().sum()
    }
}

fn get_card_matches(card: &str) -> u64 {
    let mut nums = card.split(":").nth(1).unwrap().split("|");
    let winning: HashSet<u64> = nums
        .nth(0)
        .unwrap()
        .split_whitespace()
        .map(|str| str.parse::<u64>().unwrap_or(0))
        .collect();

    let mine: HashSet<u64> = nums
        .nth(0)
        .unwrap()
        .split_whitespace()
        .map(|str| str.parse::<u64>().unwrap_or(0))
        .collect();

    winning
        .intersection(&mine)
        .copied()
        .count()
        .try_into()
        .unwrap()
}
