use common::base_day::BaseDay;
use common::file::get_input_path;
use common::graph::{Graph, Node};
use common::grid::{Grid, Point};
use common::utils::{DIFFS, bfs_distances};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;

const NODE_SYMBOL: char = '.';
const WALL_SYMBOL: char = '#';
const START_SYMBOL: char = 'S';
const END_SYMBOL: char = 'E';
const PAIRS: [((i32, i32), (i32, i32)); 2] = [((0, 1), (0, -1)), ((1, 0), (-1, 0))];
const POSSIBLE_CHARS: [char; 3] = [NODE_SYMBOL, START_SYMBOL, END_SYMBOL];

pub struct Day20 {
    day_number: u32,
    file_path: PathBuf,
    graph: Graph<Point>,
    start_point: u32,
    end_point: u32,
    costs_from_start: HashMap<u32, u64>,
    costs_from_end: HashMap<u32, u64>,
    original_cost: u64,
    grid: Grid<char>,
    min_saved_cost: u64,
}

#[allow(dead_code)]
impl Day20 {
    pub fn new() -> Day20 {
        Day20 {
            day_number: 20,
            file_path: get_input_path(2024, 20),
            graph: Graph::new(),
            start_point: u32::MAX,
            end_point: u32::MAX,
            costs_from_start: HashMap::new(),
            costs_from_end: HashMap::new(),
            original_cost: 0,
            grid: Grid::new(),
            min_saved_cost: 100,
        }
    }

    fn get_walls(&self) -> Vec<Point> {
        self.grid.filter(WALL_SYMBOL)
    }

    fn build_graph(&mut self, input: &[Vec<char>]) {
        for (x, row) in input.iter().enumerate() {
            for (y, &value) in row.iter().enumerate() {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };

                if value == NODE_SYMBOL {
                    self.graph.add_node(point);
                } else if value == START_SYMBOL {
                    self.start_point = self.graph.add_node(point);
                } else if value == END_SYMBOL {
                    self.end_point = self.graph.add_node(point);
                }
            }
        }

        let nodes = self
            .graph
            .get_nodes()
            .iter()
            .map(|(&id, &node)| (id, node.value))
            .collect::<Vec<(u32, Point)>>();

        for (node_id, point) in nodes {
            for diff in DIFFS {
                let neighbour = point.add_tuple(diff);
                if let Some(neighbor_id) = self.graph.get_node_id(Node { value: neighbour }) {
                    self.graph.add_edge(node_id, neighbor_id, 1);
                }
            }
        }

        let (result, _) = self.graph.dijkstra(self.start_point);
        self.costs_from_start = result;
        self.original_cost = *self
            .costs_from_start
            .get(&self.end_point)
            .expect("Path from start to end not found");

        let (result, _) = self.graph.dijkstra(self.end_point);
        self.costs_from_end = result;
    }

    fn calculate_cost_for_neighbours(&self, n1: Point, n2: Point) -> (u64, u64) {
        let neighbour_1 = self
            .graph
            .get_node_id(Node { value: n1 })
            .expect("Node n1 exists in grid but not in graph");

        let neighbour_2 = self
            .graph
            .get_node_id(Node { value: n2 })
            .expect("Node n2 exists in grid but not in graph");

        let mut cost_1_from_start = *self
            .costs_from_start
            .get(&neighbour_1)
            .expect("Cost for neighbour 1 not found");

        let cost_2_from_start = *self
            .costs_from_start
            .get(&neighbour_2)
            .expect("Cost for neighbour 1 not found");

        let mut cost_2_from_end = *self
            .costs_from_end
            .get(&neighbour_2)
            .expect("Cost for neighbour 2 not found");

        if cost_2_from_start < cost_1_from_start {
            cost_1_from_start = *self
                .costs_from_end
                .get(&neighbour_1)
                .expect("Cost for neighbour 1 not found");

            cost_2_from_end = *self
                .costs_from_start
                .get(&neighbour_2)
                .expect("Cost for neighbour 2 not found");
        }

        (cost_1_from_start, cost_2_from_end)
    }

    fn calculate_cost_for_cheat(&self, wall: Point) -> Option<u64> {
        for pair in &PAIRS {
            let n1 = wall.add(pair.0.0, pair.0.1);
            let n2 = wall.add(pair.1.0, pair.1.1);

            let is_n1_valid = self
                .grid
                .get(n1)
                .is_some_and(|&n| POSSIBLE_CHARS.contains(&n));

            let is_n2_valid = self
                .grid
                .get(n2)
                .is_some_and(|&n| POSSIBLE_CHARS.contains(&n));

            if is_n1_valid && is_n2_valid {
                let (cost_1_from_start, cost_2_from_end) =
                    self.calculate_cost_for_neighbours(n1, n2);

                return Some(cost_1_from_start + cost_2_from_end + 2);
            }
        }
        None
    }

    fn part_2_v1(&mut self) -> u64 {
        let mut result = 0;

        let nodes: HashSet<Point> = HashSet::from_iter(self.grid.filter_contains(&POSSIBLE_CHARS));
        let radius: i32 = 20;
        for node_1 in nodes.iter() {
            for dx in -radius..=radius {
                let max_dy = radius - dx.abs();
                for dy in -max_dy..=max_dy {
                    let node_2 = node_1.add(dx, dy);

                    if node_2.x < node_1.x || (node_2.x == node_1.x && node_2.y <= node_1.y) {
                        continue;
                    }

                    if nodes.contains(&node_2) {
                        let manhattan_distance = node_1.manhattan_distance(node_2);
                        let (cost_1_from_start, cost_2_from_end) =
                            self.calculate_cost_for_neighbours(*node_1, node_2);
                        let cheat_cost =
                            cost_1_from_start + manhattan_distance as u64 + cost_2_from_end;
                        let diff = self.original_cost.saturating_sub(cheat_cost);
                        if diff >= self.min_saved_cost {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }

    fn part_2_v2(&mut self) -> u64 {
        let mut result = 0;

        let input = self.read_file_into_vec_of_vec();
        let grid = Grid::from_vector(&input);
        let rows = grid.rows();
        let cols = grid.cols();

        let &start = grid.find(START_SYMBOL).unwrap().0;
        let &end = grid.find(END_SYMBOL).unwrap().0;

        let ds = bfs_distances(&input, start, WALL_SYMBOL);
        let de = bfs_distances(&input, end, WALL_SYMBOL);
        let original_cost = ds[end.x as usize][end.y as usize];

        let mut offsets = Vec::new();
        let radius: i32 = 20;

        for dx in -radius..=radius {
            let max_dy = radius - dx.abs();
            for dy in -max_dy..=max_dy {
                offsets.push((dx, dy));
            }
        }

        let walkable: HashSet<Point> =
            HashSet::from_iter(self.grid.filter_contains(&POSSIBLE_CHARS));
        for a in walkable.iter() {
            for (dx, dy) in offsets.iter() {
                let b = a.add(*dx, *dy);
                if b.x < 0 || b.y < 0 || b.x >= rows || b.y >= cols {
                    continue;
                }
                if walkable.contains(&b) && de[b.x as usize][b.y as usize] >= 0 {
                    let new_cost = ds[a.x as usize][a.y as usize]
                        + dx.abs()
                        + dy.abs()
                        + de[b.x as usize][b.y as usize];
                    if original_cost - new_cost >= self.min_saved_cost as i32 {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}

impl BaseDay for Day20 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result = 0;

        for wall in self.get_walls() {
            if let Some(new_cost) = self.calculate_cost_for_cheat(wall) {
                let diff = self.original_cost.saturating_sub(new_cost);

                if diff >= self.min_saved_cost {
                    result += 1;
                }
            }
        }
        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let result = self.part_2_v1();
        // let result = self.part_2_v2(&path);

        Ok(result.to_string())
    }

    fn setup(&mut self) {
        let input = self.read_file_into_vec_of_vec();

        self.build_graph(&input);
        self.grid = Grid::from_vector(&input);
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

        let mut day = Day20::new();
        day.min_saved_cost = 0;
        let _path = "./data/day_20/example_1.txt".to_string();
        let expected = "44";

        day.setup();
        let result = day.part_1().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn part_2_test() {
        init_logger();

        let mut day = Day20::new();
        day.min_saved_cost = 50;
        let path = "./data/day_20/example_1.txt".to_string();
        let expected = "285";

        day.setup();
        let result = day.part_2().unwrap();

        assert_eq!(result, expected);
    }
}
