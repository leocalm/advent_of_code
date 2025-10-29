use common::base_day::BaseDay;
use common::file::get_input_path;
use common::graph::Graph;
use common::grid::{Grid, Point};
use common::utils::{add_corners, add_edges_to_graph, rebuild_path_counting_nodes};
use std::error::Error;
use std::path::PathBuf;

const START_SYMBOL: char = 'S';
const END_SYMBOL: char = 'E';
const NODE_SYMBOL: char = '.';
const WALL_SYMBOL: char = '#';

pub struct Day16 {
    day_number: u32,
    file_path: PathBuf,
    part_1_result: Option<u64>,
    part_2_result: Option<u64>,
}

impl Day16 {
    pub fn new() -> Day16 {
        Day16 {
            day_number: 16,
            file_path: get_input_path(2024, 16),
            part_1_result: None,
            part_2_result: None,
        }
    }

    fn run(&self) -> (u64, u64) {
        let input = self.read_file_into_vec_of_vec();
        let grid = Grid::from_vector(&input);

        let starting_point = *grid.find(START_SYMBOL).unwrap().0;
        let finishing_point = *grid.find(END_SYMBOL).unwrap().0;

        let mut graph: Graph<Point> = Graph::new();

        let start_node = graph.add_node(starting_point);
        let end_node = graph.add_node(finishing_point);
        add_corners(&grid, &mut graph, NODE_SYMBOL);
        add_edges_to_graph(&grid, &mut graph, WALL_SYMBOL);
        let (result, predecessors) = graph.dijkstra(start_node);

        let unique_points =
            rebuild_path_counting_nodes(&graph, &predecessors, start_node, end_node);

        (*result.get(&end_node).unwrap(), unique_points.len() as u64)
    }
}

impl BaseDay for Day16 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.part_1_result.unwrap().to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.part_2_result.unwrap().to_string())
    }

    fn setup(&mut self) -> () {
        let (part_1, part_2) = self.run();
        self.part_1_result = Some(part_1);
        self.part_2_result = Some(part_2);
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
    fn part_1_example_1_test() {
        init_logger();
        let _path = "./data/day_16/example_1.txt";
        let expected = (7036, 45);
        let day = Day16::new();
        let result = day.run();

        assert_eq!(expected, result);
    }

    #[test]
    fn part_1_example_2_test() {
        init_logger();
        let _path = "./data/day_16/example_2.txt";
        let expected = (11048, 64);
        let day = Day16::new();
        let result = day.run();

        assert_eq!(expected, result);
    }

    #[test]
    fn part_1_example_3_test() {
        init_logger();
        let _path = "./data/day_16/example_3.txt";
        let expected = "1006";
        let mut day = Day16::new();
        let result = day.part_1();

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_1_example_4_test() {
        init_logger();
        let _path = "./data/day_16/example_4.txt";
        let expected = "1004";
        let mut day = Day16::new();
        let result = day.part_1();

        assert_eq!(expected, result.unwrap());
    }
}
