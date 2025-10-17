use std::error::Error;
use std::path::PathBuf;
use common::base_day::BaseDay;
use itertools::Itertools;
use common::file::get_input_path;

pub struct Day7 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            day_number: 7,
            file_path: get_input_path(2024, 7),
        }
    }

    fn cartesian_with_repetition<'a>(&self, v: &[&'a str], k: usize) -> Vec<Vec<&'a str>> {
        (0..k)
            .map(|_| v.iter())
            .multi_cartesian_product()
            .map(|p| p.into_iter().cloned().collect())
            .collect()
    }
}

impl BaseDay for Day7 {
    fn get_day_number(&self) -> u32 { self.day_number }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        let operators = vec!["+", "*"];

        for line in self.read_file_into_vec() {
            let split = line.split(":").collect::<Vec<&str>>();
            let expected_result = split[0].parse::<u64>().unwrap();
            let numbers = split[1].trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

            for possible_operators in self.cartesian_with_repetition(&operators, numbers.len() - 1) {
                let mut index = 0;
                let calibration_value = numbers[1..].iter().fold(numbers[0], |a, b| {
                    let op = possible_operators[index];
                    index += 1;
                    if op == "+"{
                        return a + b;
                    } else if op == "*" {
                        return a * b;
                    }
                    return 0;
                });

                if calibration_value == expected_result {
                    result += calibration_value;
                    break;
                }

            }
        }
        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        let operators = vec!["+", "*", "||"];

        for line in self.read_file_into_vec() {
            let split = line.split(":").collect::<Vec<&str>>();
            let expected_result = split[0].parse::<u64>().unwrap();
            let numbers = split[1].trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

            for possible_operators in self.cartesian_with_repetition(&operators, numbers.len() - 1) {
                let mut index = 0;
                let calibration_value = numbers[1..].iter().fold(numbers[0], |a, b| {
                    let op = possible_operators[index];
                    index += 1;
                    if op == "+"{
                        return a + b;
                    } else if op == "*" {
                        return a * b;
                    } else if op == "||" {
                        return (a.to_string() + &b.to_string()).parse::<u64>().unwrap();
                    }
                    return 0;
                });

                if calibration_value == expected_result {
                    result += calibration_value;
                    break;
                }

            }
        }
        Ok(result.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}