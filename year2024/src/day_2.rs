use std::error::Error;
use std::path::PathBuf;
use common::base_day::BaseDay;
use common::file::get_input_path;

pub struct Day2 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {
            day_number: 2,
            file_path: get_input_path(2024, 2)
        }
    }

    fn is_level_safe(&self, data: &Vec<u64>) -> bool {
        let ascending = data[0]< data[1];
        if data[0].abs_diff(data[1]) > 3 || data[0] == data[1] {
            return false;
        }

        for index in 1..data.len() - 1 {
            if data[index].abs_diff(data[index + 1]) > 3 || data[index] == data[index + 1] {
                return false;
            }

            if ascending {
                if data[index] > data[index + 1]{
                    return false;
                }
            } else {
                if data[index] < data[index + 1]{
                    return false;
                }
            }
        }
        true
    }
}

impl BaseDay for Day2{
    fn get_day_number(&self) -> u32{
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        for line in self.read_file_into_vec() {
            let data = line.split(' ').map(|c| c.parse().unwrap()).collect::<Vec<u64>>();
            if self.is_level_safe(&data) {
                result += 1;
            }
        }
        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        for line in self.read_file_into_vec() {
            let data = line.split(' ').map(|c| c.parse().unwrap()).collect::<Vec<u64>>();
            if self.is_level_safe(&data) {
                result += 1;
            } else {
                for index in 0..data.len() {
                    let mut new_data = data.clone();
                    new_data.remove(index);

                    if self.is_level_safe(&new_data) {
                        result += 1;
                        break
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