use common::base_day::BaseDay;
use common::file::get_input_path;
use common::utils::init_logger;
use log::info;
use std::error::Error;
use std::path::PathBuf;

pub struct Day13 {
    day_number: u32,
    file_path: PathBuf,
}

impl Default for Day13 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day13 {
    pub fn new() -> Day13 {
        Day13 {
            day_number: 13,
            file_path: get_input_path(2023, 13),
        }
    }
}

impl BaseDay for Day13 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(String::new())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(String::new())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = Day13::new();
    init_logger();
    common::file::download_input_file(2023, day.day_number).await?;

    let result = day.run_day()?;
    info!("Day {} - part 1: {:?}", result.day, result.part_1);
    info!("Day {} - part 2: {:?}", result.day, result.part_2);

    Ok(())
}
