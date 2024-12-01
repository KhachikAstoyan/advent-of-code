use std::{ops::Add, time::Duration};

use crate::days::DayResult;

pub fn read_input(day: usize) -> String {
    let path = format!("inputs/day{}.txt", day);
    std::fs::read_to_string(path).expect("Failed to read input file")
}

// heavily inspired by https://github.com/tymscar/Advent-Of-Code/blob/master/2023/rust/src/main.rs
pub fn draw_table(days: Vec<DayResult>) {
    let max_name_len = days.iter().map(|d| d.name.len()).max().unwrap();
    let max_p1_len = days
        .iter()
        .map(|d| d.part_one_answer.to_string().len())
        .max()
        .unwrap();
    let max_p2_len = days
        .iter()
        .map(|d| d.part_one_answer.to_string().len())
        .max()
        .unwrap();
    let max_p1_time_len = days
        .iter()
        .map(|d| format!("{:.2?}", d.part_one_time).len())
        .max()
        .unwrap();
    let max_p2_time_len = days
        .iter()
        .map(|d| format!("{:.2?}", d.part_two_time).len())
        .max()
        .unwrap();

    let p1_header_len = max_p1_len + 5;
    let p2_header_len = max_p2_len + 5;

    let max_total_len =
        max_name_len + p1_header_len + p2_header_len + max_p1_time_len + max_p2_time_len + 10;

    println!("╔{}╗", "═".repeat(max_total_len + 4));
    println!("║ {:^max_total_len$} ║", "🦀 Advent of Code 2023 🦀");
    println!(
        "╠{}╦{}╦{}╦{}╦{}╣",
        "═".repeat(max_name_len + 2),
        "═".repeat(p1_header_len + 2),
        "═".repeat(max_p1_time_len + 2),
        "═".repeat(p2_header_len + 2),
        "═".repeat(max_p2_time_len + 2),
    );

    println!(
        "║ {:max_name_len$} ║ {:p1_header_len$} ║ {:max_p1_time_len$} ║ {:p2_header_len$} ║ {:max_p2_time_len$} ║",
        "Day", "Part 1", "Time", "Part 2", "Time"
    );

    println!(
        "╠{}╬{}╬{}╬{}╬{}╣",
        "═".repeat(max_name_len + 2),
        "═".repeat(p1_header_len + 2),
        "═".repeat(max_p1_time_len + 2),
        "═".repeat(p2_header_len + 2),
        "═".repeat(max_p2_time_len + 2),
    );

    for day in &days {
        println!(
            "║ {:max_name_len$} ║ {:p1_header_len$} ║ {:max_p1_time_len$} ║ {:p2_header_len$} ║ {:max_p2_time_len$} ║",
            day.name,
            day.part_one_answer,
            format!("{:.2?}", day.part_one_time),
            day.part_two_answer,
            format!("{:.2?}", day.part_two_time),
        );
    }

    println!(
        "╚{}╩{}╩{}╩{}╩{}╝",
        "═".repeat(max_name_len + 2),
        "═".repeat(p1_header_len + 2),
        "═".repeat(max_p1_time_len + 2),
        "═".repeat(p2_header_len + 2),
        "═".repeat(max_p2_time_len + 2),
    );

    let total_time: Duration = days
        .iter()
        .map(|d| d.part_one_time.add(d.part_two_time))
        .sum();

    println!("Total time: {:.2?}", total_time);
}
