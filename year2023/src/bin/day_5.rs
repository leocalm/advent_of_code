use common::base_day::BaseDay;
use common::file::get_input_path;
use common::utils::init_logger;
use log::info;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use common::time_it;

const MAP_ORDER: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct MapEntry {
    source: u128,
    destination: u128,
    size: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    name: String,
    entries: Vec<MapEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PuzzleInput {
    seeds: Vec<u128>,
    maps: HashMap<String, Map>,
}

impl PuzzleInput {
    fn new() -> Self {
        Self {
            seeds: Vec::new(),
            maps: HashMap::new(),
        }
    }
}

impl Default for PuzzleInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    fn new(name: String) -> Self {
        Self {
            name,
            entries: Vec::<MapEntry>::new(),
        }
    }
    fn destination(&self, source: u128) -> u128 {
        self.entries
            .iter()
            .filter_map(|entry| if (entry.source..entry.source + entry.size)
                .contains(&source) { Some(entry.destination + (source - entry.source)) } else { None })
            .min()
            .unwrap_or(source)
    }

    fn destinations(&self, start_source: u128, size: u128) -> Vec<(u128, u128)> {
        if let Some(entry) = self.entries.iter().find(|entry| start_source >= entry.source && start_source < entry.source + entry.size) {
            let final_source = start_source + size;

            if final_source < entry.source + entry.size {
                let start_destination = entry.destination + (start_source - entry.source);
                vec![(start_destination, size)]
            } else {
                let start_destination = entry.destination + (start_source - entry.source);
                let current_size = entry.size - (start_source - entry.source);
                let mut destinations = vec![(start_destination, current_size)];
                destinations.extend(self.destinations(start_source + current_size, size - current_size));
                destinations
            }
        } else {
            if let Some(entry) = self.entries.iter().find(|entry| (start_source + size) >= entry.source && start_source < entry.source) {
                let mut destinations = vec![(start_source, entry.source - start_source)];
                destinations.extend(self.destinations(entry.source, size - (entry.source - start_source)));
                destinations
            } else {
                vec![(start_source, size)]
            }
        }
    }
}

impl MapEntry {
    fn new(s_source: &str, s_destination: &str, s_size: &str) -> Self {
        let source = s_source.parse().unwrap();
        let destination = s_destination.parse().unwrap();
        let size = s_size.parse().unwrap();

        Self {
            source,
            destination,
            size,
        }
    }
}

pub struct Day5 {
    day_number: u32,
    file_path: PathBuf,
    puzzle_input: PuzzleInput
}

impl Default for Day5 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {
            day_number: 5,
            file_path: get_input_path(2023, 5),
            puzzle_input: PuzzleInput::new(),
        }
    }

    fn parse_map(lines: &[&str], start_index: usize) -> Map {
        let map_name = &lines[start_index].split_whitespace().nth(0).unwrap();
        let mut map = Map::new(map_name.to_string());

        let mut index = start_index + 1;
        while index < lines.len() && !lines[index].is_empty() {
            let split = lines[index].split_whitespace().collect::<Vec<_>>();
            let [dest, src, size] = split[..3] else {
                panic!("malformed map line");
            };
            map.entries
                .push(MapEntry::new(src, dest, size));
            index += 1;
        }

        map
    }

    fn parse_input(lines: &[&str]) -> PuzzleInput {
        let mut puzzle_input = PuzzleInput::default();

        let mut index = 0;
        while index < lines.len() {
            let line = lines[index];

            if !line.trim().is_empty() {
                if line.starts_with("seeds:") {
                    puzzle_input.seeds = line
                        .split_once(": ")
                        .unwrap()
                        .1
                        .split_whitespace()
                        .map(|s| s.parse().expect("Error parsing seed"))
                        .collect::<Vec<_>>();
                    index += 1;
                } else {
                    let map = Day5::parse_map(&lines, index);
                    index += map.entries.len() + 1;
                    puzzle_input.maps.insert(map.name.clone(), map);
                }
            } else {
                index += 1;
            }
        }

        puzzle_input
    }

    fn map_through_destination(input: u128, maps: &HashMap<String, Map>) -> u128 {
        MAP_ORDER.iter().fold(input, |value, key| maps[*key].destination(value))
    }

    fn min_location(puzzle_input: &PuzzleInput) -> u128 {
        puzzle_input.seeds.iter().map(|seed| Day5::map_through_destination(*seed, &puzzle_input.maps)).min().unwrap()
    }

    fn min_location_for_range(puzzle_input: &PuzzleInput) -> u128 {
        puzzle_input.seeds.chunks(2)
            .map(|chunk| {(chunk[0], chunk[1])})
            .flat_map(|seed| puzzle_input.maps["seed-to-soil"].destinations(seed.0, seed.1))
            .flat_map(|soil| puzzle_input.maps["soil-to-fertilizer"].destinations(soil.0, soil.1))
            .flat_map(|fertilizer| puzzle_input.maps["fertilizer-to-water"].destinations(fertilizer.0, fertilizer.1))
            .flat_map(|water| puzzle_input.maps["water-to-light"].destinations(water.0, water.1))
            .flat_map(|light| puzzle_input.maps["light-to-temperature"].destinations(light.0, light.1))
            .flat_map(|temperature| puzzle_input.maps["temperature-to-humidity"].destinations(temperature.0, temperature.1))
            .flat_map(|humidity| puzzle_input.maps["humidity-to-location"].destinations(humidity.0, humidity.1))
            .min_by_key(|value| value.0)
            .unwrap()
            .0
    }
}

impl BaseDay for Day5 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(Day5::min_location(&self.puzzle_input).to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(Day5::min_location_for_range(&self.puzzle_input).to_string())
    }

    fn setup(&mut self) {
        let input = std::fs::read_to_string(&self.file_path).unwrap();
        let lines = input.lines().collect::<Vec<_>>();
        self.puzzle_input = Day5::parse_input(&lines);
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = Day5::new();
    init_logger();
    common::file::download_input_file(2023, day.day_number).await?;

    day.setup();
    let result_part_1 = time_it!("part_1", { day.part_1()? });
    info!("Part 1: {:?}", result_part_1,);

    let result_part_2 = time_it!("part_2", { day.part_2()? });
    info!("Part 2: {:?}", result_part_2,);

    Ok(())
}

#[cfg(test)]
mod test_day_5 {
    use rstest::*;
    use super::*;
    use common::file::get_data_dir;

    #[fixture]
    pub fn map() -> Map {
        Map {
            name: String::from("test-map"),
            entries: vec![
                MapEntry {
                    source: 10,
                    destination: 20,
                    size: 30,
                },
                MapEntry {
                    source: 40,
                    destination: 0,
                    size: 10,
                },
                MapEntry {
                    source: 60,
                    destination: 100,
                    size: 20,
                },
            ],
        }
    }

    #[rstest]
    fn destination_test(map: Map) {
        let source = 12;
        let expected_destination = 22;

        let result = map.destination(source);
        assert_eq!(result, expected_destination);
    }

    #[rstest]
    #[case(12, 10, vec![(22, 10)])]
    #[case(99, 10, vec![(99, 10)])]
    #[case(35, 10, vec![(45, 5), (0, 5)])]
    #[case(35, 50, vec![(45, 5), (0, 10), (50, 10), (100, 20), (80, 5)])]
    #[case(15, 50, vec![(25, 25), (0, 10), (50, 10), (100, 5)])]
    fn destinations_test(#[case] start_source: u128, #[case] size: u128, #[case] expected_destinations: Vec<(u128, u128)>, map: Map) {
        let result = map.destinations(start_source, size);
        assert_eq!(result, expected_destinations);
    }

    #[rstest]
    fn destination_not_found_test(map: Map) {
        let source = 99;
        let expected_destination = 99;

        let result = map.destination(source);
        assert_eq!(result, expected_destination);
    }

    #[rstest]
    #[case(vec![10, 5], 40)]
    fn flat_map_test(#[case] seeds: Vec<u128>, #[case] min_destination: u128, map: Map) {
        let result = seeds.chunks(2)
            .flat_map(|seed| map.destinations(seed[0], seed[1]))
            .flat_map(|seed| map.destinations(seed.0, seed.1))
            .flat_map(|seed| map.destinations(seed.0, seed.1))
            .min_by_key(|value| value.0)
            .unwrap().0;
        assert_eq!(result, min_destination);
    }

    #[test]
    fn part_1_test() -> Result<(), Box<dyn Error>> {
        let mut day = Day5::new();
        day.file_path = get_data_dir(2023, 5).join("example_1.txt");
        day.setup();

        let result = day.part_1()?;
        let expected = String::from("35");
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<(), Box<dyn Error>> {
        let mut day = Day5::new();
        day.file_path = get_data_dir(2023, 5).join("example_1.txt");
        day.setup();

        let result = day.part_2()?;
        let expected = String::from("46");
        assert_eq!(result, expected);

        Ok(())
    }
}
