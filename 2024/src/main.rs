use crate::days::*;
use std::{env, process::exit, vec};

mod days;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(day1::Day1 {})
    ];

    let mut results: Vec<DayResult> = Vec::new();

    if args.len() < 2 {
        for (i, day) in days.iter().enumerate() {
            results.push(get_day_result(i + 1, &day))
        }
    } else {
        let day_num = args[1].parse::<usize>().unwrap_or(0);

        if day_num == 0 {
            println!("Please provide a valid day number");
            return;
        }

        let day_index = day_num - 1;

        let day = match days.get(day_index) {
            Some(d) => d,
            None => {
                println!("Please provide a valid day number");
                exit(1);
            }
        };
        results.push(get_day_result(day_num, day));
    }
    utils::draw_table(results);
}
