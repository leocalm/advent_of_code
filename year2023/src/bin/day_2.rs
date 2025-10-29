use common::base_day::BaseDay;
use common::file::get_input_path;
use common::utils::init_logger;
use log::{info, warn};
use std::error::Error;
use std::path::PathBuf;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    game_id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.draws.iter().all(|draw| draw.is_valid())
    }

    fn power(&self) -> u32 {
        let max_red = self.draws.iter().map(|d| d.red_count).max().unwrap_or(0);
        let max_green = self.draws.iter().map(|d| d.green_count).max().unwrap_or(0);
        let max_blue = self.draws.iter().map(|d| d.blue_count).max().unwrap_or(0);

        max_red * max_green * max_blue
    }
}

struct Draw {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

impl Draw {
    fn is_valid(&self) -> bool {
        self.red_count <= MAX_RED && self.green_count <= MAX_GREEN && self.blue_count <= MAX_BLUE
    }
}

pub struct Day2 {
    day_number: u32,
    file_path: PathBuf,
}

impl Default for Day2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {
            day_number: 2,
            file_path: get_input_path(2023, 2),
        }
    }

    fn process_game_combination(&self, combination: &str) -> Result<Draw, Box<dyn Error>> {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for item in combination.split(", ") {
            let (count_str, color) = item.split_once(" ").unwrap();
            let count = count_str.parse::<u32>()?;
            match color {
                "red" => red_count += count,
                "green" => green_count += count,
                "blue" => blue_count += count,
                other => return Err(format!("Unexpected color '{}'", other).into()),
            }
        }

        Ok(Draw {
            red_count,
            green_count,
            blue_count,
        })
    }

    fn parse_game_line(&self, line: &str) -> Result<Game, Box<dyn Error>> {
        let Some((game_info, game_data)) = line.split_once(": ") else {
            warn!("Line '{}' is not valid, skipping.", line);
            return Err("Invalid line format".into());
        };
        let game_id = game_info
            .split_whitespace()
            .nth(1)
            .ok_or("Missing game id")?
            .parse()?;

        let game = Game {
            game_id,
            draws: game_data
                .split("; ")
                .map(|combination| self.process_game_combination(combination))
                .collect::<Result<Vec<Draw>, Box<dyn Error>>>()?,
        };

        Ok(game)
    }
}

impl BaseDay for Day2 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .read_file_into_vec()
            .iter()
            .filter_map(|line| self.parse_game_line(line).ok())
            .filter(|game| game.is_valid())
            .map(|game| game.game_id)
            .sum::<u32>()
            .to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .read_file_into_vec()
            .iter()
            .filter_map(|line| self.parse_game_line(line).ok())
            .map(|game| game.power())
            .sum::<u32>()
            .to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = Day2::new();
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
    use rstest::rstest;

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let expected = String::from("8");

        let mut day = Day2::new();
        day.file_path = get_data_dir(2023, 2).join("example_1.txt");

        let result = day.part_1()?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let expected = String::from("2286");

        let mut day = Day2::new();
        day.file_path = get_data_dir(2023, 2).join("example_1.txt");

        let result = day.part_2()?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[rstest]
    #[case((10, 10, 10), true)]
    #[case((40, 10, 10), false)]
    #[case((10, 40, 10), false)]
    #[case((10, 10, 40), false)]
    #[case((40, 40, 40), false)]
    fn is_valid_test(#[case] input: (u32, u32, u32), #[case] expected: bool) {
        let draw = Draw {
            red_count: input.0,
            green_count: input.1,
            blue_count: input.2,
        };
        assert_eq!(draw.is_valid(), expected);
    }
}
