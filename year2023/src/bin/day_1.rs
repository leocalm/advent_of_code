use common::base_day::BaseDay;
use common::file::get_input_path;
use common::utils::init_logger;
use log::info;
use std::error::Error;
use std::path::PathBuf;

const DIGITS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

pub struct Day1 {
    day_number: u32,
    file_path: PathBuf,
}

impl Default for Day1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            day_number: 1,
            file_path: get_input_path(2023, 1),
        }
    }
}

impl BaseDay for Day1 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result = 0;

        for line in self.read_file_into_vec() {
            let mut first_digit = None;
            let mut last_digit = None;

            line.chars().for_each(|c| {
                if c.is_ascii_digit() {
                    if first_digit.is_none() {
                        first_digit = Some(c.to_digit(10).unwrap());
                    }

                    last_digit = Some(c.to_digit(10).unwrap());
                }
            });

            result += first_digit.unwrap() * 10 + last_digit.unwrap();
        }

        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result = 0;

        for line in self.read_file_into_vec() {
            let mut first_digit = 0;
            let mut first_position = usize::MAX;

            let mut last_digit = 0;
            let mut last_position = 0;

            for (index, digit) in DIGITS.iter().enumerate() {
                if let Some(position) = line.find(digit)
                    && position < first_position
                {
                    first_position = position;
                    first_digit = if index < 10 { index } else { index - 10 };
                }
            }

            for (index, digit) in DIGITS.iter().enumerate() {
                if let Some(position) = line.rfind(digit)
                    && position > last_position
                {
                    last_position = position;
                    last_digit = if index < 10 { index } else { index - 10 };
                }
            }

            if last_digit == 0 {
                last_digit = first_digit;
            }

            if first_digit == 0 {
                first_digit = last_digit;
            }

            result += first_digit * 10 + last_digit;
        }

        Ok(result.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut day = Day1::new();
    init_logger();
    common::file::download_input_file(2023, day.day_number).await?;

    let result = day.run_day()?;
    info!("Day {} - part 1: {:?}", result.day, result.part_1);
    info!("Day {} - part 2: {:?}", result.day, result.part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::file::get_data_dir;
    use common::test_utils;

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        test_utils::init_logger();

        let mut day = Day1::new();
        day.file_path = get_data_dir(2023, 1).join("example_1.txt");

        let expected = String::from("142");

        let result = day.part_1()?;
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        test_utils::init_logger();

        let mut day = Day1::new();
        day.file_path = get_data_dir(2023, 1).join("example_2.txt");

        let expected = String::from("281");

        let result = day.part_2()?;
        assert_eq!(expected, result);

        Ok(())
    }
}
