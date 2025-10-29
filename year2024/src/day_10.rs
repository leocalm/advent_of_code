use common::base_day::BaseDay;
use common::file::get_input_path;
use common::grid::{Grid, Point};
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq)]
struct Node {
    value: Point,
    children: Vec<Node>,
}

pub struct Day10 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 {
            day_number: 10,
            file_path: get_input_path(2024, 10),
        }
    }

    fn count_trails(&self, grid: &Grid<char>, trailhead: &Node) -> u64 {
        let mut result = 0;

        let value = *grid.get(trailhead.value).unwrap();
        if value == '9' {
            result += 1;
        }
        for child in &trailhead.children {
            result += self.count_trails(grid, child);
        }

        result
    }

    fn walk_trail(&mut self, grid: &Grid<char>, node: &mut Node, set: &mut HashSet<Point>) {
        let node_coordinates = node.value;
        let current_value = grid.get(node_coordinates).unwrap().to_digit(10).unwrap();
        if current_value == 9 {
            set.insert(node_coordinates);
            return;
        }

        let possible_moves = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for possible_move in possible_moves {
            let new_coordinates = node_coordinates.add(possible_move.0, possible_move.1);
            if new_coordinates.x >= 0
                && new_coordinates.y >= 0
                && new_coordinates.x < grid.rows()
                && new_coordinates.y < grid.cols()
            {
                let coordinates = new_coordinates;
                let value = grid.get(coordinates).unwrap();
                if value.to_digit(10).is_some_and(|x| x == current_value + 1) {
                    let mut new_node = Node {
                        value: coordinates,
                        children: Vec::new(),
                    };
                    self.walk_trail(grid, &mut new_node, set);
                    node.children.push(new_node);
                }
            }
        }
    }
}

impl BaseDay for Day10 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        let input = self.read_file_into_vec_of_vec();
        let grid = Grid::from_vector(&input);
        let trailheads = grid.filter('0');

        for trailhead in trailheads {
            let mut trailhead_node = Node {
                value: trailhead,
                children: vec![],
            };

            let mut count = HashSet::new();
            self.walk_trail(&grid, &mut trailhead_node, &mut count);
            result += count.len() as u64;
        }

        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        let input = self.read_file_into_vec_of_vec();
        let grid = Grid::from_vector(&input);
        let trailheads = grid.filter('0');

        for trailhead in trailheads {
            let mut trailhead_node = Node {
                value: trailhead,
                children: vec![],
            };

            let mut count = HashSet::new();
            self.walk_trail(&grid, &mut trailhead_node, &mut count);

            let trails_count = self.count_trails(&grid, &trailhead_node);
            result += trails_count;
        }

        Ok(result.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
