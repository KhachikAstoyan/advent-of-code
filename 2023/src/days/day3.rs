use super::Day;

pub struct Day3 {}

impl Day for Day3 {
    fn name(&self) -> String {
        String::from("Day 3: Gear Ratios")
    }

    fn part_one(&self, input: &str) -> u64 {
        let mut part_num_sum: u64 = 0;
        let lines: Vec<&str> = input.lines().collect();

        for (line_id, line) in lines.iter().enumerate() {
            let current_line_nums = nums_w_indices(line);

            for (start_idx, num) in current_line_nums.iter() {
                if is_part_number(&lines, line_id, *start_idx, start_idx + num.len() - 1) {
                    part_num_sum += num.parse().unwrap_or(0);
                }
            }
        }

        return part_num_sum;
    }

    fn part_two(&self, input: &str) -> u64 {
        let mut gear_ratio_sum: u64 = 0;
        let lines: Vec<&str> = input.lines().collect();

        for (line_id, line) in lines.iter().enumerate() {
            for (char_idx, char) in line.char_indices() {
                if char == '*' {
                    match get_gear_ratio(&lines, line_id, char_idx) {
                        Some(ratio) => gear_ratio_sum += ratio,
                        None => (),
                    }
                }
            }
        }

        return gear_ratio_sum;
    }
}

fn get_gear_ratio(schematic: &Vec<&str>, line_id: usize, gear_id: usize) -> Option<u64> {
    let mut all_nums = nums_w_indices(schematic[line_id]);
    let mut prev_line_nums = if line_id > 0 {
        nums_w_indices(schematic[line_id - 1])
    } else {
        Vec::new()
    };

    let mut next_line_nums = if line_id < schematic.len() - 1 {
        nums_w_indices(schematic[line_id + 1])
    } else {
        Vec::new()
    };

    all_nums.append(&mut prev_line_nums);
    all_nums.append(&mut next_line_nums);

    let adj_nums: Vec<u64> = all_nums
        .iter()
        .filter(|(id, num)| {
            let start = if *id > 0 { *id - 1 } else { *id };
            let end = id + num.len();

            gear_id >= start && gear_id <= end
        })
        .map(|(_, num)| num.parse().unwrap_or(0))
        .collect();

    if adj_nums.len() == 2 {
        let mut prod = 1;
        adj_nums.iter().for_each(|num| prod *= num);

        return Some(prod);
    }

    None
}

fn is_part_number(schematic: &Vec<&str>, line_id: usize, start: usize, end: usize) -> bool {
    let current_line: Vec<char> = schematic[line_id].chars().collect(); // it's fine if the program panics with invalid index
    let last_idx = current_line.len() - 1;
    let prev_idx = if start > 0 { start - 1 } else { 0 };
    let next_idx = if end < last_idx { end + 1 } else { last_idx };

    // first check adjacent indices
    if let Some(ch) = current_line.get(prev_idx) {
        if is_symbol(ch) {
            return true;
        }
    }

    if let Some(ch) = current_line.get(next_idx) {
        if is_symbol(ch) {
            return true;
        }
    }

    // check the previous line
    if line_id > 0 {
        let prev_line = schematic.get(line_id - 1);

        if let Some(line) = prev_line {
            let chars: Vec<char> = line.chars().collect();
            for i in prev_idx..=next_idx {
                let ch = chars.get(i).unwrap_or(&'.');

                if is_symbol(ch) {
                    return true;
                }
            }
        }
    }

    // check the next line
    if line_id < schematic.len() - 1 {
        let next_line = schematic.get(line_id + 1);

        if let Some(line) = next_line {
            let chars: Vec<char> = line.chars().collect();

            for i in prev_idx..=next_idx {
                let ch = chars.get(i).unwrap_or(&'.');

                if is_symbol(ch) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn is_symbol(ch: &char) -> bool {
    !ch.is_digit(10) && *ch != '.'
}

fn nums_w_indices(s: &str) -> Vec<(usize, &str)> {
    let mut result = Vec::new();
    let mut current_num_start: Option<usize> = None;

    for (index, char) in s.char_indices() {
        if char.is_digit(10) {
            if current_num_start.is_none() {
                // If this is the first digit of a number, record the start index
                current_num_start = Some(index);
            }
        } else if let Some(start) = current_num_start {
            // If we were tracking a number and encountered a non-digit character,
            // record the number and its start index
            let number = &s[start..index];
            result.push((start, number));
            current_num_start = None; // Reset for the next number
        }
    }

    // Check if the last character is part of a number
    if let Some(start) = current_num_start {
        let number = &s[start..];
        result.push((start, number));
    }

    result
}
