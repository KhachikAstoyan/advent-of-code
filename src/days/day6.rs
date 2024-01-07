use super::Day;

pub struct Day6 {}

impl Day for Day6 {
    fn name(&self) -> String {
        String::from("Day 6: Wait For It")
    }

    fn part_one(&self, input: &str) -> u64 {
        let races = parse_part_one(input);

        let win_situations: Vec<u64> = races
            .iter()
            .map(|race| {
                let mut win_cases: u64 = 0;
                for i in 1..race.time {
                    if i * (race.time - i) > race.distance {
                        win_cases += 1;
                    }
                }

                return win_cases;
            })
            .collect();

        win_situations.iter().product()
    }

    fn part_two(&self, input: &str) -> u64 {
        let (time, distance) = parse_part_two(input);

        let discriminant = (time.pow(2) - 4 * distance) as f64;
        let time = time as f64;
        let sqrt = discriminant.sqrt();
        let min = (time - sqrt) / 2.0;
        let max = (time + sqrt) / 2.0;

        (max - min).abs().round() as u64
    }
}

struct Race {
    time: u64,
    distance: u64,
}

fn parse_part_one(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let times = parse_nums_from_str(lines[0]);
    let distances = parse_nums_from_str(lines[1]);

    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect()
}

fn parse_part_two(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.lines().collect();
    let times = parse_nums_from_str(lines[0]);
    let distances = parse_nums_from_str(lines[1]);

    (concat_nums(times), concat_nums(distances))
}

fn concat_nums(nums: Vec<u64>) -> u64 {
    let t = nums
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string());

    t.parse::<u64>().unwrap()
}

fn parse_nums_from_str(string: &str) -> Vec<u64> {
    string.split_whitespace().collect::<Vec<&str>>()[1..]
        .iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}
