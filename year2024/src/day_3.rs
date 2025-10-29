use common::base_day::BaseDay;
use std::error::Error;
use std::path::PathBuf;

use common::file::get_input_path;
use regex::Regex;

pub struct Day3 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            day_number: 3,
            file_path: get_input_path(2024, 3),
        }
    }
}

impl BaseDay for Day3 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;

        for data in self.read_file_into_vec() {
            let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
            for (_, [a, b]) in re.captures_iter(data.as_str()).map(|c| c.extract()) {
                result += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
            }
        }
        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        let mut enabled = true;

        for data in self.read_file_into_vec() {
            let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();
            for match_result in re.find_iter(data.as_str()) {
                if match_result.as_str() == "do()" {
                    enabled = true;
                } else if match_result.as_str() == "don't()" {
                    enabled = false;
                } else {
                    if enabled {
                        let numbers: Vec<u64> = match_result
                            .as_str()
                            .split('(')
                            .last()
                            .unwrap()
                            .split(')')
                            .next()
                            .unwrap()
                            .split(',')
                            .map(|c| c.parse::<u64>().unwrap())
                            .collect();
                        result += numbers[0] * numbers[1];
                    }
                }
            }
        }
        Ok(result.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
