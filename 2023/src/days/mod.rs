use crate::utils;
use std::time::{Duration, Instant};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub trait Day {
    fn name(&self) -> String;
    fn part_one(&self, input: &str) -> u64;
    fn part_two(&self, input: &str) -> u64;
}

pub struct DayResult {
    pub name: String,

    pub part_one_answer: u64,
    pub part_one_time: Duration,

    pub part_two_answer: u64,
    pub part_two_time: Duration,
}

pub fn get_day_result(day_num: usize, day: &Box<dyn Day>) -> DayResult {
    let input = utils::read_input(day_num);

    let part_one_started = Instant::now();
    let part_one_answer = day.part_one(&input);
    let part_one_time = part_one_started.elapsed();

    let part_two_started = Instant::now();
    let part_two_answer = day.part_two(&input);
    let part_two_time = part_two_started.elapsed();

    DayResult {
        name: day.name(),
        part_one_answer,
        part_one_time,
        part_two_answer,
        part_two_time,
    }
}
