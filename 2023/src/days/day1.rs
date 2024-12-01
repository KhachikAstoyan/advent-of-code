use super::Day;

pub struct Day1 {}

impl Day for Day1 {
    fn name(&self) -> String {
        String::from("Day 1: Trebuchet?!")
    }

    fn part_one(&self, input: &str) -> u64 {
        input
            .lines()
            .map(|line| {
                let first = line.chars().find(|c: &char| c.is_digit(10)).unwrap();
                let second = line.chars().rfind(|c: &char| c.is_digit(10)).unwrap();
                let digit = format!("{}{}", first, second);
                digit.parse::<u64>().unwrap()
            })
            .sum::<u64>()
    }

    fn part_two(&self, input: &str) -> u64 {
        let mut input = input.to_string();
        let spelled_num = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let digit_num = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

        for (spelled, digit) in spelled_num.into_iter().zip(digit_num) {
            input = input.replace(spelled, digit);
        }

        self.part_one(&input)
    }
}
