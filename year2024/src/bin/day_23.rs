use common::base_day::BaseDay;
use common::file::get_input_path;
use common::graph::Graph;
use common::test_utils::init_logger;
use itertools::Itertools;
use log::info;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;

pub struct Day23 {
    day_number: u32,
    file_path: PathBuf,
    graph: Graph<[char; 2]>,
}

impl Day23 {
    pub fn new() -> Day23 {
        Day23 {
            day_number: 23,
            file_path: get_input_path(2024, 23),
            graph: Graph::new(),
        }
    }
}

impl BaseDay for Day23 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result = 0;

        for group_of_three in self.graph.get_nodes().iter().combinations(3) {
            let (n1_id, node_1) = group_of_three.iter().nth(0).unwrap();
            let (n2_id, node_2) = group_of_three.iter().nth(1).unwrap();
            let (n3_id, node_3) = group_of_three.iter().nth(2).unwrap();

            let n1 = self.graph.simple_edges.get(n1_id).unwrap();
            let n2 = self.graph.simple_edges.get(n2_id).unwrap();
            let n3 = self.graph.simple_edges.get(n3_id).unwrap();

            if n2.contains(&n1_id)
                && n3.contains(&n1_id)
                && n1.contains(&n2_id)
                && n3.contains(&n2_id)
                && n1.contains(&n3_id)
                && n2.contains(&n3_id)
            {
                if node_1.value[0] == 't' || node_2.value[0] == 't' || node_3.value[0] == 't' {
                    result += 1;
                }
            }
        }
        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut computer_map = HashMap::new();
        let mut computer_key = HashMap::new();
        let mut count = 0;

        let mut r = HashSet::new();
        let mut p = HashSet::new();
        let mut g = HashMap::new();
        let mut x = HashSet::new();
        let mut clique = Vec::new();

        let input = self.read_file_into_vec();
        for line in input.iter() {
            let (computer_1, computer_2) = line.split_once('-').unwrap();

            let computer_1_key = if computer_map.contains_key(computer_1) {
                computer_map[computer_1]
            } else {
                let computer_1_key = count;
                count += 1;
                computer_map.insert(computer_1, computer_1_key);
                computer_key.insert(computer_1_key, computer_1);
                computer_1_key
            };

            let computer_2_key = if computer_map.contains_key(computer_2) {
                computer_map[computer_2]
            } else {
                let computer_2_key = count;
                count += 1;
                computer_map.insert(computer_2, computer_2_key);
                computer_key.insert(computer_2_key, computer_2);
                computer_2_key
            };

            g.entry(computer_1_key)
                .or_insert_with(HashSet::new)
                .insert(computer_2_key);
            g.entry(computer_2_key)
                .or_insert_with(HashSet::new)
                .insert(computer_1_key);

            p.insert(computer_1_key);
            p.insert(computer_2_key);
        }

        common::utils::bron_kerbosch(&mut r, &mut p, &mut x, &mut g, &mut clique);
        let group = clique.iter().sorted_by_key(|c| c.len()).last().unwrap();

        let mut result = Vec::new();
        for &c in group {
            result.push(computer_key.get(&c).unwrap().to_string());
        }
        result.sort();

        Ok(result.join(","))
    }

    fn setup(&mut self) {
        println!("Building graph...");
        let input = self.read_file_into_vec();

        for line in input.iter() {
            let (computer_1, computer_2) = line.split_once('-').unwrap();

            let computer_1_1 = computer_1.chars().nth(0).unwrap();
            let computer_1_2 = computer_1.chars().nth(1).unwrap();
            let node_1 = self.graph.add_node([computer_1_1, computer_1_2]);

            let computer_2_1 = computer_2.chars().nth(0).unwrap();
            let computer_2_2 = computer_2.chars().nth(1).unwrap();
            let node_2 = self.graph.add_node([computer_2_1, computer_2_2]);

            self.graph.add_simple_edge(node_1, node_2);
            self.graph.add_simple_edge(node_2, node_1);
        }
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut day = Day23::new();
    init_logger();
    common::file::download_input_file(2024, day.day_number).await?;

    let result = day.run_day()?;
    info!("Day {} - part 1: {:?}", result.day, result.part_1);
    info!("Day {} - part 2: {:?}", result.day, result.part_2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use common::file::get_data_dir;
    use common::test_utils::init_logger;

    #[test]
    fn part_1_test() -> Result<(), Box<dyn Error>> {
        init_logger();
        let expected = String::from("7");

        let dir = get_data_dir(2024, 23);

        let mut day = Day23::new();
        day.file_path = dir.join("example_1.txt");
        day.setup();

        let result = day.part_1()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<(), Box<dyn Error>> {
        init_logger();
        let expected = String::from("co,de,ka,ta");

        let dir = get_data_dir(2024, 23);

        let mut day = Day23::new();
        day.file_path = dir.join("example_1.txt");
        day.setup();

        let result = day.part_2()?;

        assert_eq!(expected, result);

        Ok(())
    }
}
