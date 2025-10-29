use common::base_day::BaseDay;
use common::file::get_input_path;
use common::grid::{Grid, Point};
use std::error::Error;
use std::path::PathBuf;

pub struct Day6 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {
            day_number: 6,
            file_path: get_input_path(2024, 6),
        }
    }

    fn move_guard(
        &self,
        grid: &mut Grid<char>,
        start_position: Point,
        guard: char,
    ) -> Option<(Point, char)> {
        let diff: (i32, i32) = match guard {
            '^' => (-1, 0),
            'v' => (1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            _ => (0, 0),
        };

        let new_position = Point {
            x: start_position.x + diff.0,
            y: start_position.y + diff.1,
        };
        if new_position.x < 0
            || new_position.y < 0
            || new_position.x > grid.rows() - 1
            || new_position.y > grid.cols() - 1
        {
            None
        } else {
            let space = *grid.get(new_position).unwrap();

            if space == '#' || space == 'O' {
                let new_guard = match guard {
                    '^' => '>',
                    'v' => '<',
                    '>' => 'v',
                    '<' => '^',
                    _ => '^',
                };

                // return Some(((start_position.0, start_position.1), new_guard));
                return self.move_guard(grid, start_position, new_guard);
            }

            grid.update(new_position, 'X');
            Some((new_position, guard))
        }
    }
}

impl BaseDay for Day6 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut grid = Grid::from_vector(&self.read_file_into_vec_of_vec());
        let mut start_position = *grid.find('^').unwrap().0;
        let mut tmp = self.move_guard(&mut grid, start_position, '^');
        while tmp.is_some() {
            let (new_position, new_symbol) = tmp.unwrap();
            start_position = new_position;

            tmp = self.move_guard(&mut grid, start_position, new_symbol);
        }

        Ok(grid.count_values('X').to_string())
    }

    #[allow(dead_code)]
    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut tmp_grid = Grid::from_vector(&self.read_file_into_vec_of_vec());
        let mut start_position = *tmp_grid.find('^').unwrap().0;
        let mut tmp = self.move_guard(&mut tmp_grid, start_position, '^');
        while tmp.is_some() {
            let (new_position, new_symbol) = tmp.unwrap();
            start_position = new_position;

            tmp = self.move_guard(&mut tmp_grid, start_position, new_symbol);
        }

        let indexes = tmp_grid.filter('X');

        let mut result = 0;

        let mut grid = Grid::from_vector(&self.read_file_into_vec_of_vec());
        let start_position = *grid.find('^').unwrap().0;

        for point in indexes {
            let mut position_slow = start_position;
            let mut position_fast = start_position;
            if point == start_position {
                continue;
            }
            grid.update(point, 'O');

            let mut tmp_slow = self.move_guard(&mut grid, position_slow, '^');
            let mut tmp_fast = self.move_guard(&mut grid, position_fast, '^');
            tmp_fast = self.move_guard(&mut grid, tmp_fast.unwrap().0, tmp_fast.unwrap().1);

            while tmp_slow.is_some() && tmp_fast.is_some() {
                let (new_position_slow, new_symbol_slow) = tmp_slow.unwrap();
                let (new_position_fast, new_symbol_fast) = tmp_fast.unwrap();

                if new_position_slow == new_position_fast && new_symbol_slow == new_symbol_fast {
                    result += 1;
                    break;
                }

                position_slow = new_position_slow;
                tmp_slow = self.move_guard(&mut grid, position_slow, new_symbol_slow);

                position_fast = new_position_fast;
                tmp_fast = self.move_guard(&mut grid, position_fast, new_symbol_fast);
                if tmp_fast.is_some() {
                    tmp_fast = self.move_guard(&mut grid, tmp_fast.unwrap().0, tmp_fast.unwrap().1);
                }
            }

            grid = Grid::from_vector(&self.read_file_into_vec_of_vec());
        }

        Ok(result.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
