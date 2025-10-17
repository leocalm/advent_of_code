use std::error::Error;
use std::path::PathBuf;
use itertools::Itertools;
use common::base_day::BaseDay;
use common::file::get_input_path;

#[derive(Debug, Clone)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    fn from_input(input: &String) -> Robot {
        let re = regex::Regex::new(r"p=(\d+),(\d+) v=(-?)(\d+),(-?)(\d+)").unwrap();
        let captures = re.captures(input.as_str()).unwrap();
        let pos_x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let pos_y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let vel_x_signal = captures.get(3).unwrap().as_str();
        let mut vel_x = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
        let vel_y_signal = captures.get(5).unwrap().as_str();
        let mut vel_y = captures.get(6).unwrap().as_str().parse::<i64>().unwrap();

        if vel_x_signal.len() == 1 {
            vel_x = -vel_x;
        }

        if vel_y_signal.len() == 1 {
            vel_y = -vel_y;
        }

        Robot { position: (pos_y, pos_x), velocity: (vel_y, vel_x) }
    }
}

pub struct Day14 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 {
            day_number: 14,
            file_path: get_input_path(2024, 14),
        }
    }

    fn move_robot_without_loop(&self, robot: &Robot, number_of_steps: i64, grid_size: (i64, i64)) -> (i64, i64) {
        let mut new_position = (robot.position.0 + number_of_steps * robot.velocity.0, robot.position.1 + number_of_steps * robot.velocity.1);

        let rest_0 = new_position.0 % (grid_size.0);
        let rest_1 = new_position.1 % (grid_size.1);

        if rest_0 == 0 {
            new_position.0 = rest_0;
        } else if rest_0 > 0 {
            new_position.0 = rest_0;
        } else {
            new_position.0 = grid_size.0 + new_position.0 % grid_size.0;
        }

        if rest_1 == 0 {
            new_position.1 = rest_1;
        } else if rest_1 > 0 {
            new_position.1 = rest_1;
        } else {
            new_position.1 = grid_size.1 + new_position.1 % grid_size.1;
        }

        new_position
    }

    fn print_grid(&self, positions: &Vec<(i64, i64)>, grid_size: (i64, i64)) {
        for x in 0..grid_size.0 {
            for y in 0..grid_size.1 {
                let count = positions.into_iter().filter(|c| **c == (x, y)).count();
                if count > 0 {
                    print!("{}", count);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn number_of_robots_per_quadrant(&self, positions: &Vec<(i64, i64)>, grid_size: (i64, i64)) -> (u64, u64, u64, u64) {
        let mut first_quadrant = 0;
        let mut second_quadrant = 0;
        let mut third_quadrant = 0;
        let mut fourth_quadrant = 0;

        for x in 0..grid_size.0 {
            for y in 0..grid_size.1 {
                let count = positions.into_iter().filter(|c| **c == (x, y)).count();
                if x < grid_size.0 / 2 {
                    if y < grid_size.1 / 2 {
                        first_quadrant += count;
                    } else if y > grid_size.1 / 2 {
                        second_quadrant += count;
                    }
                } else if x > grid_size.0 / 2 {
                    if y < grid_size.1 / 2 {
                        third_quadrant += count;
                    } else if y > grid_size.1 / 2 {
                        fourth_quadrant += count;
                    }
                }
            }
        }

        (first_quadrant as u64, second_quadrant as u64, third_quadrant as u64, fourth_quadrant as u64)
    }

    fn count_touches(&self, positions: &Vec<(i64, i64)>) -> u64 {
        let mut result = 0;

        for p in positions {
            if positions.into_iter().contains(&(p.0 + 1, p.1))
                || positions.into_iter().contains(&(p.0, p.1 + 1))
                || positions.into_iter().contains(&(p.0 + 1, p.1 + 1))
                || positions.into_iter().contains(&(p.0 -1, p.1))
                || positions.into_iter().contains(&(p.0, p.1 - 1))
                || positions.into_iter().contains(&(p.0 -1, p.1- 1)) {
                result += 1;
            }
        }

        result
    }

}

impl BaseDay for Day14 {
    fn get_day_number(&self) -> u32 { self.day_number }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let grid_size = (103, 101);
        let count = 7916;
        let mut final_positions = Vec::new();

        let input = self.read_file_into_vec();
        for line in input {
            let robot = Robot::from_input(&line);
            let final_position = self.move_robot_without_loop(&robot, count, grid_size);
            final_positions.push(final_position);
        }

        self.print_grid(&final_positions, grid_size);

        let counts = self.number_of_robots_per_quadrant(&final_positions, grid_size);
        Ok((counts.0 * counts.1 * counts.2 * counts.3).to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let grid_size = (103, 101);
        let mut count = 6_098;
        let mut touches = Vec::new();
        let mut max_touches = (0, 0);

        while count <= 8_000 {
            let mut final_positions = Vec::new();
            let input = self.read_file_into_vec();
            for line in input {
                let robot = Robot::from_input(&line);
                let final_position = self.move_robot_without_loop(&robot, count, grid_size);
                final_positions.push(final_position);
            }

            let t = self.count_touches(&final_positions);
            if t > max_touches.1 {
                max_touches = (count, t);
                println!("max_touches: {:?}", max_touches);
            }
            touches.push(t);
            count += 101;
        }
        println!("max_touches: {:?}", max_touches);
        Ok(count.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
                