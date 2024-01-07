use super::Day;

pub struct Day5 {}

// the input format is the following
// dst_start src_start length

impl Day for Day5 {
    fn name(&self) -> String {
        String::from("Day 5: If You Give A Seed A Fertilizer")
    }

    fn part_one(&self, input: &str) -> u64 {
        let sections = parse_input(input);
        let mut seeds = sections[0].to_owned();

        for section in &sections[1..] {
            let mut is_mapped = vec![false; seeds.len()];

            for chunk in section.chunks(3) {
                let (dst_start, src_start, length) = (chunk[0], chunk[1], chunk[2]);

                for i in 0..seeds.len() {
                    if is_mapped[i] {
                        continue;
                    }

                    let current_seed = seeds[i];

                    if is_in_range(current_seed, src_start, length) {
                        let diff = current_seed - src_start;
                        seeds[i] = dst_start + diff;
                        is_mapped[i] = true;
                    }
                }
            }
        }

        *(seeds.iter().min().unwrap_or(&0))
    }

    fn part_two(&self, _input: &str) -> u64 {
        // let sections = parse_input(input);
        // let mut mappedRanges: Vec<(u64, u64)> = Vec::new();
        // let mut seed_ranges_queue: VecDeque<&[u64]> = sections[0].chunks(2).collect();

        // while !seed_ranges_queue.is_empty() {
        //     let current_range = seed_ranges_queue.pop_back().unwrap();

        //     for section in &sections[1..] {
        //         for chunk in section.chunks(3) {
        //             let (dst_start, src_start, length) = (chunk[0], chunk[1], chunk[2]);
        //         }
        //     }
        // }
        0
    }

    /*
        I saw a comment which inspired my solution
        So I think I can create a vector of tuples (src_start, src_len)
        Then I can iterate over the maps, and find overlapping range
        An overlapping range will get mapped to a new range
        And the non-overlapping part will be put in a queue for later processing?
    */
}

fn is_in_range(seed: u64, range_start: u64, length: u64) -> bool {
    seed >= range_start && seed <= range_start + length
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .split("\n\n")
        .map(|x| {
            x.split_once(':')
                .unwrap()
                .1
                .replace('\n', " ")
                .split_whitespace()
                .map(|z| z.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}
