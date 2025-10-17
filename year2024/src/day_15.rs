use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use common::grid::{Grid, Point};
use log::debug;
use common::base_day::BaseDay;
use common::file::get_input_path;

pub struct Day15 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day15 {
    pub fn new() -> Day15 {
        Day15 {
            day_number: 15,
            file_path: get_input_path(2024, 15),
        }
    }

    fn read_input(&self, input: Vec<Vec<char>>) -> (Grid<char>, Vec<char>) {
        let mut map = Vec::new();
        let mut instructions = Vec::new();

        for line in input {
            if !line.is_empty() {
                if *line.get(0).unwrap() == '#' {
                    // Map
                    map.push(line);
                } else {
                    // Instructions
                    instructions.extend(line);
                }
            }
        }

        let grid = Grid::from_vector(&map);
        (grid, instructions)
    }

    fn move_object_2(&self, grid: &mut Grid<char>, current_position: Point, instruction: char, walls: &Vec<Point>) -> HashMap<Point, char> {
        debug!("current_position: {:?}, instruction {:?}", current_position, instruction);
        let mut operations = HashMap::new();

        let current_symbol = *grid.get(current_position).unwrap();
        let mut position = current_position;
        let diff = match instruction {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => (0, 0)
        };
        position = position.add(diff.0, diff.1);
        let value = *grid.get(position).unwrap();
        debug!("New Position: {:?}, value: {:?}", position, value);

        if walls.contains(&position) {
            debug!("Hit a wall at {:?}, not moving!", position);
            return HashMap::new();
        }
        if value == '[' {
            if walls.contains(&position.add(0, 1)) {
                debug!("Hit a wall at {:?}, not moving!", position);
                return HashMap::new();
            }
        } else if value == ']' {
            if walls.contains(&position.add(0, -1)) {
                debug!("Hit a wall at {:?}, not moving!", position);
                return HashMap::new();
            }
        }

        if value == 'O' {
            debug!("Hit a box at {:?}", position);
            let _operations = self.move_object_2(grid, position, instruction, walls);
            if _operations.is_empty() {
                debug!("Hit a wall at {:?}, not moving!", position);
                return HashMap::new();
            }
            operations.extend(_operations);
        }

        if value == ']' || value == '[' {
            debug!("Hit a double box at {:?}", position);

            if instruction == '>' || instruction == '<' {
                let _operations = self.move_object_2(grid, position, instruction, walls);

                if _operations.is_empty() {
                    debug!("Hit a wall at {:?}, not moving!", position);
                    return HashMap::new();
                }
                operations.extend(_operations);
            } else {
                // Need to move the full box
                let operations_1 = self.move_object_2(grid, position, instruction, walls);

                if operations_1.is_empty() {
                    debug!("Hit a wall at position_1 {:?}, not moving!", position);
                    return HashMap::new();
                }

                let mut _position = position;
                if value == ']' {
                    _position = position.add(0, -1);
                } else {
                    _position = position.add(0, 1);
                }

                let operations_2 = self.move_object_2(grid, _position, instruction, walls);

                if operations_2.is_empty() {
                    debug!("Hit a wall at position_2 {:?}, not moving!", position);
                    return HashMap::new();
                }

                debug!("operations_1, {:?}, operations_2: {:?}", operations_1, operations_2);
                if operations_1.is_empty() || operations_2.is_empty() {
                    debug!("Hit a wall at position_2 {:?}, not moving!", position);
                    return HashMap::new();
                }

                operations.extend(&operations_1);
                operations.extend(&operations_2);

                for (k_1, v_1) in operations_1.iter() {
                    for (k_2, v_2) in operations_2.iter() {
                        if k_1 == k_2 {
                            debug!("k_1: {:?}, v1:{:?}, k_2: {:?}, v2: {:?}", k_1, operations_1.get(k_1), k_2, operations_2.get(k_2));
                            if *v_1 != *v_2 {
                                if *v_1 == '.' {
                                    operations.insert(*k_1, *v_2);
                                }

                                if *v_2 == '.' {
                                    operations.insert(*k_2, *v_1);
                                }
                            }

                        }
                    }
                }
            }
        }

        operations.insert(current_position, '.');
        operations.insert(position, current_symbol);

        operations
    }

    fn duplicate_width(&self, grid: &Grid<char>) -> Grid<char> {
        let mut adjusted_grid = Grid::new();

        for row in 0..grid.rows() {
            let mut adjusted_col = 0;
            for col in 0..grid.cols() {
                let value = *grid.get(Point { x: row, y: col }).unwrap();
                match value {
                    '#' => {
                        adjusted_grid.insert(Point { x: row, y: adjusted_col }, '#');
                        adjusted_col += 1;
                        adjusted_grid.insert(Point { x: row, y: adjusted_col }, '#');
                        adjusted_col += 1;
                    },
                    'O' => {
                        adjusted_grid.insert(Point { x: row, y: adjusted_col }, '[');
                        adjusted_col += 1;
                        adjusted_grid.insert(Point { x: row, y: adjusted_col }, ']');
                        adjusted_col += 1;
                    },
                    '.' => {
                        adjusted_grid.insert(Point { x: row, y: adjusted_col }, '.');
                        adjusted_col += 1;
                        adjusted_grid.insert(Point { x: row, y: adjusted_col }, '.');
                        adjusted_col += 1;
                    },
                    '@' => {
                        adjusted_grid.insert(Point { x: row, y: adjusted_col }, '@');
                        adjusted_col += 1;
                        adjusted_grid.insert(Point { x: row, y: adjusted_col }, '.');
                        adjusted_col += 1;
                    },
                    _ => unreachable!()
                }
            }
        }

        adjusted_grid
    }

    fn sum_of_gps_coordinates(&self, grid: &Grid<char>, box_identifier: char) -> u64 {
        let boxes = grid.filter(box_identifier);
        boxes.iter().fold(0, |acc, b| acc + (100 * b.x) as u64 + b.y as u64)
    }

    fn follow_instructions(&self, grid: &mut Grid<char>, instructions: Vec<char>, box_identifier: char) -> u64 {
        let mut position = *grid.find('@').unwrap().0;
        let walls = grid.filter('#').iter().map(|x| *x ).collect::<Vec<Point>>();

        for instruction in instructions {
            let operations = self.move_object_2(grid, position, instruction, &walls);
            for operation in operations {
                grid.update(operation.0, operation.1);
                if operation.1 == '@' {
                    position = operation.0;
                }
            }
            grid.print();
        }

        self.sum_of_gps_coordinates(&grid, box_identifier)
    }
}

impl BaseDay for Day15 {
    fn get_day_number(&self) -> u32 { self.day_number }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let input = self.read_file_into_vec_of_vec();
        let (mut grid, instructions) = self.read_input(input);
        Ok(self.follow_instructions(&mut grid, instructions, 'O').to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let input = self.read_file_into_vec_of_vec();
        let (_grid, instructions) = self.read_input(input);
        let mut adjusted_grid = self.duplicate_width(&_grid);

        Ok(self.follow_instructions(&mut adjusted_grid, instructions, '[').to_string())
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
            .filter_level(log::LevelFilter::max())
            .is_test(true)
            .try_init();
    }

    #[test]
    fn part_1_test_big_example() {
        init_logger();

        let expected = "10092";
        let mut day = Day15::new();
        day.file_path = PathBuf::from("./data/day_15/big_example.txt");

        let result = day.part_1();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_1_test_small_example() {
        init_logger();

        let expected = "2028";
        let mut day = Day15::new();
        day.file_path = PathBuf::from("./data/day_15/small_example.txt");

        let result = day.part_1();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_2_test_small_example() {
        init_logger();

        let expected = "618";
        let mut day = Day15::new();
        day.file_path = PathBuf::from("./data/day_15/small_example_part_2.txt");

        let result = day.part_2();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_2_test_small_example_1() {
        init_logger();

        let expected = "406";
        let mut day = Day15::new();
        day.file_path = PathBuf::from("./data/day_15/small_example_part_2_1.txt");

        let result = day.part_2();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_2_test_small_example_2() {
        init_logger();

        let expected = "509";
        let mut day = Day15::new();
        day.file_path = PathBuf::from("./data/day_15/small_example_part_2_2.txt");

        let result = day.part_2();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_2_test_big_example() {
        init_logger();

        let expected = "9021";
        let mut day = Day15::new();
        day.file_path = PathBuf::from("./data/day_15/big_example.txt");

        let result = day.part_2();
        assert_eq!(expected, result.unwrap());
    }
}
                