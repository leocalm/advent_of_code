use common::base_day::BaseDay;
use common::file::get_input_path;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;

pub struct Day19 {
    day_number: u32,
    file_path: PathBuf,
    data_bool: HashMap<String, bool>,
    data_count: HashMap<String, u64>,
}

impl Day19 {
    pub fn new() -> Day19 {
        Day19 {
            day_number: 19,
            file_path: get_input_path(2024, 19),
            data_bool: HashMap::new(),
            data_count: HashMap::new(),
        }
    }

    fn find_valid_chunks(
        &self,
        line: &str,
        size: usize,
        available_towels: &Vec<String>,
    ) -> Vec<String> {
        let mut result = Vec::new();
        for index in 0..line.len() - size + 1 {
            let _s = line[index..(index + size)].to_string();
            if available_towels.contains(&_s) {
                result.push(_s);
            }
        }

        result
    }

    fn can_build(&mut self, target: &str, possible_combinations: &HashSet<String>) -> bool {
        if let Some(&cached) = self.data_bool.get(target) {
            return cached;
        }

        if target.is_empty() {
            true
        } else {
            for p in possible_combinations {
                if target.starts_with(p) {
                    if self.can_build(&target[p.len()..], possible_combinations) {
                        self.data_bool.insert(target.to_string(), true);
                        return true;
                    }
                }
            }
            self.data_bool.insert(target.to_string(), false);
            false
        }
    }

    fn count_paths(&mut self, target: &str, possible_combinations: &HashSet<String>) -> u64 {
        if let Some(&value) = self.data_count.get(target) {
            return value;
        }

        let mut result = 0;
        if target.is_empty() {
            return 1;
        } else {
            for p in possible_combinations {
                if target.starts_with(p) {
                    result += self.count_paths(&target[p.len()..], possible_combinations);
                }
            }
        }
        self.data_count.insert(target.to_string(), result);
        result
    }
}

impl BaseDay for Day19 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result = 0;

        let input = self.read_file_into_vec();
        let available_towels = input
            .get(0)
            .unwrap()
            .split(", ")
            .map(|t| t.to_string())
            .collect::<Vec<String>>();
        let mut towels_by_size = HashMap::new();
        for towel in available_towels.iter() {
            let entry = towels_by_size.entry(towel.len()).or_insert(Vec::new());
            entry.push(towel);
        }
        for line in input[2..input.len()].iter() {
            let mut possible = HashSet::new();
            for size in towels_by_size.keys().sorted() {
                let bla = self.find_valid_chunks(line, *size, &available_towels);
                possible.extend(bla);
            }
            if self.can_build(line, &possible) {
                result += 1;
            }
        }
        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result = 0;

        let input = self.read_file_into_vec();
        let available_towels = input
            .get(0)
            .unwrap()
            .split(", ")
            .map(|t| t.to_string())
            .collect::<Vec<String>>();
        let mut towels_by_size = HashMap::new();
        for towel in available_towels.iter() {
            let entry = towels_by_size.entry(towel.len()).or_insert(Vec::new());
            entry.push(towel);
        }
        for line in input[2..input.len()].iter() {
            let mut possible = HashSet::new();
            for size in towels_by_size.keys().sorted() {
                let bla = self.find_valid_chunks(line, *size, &available_towels);
                possible.extend(bla);
            }
            result += self.count_paths(line, &possible);
        }
        Ok(result.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();
    }

    #[test]
    fn part_1_test() {
        init_logger();
        let mut day = Day19::new();
        let _path = "./data/day_19/example_1.txt".to_string();
        let expected = "6";

        let result = day.part_1();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_1_test_2() {
        let mut day = Day19::new();
        let _path = "./data/day_19/example_2.txt".to_string();
        let expected = "1";

        let result = day.part_1();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_2_test() {
        init_logger();

        let mut day = Day19::new();
        let _path = "./data/day_19/example_1.txt".to_string();
        let expected = "16";

        let result = day.part_2();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn rebuild_string_test() {
        let mut day = Day19::new();
        let target = "brabwurr";
        let possible_combinations = HashSet::from_iter(
            vec!["bwu", "b", "r", "ab", "br", "wu"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );

        let result = day.can_build(target, &possible_combinations);
        assert_eq!(result, true);
    }

    #[test]
    fn rebuild_string_test_2() {
        let mut day = Day19::new();
        let target = "bbrgwb";
        let possible_combinations = HashSet::from_iter(
            vec!["r", "b", "g", "br"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );

        let result = day.can_build(target, &possible_combinations);
        println!("data: {:?}", day.data_bool);
        assert_eq!(result, false);
    }
}
