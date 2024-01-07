use super::Day;
use std::collections::HashMap;

pub struct Day2 {}

impl Day for Day2 {
    fn name(&self) -> String {
        String::from("Day 2: Cube Conundrum")
    }

    fn part_one(&self, input: &str) -> u64 {
        let initial = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        let mut sum = 0;

        input.lines().for_each(|line: &str| {
            let (id, cube_map) = get_cube_info(line);
            let mut is_valid = true;

            for (color, count) in cube_map.into_iter() {
                let initial = initial.get(color).unwrap();

                if count > *initial {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                sum += id;
            }
        });

        return sum;
    }

    fn part_two(&self, input: &str) -> u64 {
        let mut power_sum = 0;

        input.lines().for_each(|line: &str| {
            let (_, cube_map) = get_cube_info(line);
            let mut power = 1;

            for (_, count) in cube_map.into_iter() {
                power *= count;
            }

            power_sum += power;
        });

        return power_sum;
    }
}

fn get_cube_info(line: &str) -> (u64, HashMap<&str, u64>) {
    let mut cube_map: HashMap<&str, u64> = HashMap::new();
    let mut split = line.split(":");

    let id = split.next().unwrap().split(" ").nth(1).unwrap();

    let cube_entries = match split.next() {
        Some(x) => x,
        None => panic!("Incorrect entry"),
    }
    .split(";");

    for entry in cube_entries {
        entry.split(",").for_each(|c: &str| {
            let mut parsed_cube = c.trim().split(" ");
            let count = parsed_cube.next().unwrap().trim().parse().unwrap();
            let color = parsed_cube.next().unwrap();

            match cube_map.get(color) {
                Some(x) => {
                    if count > *x {
                        cube_map.insert(color, count);
                    }
                }

                None => {
                    cube_map.insert(color, count);
                }
            }
        });
    }

    return (id.parse().unwrap(), cube_map);
}
