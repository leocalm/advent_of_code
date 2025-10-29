use std::fs;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct DayResult {
    pub day: u32,
    pub part_1: String,
    pub part_2: String,
}

pub trait BaseDay {
    fn get_day_number(&self) -> u32;

    fn part_1(&mut self) -> Result<String, Box<dyn std::error::Error>>;
    fn part_2(&mut self) -> Result<String, Box<dyn std::error::Error>>;

    fn setup(&mut self) {}

    fn run_day(&mut self) -> Result<DayResult, Box<dyn std::error::Error>> {
        self.setup();
        Ok(DayResult {
            day: self.get_day_number(),
            part_1: self.part_1()?,
            part_2: self.part_2()?,
        })
    }

    fn get_input_file_path(&self) -> PathBuf;

    fn read_file(&self) -> String {
        fs::read_to_string(self.get_input_file_path()).expect("Error reading input!")
    }

    fn read_file_into_vec(&self) -> Vec<String> {
        fs::read_to_string(self.get_input_file_path())
            .expect("Error reading input!")
            .lines()
            .map(String::from)
            .collect()
    }

    fn read_file_into_vec_of_vec(&self) -> Vec<Vec<char>> {
        fs::read_to_string(self.get_input_file_path())
            .expect("Error reading input!")
            .lines()
            .map(|c| c.chars().collect())
            .collect()
    }
}
