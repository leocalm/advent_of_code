use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use common::base_day::BaseDay;
use common::file::get_input_path;

pub struct Day5 {
    day_number: u32,
    file_path: PathBuf,
    rules: HashSet<(u64, u64)>,
    reversed_rules: HashSet<(u64, u64)>,
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {
            day_number: 5,
            file_path: get_input_path(2024, 5),
            rules: HashSet::new(),
            reversed_rules: HashSet::new(),
        }
    }

    fn is_line_valid(&self, data: &Vec<u64>) -> bool {
        for index in 0..data.len() {
            for second_index in index..data.len() {
                let first = data.get(index).unwrap();
                let second = data.get(second_index).unwrap();
                if self.reversed_rules.contains(&(*first, *second)) {
                    return false;
                }
            }
        }
        true
    }

    fn get_errors(&self, data: &Vec<u64>) -> Vec<((usize, u64), (usize, u64))> {
        let mut errors: Vec<((usize, u64), (usize, u64))> = Vec::new();

        for index in 0..data.len() {
            for second_index in index..data.len() {
                let first = data.get(index).unwrap();
                let second = data.get(second_index).unwrap();
                if self.reversed_rules.contains(&(*first, *second)) {
                    errors.push(((index, *first), (second_index, *second)));
                }
            }
        }

        errors
    }

    fn read_rule(&mut self, line: &String) {
        let (first, second) = line.split_once("|").unwrap();
        self.rules.insert((first.parse::<u64>().unwrap(), second.parse::<u64>().unwrap()));
        self.reversed_rules.insert((second.parse::<u64>().unwrap(), first.parse::<u64>().unwrap()));
    }

    fn fix_line_and_get_middle(&mut self, data: &Vec<u64>, errors: &Vec<((usize, u64), (usize, u64))>) -> u64 {
        let error = *errors.first().unwrap();

        let mut new_data = data.clone();
        new_data.remove(error.0.0);
        new_data.insert(error.0.0, error.1.1);
        new_data.remove(error.1.0);
        new_data.insert(error.1.0, error.0.1);

        if self.is_line_valid(&new_data) {
            return *new_data.get(new_data.len() / 2).unwrap();
        }
        self.fix_line_and_get_middle(&new_data, &self.get_errors(&new_data))
    }
}

impl BaseDay for Day5 {
    fn get_day_number(&self) -> u32 { self.day_number }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;

        for line in self.read_file_into_vec() {
            if line.contains("|") {
                self.read_rule(&line);
            } else if line.len() > 1 {
                let data = line.split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                if self.is_line_valid(&data) {
                    let middle = data.get(data.len() / 2).unwrap();
                    result += middle;
                }
            }
        }

        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;

        for line in self.read_file_into_vec() {
            if line.contains("|") {
                self.read_rule(&line);
            } else if line.len() > 1 {
                let data = line.split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                let errors = self.get_errors(&data);
                if errors.len() > 0 {
                    result += self.fix_line_and_get_middle(&data, &errors);
                }
            }
        }

        Ok(result.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}