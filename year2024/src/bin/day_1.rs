use common::base_day::BaseDay;
use common::file::get_input_path;
use common::utils::init_logger;
use log::info;
use std::path::PathBuf;
use std::str::FromStr;

pub struct Day1 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            day_number: 1,
            file_path: get_input_path(2024, 1),
        }
    }

    fn read_lines_into_columns(&self, input: Vec<String>) -> (Vec<u64>, Vec<u64>) {
        let mut col_1: Vec<u64> = Vec::new();
        let mut col_2: Vec<u64> = Vec::new();

        for line in input {
            let split: Vec<u64> = line
                .split("   ")
                .map(|x| u64::from_str(x).expect("Error parsing"))
                .collect::<Vec<u64>>();
            col_1.push(split[0]);
            col_2.push(split[1]);
        }

        (col_1, col_2)
    }
}

impl BaseDay for Day1 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let (mut col_1, mut col_2) = self.read_lines_into_columns(self.read_file_into_vec());

        col_1.sort();
        col_2.sort();

        let mut result: u64 = 0;
        for i in 0..col_1.len() {
            result += col_1[i].abs_diff(col_2[i]);
        }

        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let (col_1, col_2) = self.read_lines_into_columns(self.read_file_into_vec());

        let mut result: u64 = 0;
        for x in col_1.iter() {
            result += col_2.iter().filter(|y| *y == x).count() as u64 * x;
        }
        Ok(result.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = Day1::new();
    init_logger();
    common::file::download_input_file(2024, day.day_number).await?;

    let result = day.run_day()?;
    info!("Day {} - part 1: {:?}", result.day, result.part_1);
    info!("Day {} - part 2: {:?}", result.day, result.part_2);

    Ok(())
}
