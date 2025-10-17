use common::base_day::BaseDay;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use common::file::get_input_path;

pub struct Day11 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 {
            day_number: 11,
            file_path: get_input_path(2024, 11)
        }
    }

    fn blink_stone_tuple(&self, stone: u64) -> (u64, Option<u64>) {
        if stone == 0 {
            (1, None)
        } else if stone.to_string().len() % 2 == 0 {
            let stone_string = stone.to_string();
            let first_stone = *&stone_string.as_str()[0..stone_string.len() / 2]
                .parse::<u64>()
                .unwrap();
            let second_stone = *&stone_string.as_str()[stone_string.len() / 2..stone_string.len()]
                .parse::<u64>()
                .unwrap();
            (first_stone, Some(second_stone))
        } else {
            (stone * 2024, None)
        }
    }

    fn blink_for(&self, times: u64, input: &Vec<u64>) -> u64 {
        let mut counts = HashMap::new();
        for v in input {
            *counts.entry(*v).or_insert(0) += 1;
        }

        for _ in 0..times {
            let mut tmp = HashMap::new();
            for (stone, count) in counts {
                let blink_result = self.blink_stone_tuple(stone);
                *tmp.entry(blink_result.0).or_insert(0) += count;
                if blink_result.1.is_some() {
                    *tmp.entry(blink_result.1.unwrap()).or_insert(0) += count;
                }
            }
            counts = tmp;
        }
        counts.values().sum()
    }
}

impl BaseDay for Day11 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.blink_for(
            25,
            &self
                .read_file_into_vec()
                .first()
                .unwrap()
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        ).to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.blink_for(
            75,
            &self
                .read_file_into_vec()
                .first()
                .unwrap()
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        ).to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
