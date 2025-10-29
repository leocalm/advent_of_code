use common::base_day::BaseDay;
use common::file::get_input_path;
use common::grid::{Grid, Point};
use std::error::Error;
use std::path::PathBuf;

pub struct Day4 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {
            day_number: 4,
            file_path: get_input_path(2024, 4),
        }
    }

    fn xmas_vertical_top_to_bottom(&self, grid: &Grid<char>, point: Point) -> bool {
        grid.get(Point {
            x: point.x + 1,
            y: point.y,
        })
        .is_some_and(|c| *c == 'M')
            && grid
                .get(Point {
                    x: point.x + 2,
                    y: point.y,
                })
                .is_some_and(|c| *c == 'A')
            && grid
                .get(Point {
                    x: point.x + 3,
                    y: point.y,
                })
                .is_some_and(|c| *c == 'S')
    }

    fn xmas_vertical_bottom_to_top(&self, grid: &Grid<char>, point: Point) -> bool {
        if point.x < 3 {
            return false;
        }

        grid.get(Point {
            x: point.x - 1,
            y: point.y,
        })
        .is_some_and(|c| *c == 'M')
            && grid
                .get(Point {
                    x: point.x - 2,
                    y: point.y,
                })
                .is_some_and(|c| *c == 'A')
            && grid
                .get(Point {
                    x: point.x - 3,
                    y: point.y,
                })
                .is_some_and(|c| *c == 'S')
    }

    fn xmas_horizontal_left_to_right(&self, grid: &Grid<char>, point: Point) -> bool {
        grid.get(Point {
            x: point.x,
            y: point.y + 1,
        })
        .is_some_and(|c| *c == 'M')
            && grid
                .get(Point {
                    x: point.x,
                    y: point.y + 2,
                })
                .is_some_and(|c| *c == 'A')
            && grid
                .get(Point {
                    x: point.x,
                    y: point.y + 3,
                })
                .is_some_and(|c| *c == 'S')
    }

    fn xmas_horizontal_right_to_left(&self, grid: &Grid<char>, point: Point) -> bool {
        if point.y < 3 {
            return false;
        }

        grid.get(Point {
            x: point.x,
            y: point.y - 1,
        })
        .is_some_and(|c| *c == 'M')
            && grid
                .get(Point {
                    x: point.x,
                    y: point.y - 2,
                })
                .is_some_and(|c| *c == 'A')
            && grid
                .get(Point {
                    x: point.x,
                    y: point.y - 3,
                })
                .is_some_and(|c| *c == 'S')
    }

    fn xmas_diagonal_top_to_bottom_left_to_right(&self, grid: &Grid<char>, point: Point) -> bool {
        grid.get(Point {
            x: point.x + 1,
            y: point.y + 1,
        })
        .is_some_and(|c| *c == 'M')
            && grid
                .get(Point {
                    x: point.x + 2,
                    y: point.y + 2,
                })
                .is_some_and(|c| *c == 'A')
            && grid
                .get(Point {
                    x: point.x + 3,
                    y: point.y + 3,
                })
                .is_some_and(|c| *c == 'S')
    }

    fn xmas_diagonal_top_to_bottom_right_to_left(&self, grid: &Grid<char>, point: Point) -> bool {
        if point.y < 3 {
            return false;
        }

        grid.get(Point {
            x: point.x + 1,
            y: point.y - 1,
        })
        .is_some_and(|c| *c == 'M')
            && grid
                .get(Point {
                    x: point.x + 2,
                    y: point.y - 2,
                })
                .is_some_and(|c| *c == 'A')
            && grid
                .get(Point {
                    x: point.x + 3,
                    y: point.y - 3,
                })
                .is_some_and(|c| *c == 'S')
    }

    fn xmas_diagonal_bottom_to_top_left_to_right(&self, grid: &Grid<char>, point: Point) -> bool {
        if point.x < 3 {
            return false;
        }

        grid.get(Point {
            x: point.x - 1,
            y: point.y + 1,
        })
        .is_some_and(|c| *c == 'M')
            && grid
                .get(Point {
                    x: point.x - 2,
                    y: point.y + 2,
                })
                .is_some_and(|c| *c == 'A')
            && grid
                .get(Point {
                    x: point.x - 3,
                    y: point.y + 3,
                })
                .is_some_and(|c| *c == 'S')
    }

    fn xmas_diagonal_bottom_to_top_right_to_left(&self, grid: &Grid<char>, point: Point) -> bool {
        if point.x < 3 || point.y < 3 {
            return false;
        }

        grid.get(Point {
            x: point.x - 1,
            y: point.y - 1,
        })
        .is_some_and(|c| *c == 'M')
            && grid
                .get(Point {
                    x: point.x - 2,
                    y: point.y - 2,
                })
                .is_some_and(|c| *c == 'A')
            && grid
                .get(Point {
                    x: point.x - 3,
                    y: point.y - 3,
                })
                .is_some_and(|c| *c == 'S')
    }

    fn is_second_diagonal_x_mask(&self, grid: &Grid<char>, point: Point) -> bool {
        if grid
            .get(Point {
                x: point.x - 1,
                y: point.y + 1,
            })
            .is_some_and(|c| *c == 'M')
        {
            if grid
                .get(Point {
                    x: point.x + 1,
                    y: point.y - 1,
                })
                .is_some_and(|c| *c == 'S')
            {
                return true;
            }
        }
        if grid
            .get(Point {
                x: point.x - 1,
                y: point.y + 1,
            })
            .is_some_and(|c| *c == 'S')
        {
            if grid
                .get(Point {
                    x: point.x + 1,
                    y: point.y - 1,
                })
                .is_some_and(|c| *c == 'M')
            {
                return true;
            }
        }

        false
    }

    fn is_x_mas(&self, grid: &Grid<char>, point: Point) -> bool {
        if point.x < 1 || point.y < 1 {
            return false;
        }

        if grid
            .get(Point {
                x: point.x - 1,
                y: point.y - 1,
            })
            .is_some_and(|c| *c == 'M')
        {
            if grid
                .get(Point {
                    x: point.x + 1,
                    y: point.y + 1,
                })
                .is_some_and(|c| *c == 'S')
            {
                return self.is_second_diagonal_x_mask(grid, point);
            }
        }

        if grid
            .get(Point {
                x: point.x - 1,
                y: point.y - 1,
            })
            .is_some_and(|c| *c == 'S')
        {
            if grid
                .get(Point {
                    x: point.x + 1,
                    y: point.y + 1,
                })
                .is_some_and(|c| *c == 'M')
            {
                return self.is_second_diagonal_x_mask(grid, point);
            }
        }

        false
    }
}

impl BaseDay for Day4 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        let input = self.read_file_into_vec_of_vec();
        let grid = Grid::from_vector(&input);

        for row in 0..input.len() {
            for col in 0..input[row].len() {
                let point = Point {
                    x: row as i32,
                    y: col as i32,
                };
                if grid.get(point).is_some() && *grid.get(point).unwrap() == 'X' {
                    if self.xmas_vertical_top_to_bottom(&grid, point) {
                        result += 1;
                    }
                    if self.xmas_vertical_bottom_to_top(&grid, point) {
                        result += 1;
                    }
                    if self.xmas_horizontal_left_to_right(&grid, point) {
                        result += 1;
                    }
                    if self.xmas_horizontal_right_to_left(&grid, point) {
                        result += 1;
                    }
                    if self.xmas_diagonal_top_to_bottom_left_to_right(&grid, point) {
                        result += 1;
                    }
                    if self.xmas_diagonal_top_to_bottom_right_to_left(&grid, point) {
                        result += 1;
                    }
                    if self.xmas_diagonal_bottom_to_top_left_to_right(&grid, point) {
                        result += 1;
                    }
                    if self.xmas_diagonal_bottom_to_top_right_to_left(&grid, point) {
                        result += 1;
                    }
                }
            }
        }

        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        let input = self.read_file_into_vec_of_vec();
        let grid = Grid::from_vector(&input);

        for row in 0..input.len() {
            for col in 0..input[row].len() {
                let point = Point {
                    x: row as i32,
                    y: col as i32,
                };
                if grid.get(point).is_some() && *grid.get(point).unwrap() == 'A' {
                    if self.is_x_mas(&grid, point) {
                        result += 1;
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
