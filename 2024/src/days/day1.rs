use std::collections::HashMap;

use super::Day;

pub struct Day1 {}

impl Day for Day1 {
    fn name(&self) -> String {
        String::from("Day 1: Historian Hysteria")
    }

    // Correct answer: 2285373
    fn part_one(&self, input: &str) -> u64 {
        let (mut left, mut right) = parse_lists(input);
        left.sort();
        right.sort();

        let mut answer: u64 = 0;
        for i in 0..left.len() {
            answer += left[i].abs_diff(right[i]);
        }

        answer
    }

    // Correct answer: 21142653 
    fn part_two(&self, input: &str) -> u64 {
        let (left, right) = parse_lists(input);
        let mut map: HashMap<u64, u64> = HashMap::new();
        left.iter().for_each(|num| {
             map.entry(*num).or_insert(0);
        });

        right.iter().for_each(|num| {
            let current_entry = map.get(num);
            if current_entry.is_some() {
                map.insert(*num, current_entry.unwrap() + 1);
            }
        });

        let mut similarity_score = 0;
        for (num, count) in map.into_iter() {
            similarity_score += num * count;
        }
        
        similarity_score
    }
}

fn parse_lists(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        let left_num: u64 = split.next().unwrap().parse().unwrap();
        let right_num: u64 = split.next().unwrap().parse().unwrap();

        left.push(left_num);
        right.push(right_num);
    });

    return (left, right)
}
