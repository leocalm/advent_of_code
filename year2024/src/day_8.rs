use common::base_day::BaseDay;
use common::file::get_input_path;
use common::grid::{Grid, Point};
use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;

pub struct Day8 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 {
            day_number: 8,
            file_path: get_input_path(2024, 8),
        }
    }

    fn get_antinodes_for_antenna_pair(
        &self,
        antenna_1: Point,
        antenna_2: Point,
        rows: i32,
        cols: i32,
    ) -> (Option<Point>, Option<Point>) {
        let antinode_1;
        let antinode_2;

        if antenna_1.x < antenna_2.x {
            let diff_x = antenna_2.x - antenna_1.x;
            if antenna_1.y < antenna_2.y {
                let diff_y = antenna_2.y - antenna_1.y;
                if antenna_1.x < diff_x || antenna_1.y < diff_y {
                    antinode_1 = None;
                } else {
                    antinode_1 = Some(Point {
                        x: antenna_1.x - diff_x,
                        y: antenna_1.y - diff_y,
                    });
                }

                if antenna_2.x + diff_x >= rows || antenna_2.y + diff_y >= cols {
                    antinode_2 = None;
                } else {
                    antinode_2 = Some(Point {
                        x: antenna_2.x + diff_x,
                        y: antenna_2.y + diff_y,
                    });
                }
            } else {
                let diff_y = antenna_1.y - antenna_2.y;
                if antenna_1.x < diff_x || antenna_1.y + diff_y >= cols {
                    antinode_1 = None;
                } else {
                    antinode_1 = Some(Point {
                        x: antenna_1.x - diff_x,
                        y: antenna_1.y + diff_y,
                    });
                }

                if antenna_2.y < diff_y || antenna_2.x + diff_x >= rows {
                    antinode_2 = None;
                } else {
                    antinode_2 = Some(Point {
                        x: antenna_2.x + diff_x,
                        y: antenna_2.y - diff_y,
                    });
                }
            }
        } else {
            let diff_x = antenna_1.x - antenna_2.x;
            if antenna_1.y < antenna_2.y {
                let diff_y = antenna_2.y - antenna_1.y;

                if antenna_1.y < diff_y || antenna_1.x + diff_x >= rows {
                    antinode_1 = None;
                } else {
                    antinode_1 = Some(Point {
                        x: antenna_1.x + diff_x,
                        y: antenna_1.y - diff_y,
                    });
                }

                if antenna_2.x < diff_x || antenna_2.y + diff_y >= cols {
                    antinode_2 = None;
                } else {
                    antinode_2 = Some(Point {
                        x: antenna_2.x - diff_x,
                        y: antenna_2.y + diff_y,
                    });
                }
            } else {
                let diff_y = antenna_1.y - antenna_2.y;
                if antenna_2.x < diff_x || antenna_2.y < diff_y {
                    antinode_2 = None;
                } else {
                    antinode_2 = Some(Point {
                        x: antenna_2.x - diff_x,
                        y: antenna_2.y - diff_y,
                    });
                }

                if antenna_1.x + diff_x >= rows || antenna_1.y + diff_y >= cols {
                    antinode_1 = None;
                } else {
                    antinode_1 = Some(Point {
                        x: antenna_1.x + diff_x,
                        y: antenna_1.y + diff_y,
                    });
                }
            }
        }

        (antinode_1, antinode_2)
    }

    fn get_antinodes_in_line(
        &self,
        antenna_1: Point,
        antenna_2: Point,
        rows: i32,
        cols: i32,
    ) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        let diff_x = antenna_1.x - antenna_2.x;
        let diff_y = antenna_1.y - antenna_2.y;

        let mut current_node = Point {
            x: antenna_1.x,
            y: antenna_1.y,
        };
        while current_node.x >= 0
            && current_node.y >= 0
            && current_node.x < rows
            && current_node.y < cols
        {
            antinodes.insert(Point {
                x: current_node.x,
                y: current_node.y,
            });
            current_node = Point {
                x: current_node.x - diff_x,
                y: current_node.y - diff_y,
            };
        }

        current_node = Point {
            x: antenna_1.x,
            y: antenna_1.y,
        };
        while current_node.x >= 0
            && current_node.y >= 0
            && current_node.x < rows
            && current_node.y < cols
        {
            antinodes.insert(Point {
                x: current_node.x,
                y: current_node.y,
            });
            current_node = Point {
                x: current_node.x + diff_x,
                y: current_node.y + diff_y,
            };
        }

        antinodes
    }
}

impl BaseDay for Day8 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let input = self.read_file_into_vec_of_vec();
        let grid = Grid::from_vector(&input);

        let mut set_of_antinodes = HashSet::new();
        for (_, antennas_for_key) in grid.filter_different_than('.').iter() {
            let pairs_of_antennas = antennas_for_key.iter().combinations(2).collect::<Vec<_>>();

            for pair in pairs_of_antennas {
                let antinodes = self.get_antinodes_for_antenna_pair(
                    *pair[0],
                    *pair[1],
                    grid.rows(),
                    grid.cols(),
                );
                if antinodes.0.is_some() {
                    set_of_antinodes.insert(antinodes.0.unwrap());
                }

                if antinodes.1.is_some() {
                    set_of_antinodes.insert(antinodes.1.unwrap());
                }
            }
        }

        Ok(set_of_antinodes.len().to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let grid = Grid::from_vector(&self.read_file_into_vec_of_vec());

        let mut set_of_antinodes: HashSet<Point> = HashSet::new();
        for (_, antennas_for_key) in grid.filter_different_than('.').iter() {
            let pairs_of_antennas = antennas_for_key.iter().combinations(2).collect::<Vec<_>>();

            for pair in pairs_of_antennas {
                set_of_antinodes = set_of_antinodes
                    .union(&self.get_antinodes_in_line(
                        *pair[0],
                        *pair[1],
                        grid.rows(),
                        grid.cols(),
                    ))
                    .copied()
                    .collect();
            }
        }

        Ok(set_of_antinodes.len().to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
