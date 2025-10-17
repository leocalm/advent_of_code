use std::error::Error;
use std::path::PathBuf;
use common::base_day::BaseDay;
use common::graph::Graph;
use common::grid::{Grid, Point};
use common::dsu::DSU;
use common::file::get_input_path;
use common::utils::{add_corners, add_edges_to_graph};

const NODE_SYMBOL: char = '.';
const WALL_SYMBOL: char = '#';

pub struct Day18 {
    day_number: u32,
    file_path: PathBuf,
    grid_size: i32,
    bytes_to_consume: usize,
}

impl Day18 {
    pub fn new() -> Day18 {
        Day18 {
            day_number: 18,
            file_path: get_input_path(2024, 18),
            grid_size: 70,
            bytes_to_consume: 1_025,
        }
    }

    fn split_input(&self, line: &String) -> (i32, i32) {
        let _s = line.split(',').map(|x| x.parse::<i32>().unwrap()).rev().collect::<Vec<i32>>();
        (_s[0], _s[1])
    }
}

impl BaseDay for Day18 {
    fn get_day_number(&self) -> u32 { self.day_number }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let input = self.read_file_into_vec();
        let mut grid = Grid::new();

        for x in 0..=self.grid_size {
            for y in 0..=self.grid_size {
                grid.insert(Point { x, y }, '.');
            }
        }

        for line in &input[0..self.bytes_to_consume] {
            let (x, y) = self.split_input(&line);
            grid.insert(Point { x, y }, '#');
        }

        let mut graph = Graph::new();

        let start_node = graph.add_node(Point { x: 0, y: 0 });
        let end_node = graph.add_node(Point { x: self.grid_size, y: self.grid_size });

        add_corners(&grid, &mut graph, NODE_SYMBOL);
        add_edges_to_graph(&grid, &mut graph, WALL_SYMBOL);
        let (result, _) = graph.dijkstra(start_node);

        Ok(result.get(&end_node).unwrap().to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let input = self.read_file_into_vec();
        let mut walls = Vec::new();
        let mut nodes = vec![];
        let start_point = Point { x: 0, y: 0 };
        let end_point = Point { x: self.grid_size, y: self.grid_size };

        for line in &input {
            let (x, y) = self.split_input(&line);
            walls.push(Point { x, y });
        }

        for x in 0..=self.grid_size {
            for y in 0..=self.grid_size {
                if !walls.contains(&Point { x, y }) {
                    nodes.push(Point { x, y });
                }
            }
        }
        let mut dsu = DSU::new(&nodes);

        for x in 0..=self.grid_size {
            for y in 0..=self.grid_size {
                let current_node = Point { x, y };
                if dsu.find(current_node).is_none() {
                    continue;
                }
                for diffs in vec![(0, 1), (1, 0)] {
                    let other = Point { x: x + diffs.0, y: y + diffs.1 };
                    if dsu.find(other).is_some() {
                        dsu.union(current_node, other);
                    }
                }
            }
        }

        for &wall in walls.iter().rev() {
            dsu.add_new_node(wall);
            for x in 0..=self.grid_size {
                for y in 0..=self.grid_size {
                    let current_node = Point { x, y };
                    if dsu.find(current_node).is_none() {
                        continue;
                    }
                    for diffs in vec![(0, 1), (1, 0)] {
                        let other = Point { x: x + diffs.0, y: y + diffs.1 };
                        if dsu.find(other).is_some() {
                            dsu.union(current_node, other);
                        }
                    }
                }
            }

            if dsu.find(start_point) == dsu.find(end_point) {
                let result = format!("{},{}", wall.y, wall.x);
                return Ok(result);
            }
        }

        Ok(String::new())
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
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();
    }

    #[test]
    fn part_1_test() {
        init_logger();
        let expected = "22";

        let mut day = Day18::new();
        day.grid_size = 6;
        day.bytes_to_consume = 12;
        let _path = "./data/day_18/example_1.txt";
        let result = day.part_1();

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_2_test() {
        init_logger();
        let expected = "6,1";

        let mut day = Day18::new();
        day.grid_size = 6;
        day.bytes_to_consume = 12;
        let _path = "./data/day_18/example_1.txt";
        let result = day.part_2();

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_2_smaller_test() {
        init_logger();
        let expected = "0,1";

        let mut day = Day18::new();
        day.grid_size = 2;
        day.bytes_to_consume = 12;
        let _path = "./data/day_18/example_2.txt";
        let result = day.part_2();

        assert_eq!(expected, result.unwrap());
    }
}
                