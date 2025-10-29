use common::base_day::BaseDay;
use common::file::get_input_path;
use common::grid::{Grid, Point};
use common::utils::init_logger;
use log::info;
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug)]
struct PartNumber {
    value: u32,
    row: i32,
    start_col: i32,
    end_col: i32,
}

impl PartNumber {
    fn new(value: &Vec<char>, row: i32, start_col: i32, end_col: i32) -> Self {
        Self {
            value: String::from_iter(value).parse().unwrap(),
            row,
            start_col,
            end_col,
        }
    }
}

pub struct Day3 {
    day_number: u32,
    file_path: PathBuf,
    grid: Grid<char>,
    part_numbers: Vec<PartNumber>,
}

impl Default for Day3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            day_number: 3,
            file_path: get_input_path(2023, 3),
            grid: Grid::new(),
            part_numbers: Vec::new(),
        }
    }

    fn part_numbers(&self, grid: &Grid<char>) -> Vec<PartNumber> {
        let mut part_numbers = Vec::new();
        for x in 0..grid.rows() {
            let mut y = 0;
            while y < grid.cols() {
                if let Some(c) = grid.get(Point { x, y })
                    && c.is_ascii_digit()
                {
                    let number_chars: Vec<char> = (y..grid.cols())
                        .map(|col| *grid.get(Point { x, y: col }).unwrap())
                        .take_while(|c| c.is_ascii_digit())
                        .collect();
                    part_numbers.push(PartNumber::new(
                        &number_chars,
                        x,
                        y,
                        y + number_chars.len() as i32,
                    ));
                    y += number_chars.len() as i32;
                } else {
                    y += 1;
                }
            }
        }
        part_numbers
    }

    fn possible_part_positions(&self, part_number: &PartNumber) -> Vec<Point> {
        let mut possible_part_positions = (part_number.start_col - 1..=part_number.end_col)
            .flat_map(|y| {
                vec![
                    Point {
                        x: part_number.row - 1,
                        y,
                    },
                    Point {
                        x: part_number.row + 1,
                        y,
                    },
                ]
            })
            .collect::<Vec<Point>>();
        possible_part_positions.push(Point {
            x: part_number.row,
            y: part_number.start_col - 1,
        });
        possible_part_positions.push(Point {
            x: part_number.row,
            y: part_number.end_col,
        });

        possible_part_positions
    }

    fn is_gear_adjacent_to_two_parts(&self, gear: &Point) -> Option<u32> {
        let parts = self
            .part_numbers
            .iter()
            .filter(|part_number| {
                self.possible_part_positions(part_number)
                    .iter()
                    .any(|possible| possible == gear)
            })
            .map(|part_number| part_number.value)
            .collect::<Vec<_>>();

        if parts.len() == 2 {
            Some(parts[0] * parts[1])
        } else {
            None
        }
    }
}

impl BaseDay for Day3 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let parts = self
            .grid
            .cache()
            .iter()
            .filter(|(symbol, _)| {
                !['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(symbol)
            })
            .flat_map(|(_, positions)| positions)
            .collect::<HashSet<_>>();

        let parts_sum = self
            .part_numbers
            .iter()
            .filter(|part_number| {
                self.possible_part_positions(part_number)
                    .iter()
                    .any(|possible| parts.contains(possible))
            })
            .map(|part_number| part_number.value)
            .sum::<u32>();

        Ok(parts_sum.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .grid
            .cache()
            .iter()
            .filter(|(symbol, _)| **symbol == '*')
            .flat_map(|(_, positions)| positions)
            .filter_map(|gear| self.is_gear_adjacent_to_two_parts(gear))
            .sum::<u32>()
            .to_string())
    }

    fn setup(&mut self) {
        let input = self.read_file_into_vec_of_vec();
        let grid = Grid::from_vector(&input);
        self.part_numbers = self.part_numbers(&grid);
        self.grid = grid;
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = Day3::new();
    init_logger();
    common::file::download_input_file(2023, day.day_number).await?;

    let result = day.run_day()?;
    info!("Day {} - part 1: {:?}", result.day, result.part_1);
    info!("Day {} - part 2: {:?}", result.day, result.part_2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use common::file::get_data_dir;
    use rstest::rstest;

    #[rstest]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let mut day = Day3::new();
        day.file_path = get_data_dir(2023, 3).join("example_1.txt");
        day.setup();
        let expected = String::from("4361");

        assert_eq!(day.part_1()?, expected);

        Ok(())
    }

    #[rstest]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let mut day = Day3::new();
        day.file_path = get_data_dir(2023, 3).join("example_1.txt");
        day.setup();
        let expected = String::from("467835");

        assert_eq!(day.part_2()?, expected);

        Ok(())
    }
}
