use common::base_day::BaseDay;
use common::file::get_input_path;
use common::utils::{init_logger, manhattan_distance};
use log::info;
use std::collections::HashMap;
use std::path::PathBuf;

const NUMERIC: [char; 12] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', ' '];
const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

type DistanceKey = (char, char, usize);
type DistanceVal = Vec<(u64, String)>;

fn numeric_part(sequence: &str) -> u64 {
    sequence[..sequence.len() - 1].parse::<u64>().unwrap()
}

fn numeric_keypad() -> HashMap<char, (i32, i32)> {
    HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        (' ', (3, 0)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ])
}

struct NumericKeypad {
    numeric_cache: HashMap<(char, char), Vec<Vec<char>>>,
    numeric_keypad: HashMap<char, (i32, i32)>,
}

impl NumericKeypad {
    fn new() -> NumericKeypad {
        let mut nk = NumericKeypad {
            numeric_cache: HashMap::new(),
            numeric_keypad: numeric_keypad(),
        };
        nk.populate_cache();

        nk
    }

    fn distance_numeric(&self, from: char, to: char) -> Vec<Vec<char>> {
        let &from_coordinates = self.numeric_keypad.get(&from).unwrap();
        let &to_coordinates = self.numeric_keypad.get(&to).unwrap();

        if to == ' ' || from == ' ' {
            return Vec::new();
        }

        let manhattan_distance = manhattan_distance(from_coordinates, to_coordinates);

        if from_coordinates.0 == to_coordinates.0 {
            if from_coordinates.1 == to_coordinates.1 {
                vec![]
            } else if from_coordinates.1 > to_coordinates.1 {
                vec![vec!['<'; manhattan_distance as usize]]
            } else {
                vec![vec!['>'; manhattan_distance as usize]]
            }
        } else if from_coordinates.0 > to_coordinates.0 {
            if from_coordinates.1 == to_coordinates.1 {
                vec![vec!['^'; manhattan_distance as usize]]
            } else if from_coordinates.1 > to_coordinates.1 {
                let mut result = Vec::new();

                let from_n1 =
                    NUMERIC_KEYPAD[(from_coordinates.0 - 1) as usize][from_coordinates.1 as usize];
                let from_n2 =
                    NUMERIC_KEYPAD[from_coordinates.0 as usize][(from_coordinates.1 - 1) as usize];

                for n in self.distance_numeric(from_n1, to) {
                    let mut tmp_result = vec!['^'];
                    tmp_result.extend(n);
                    result.push(tmp_result)
                }

                for n in self.distance_numeric(from_n2, to) {
                    let mut result_n1 = vec!['<'];
                    result_n1.extend(n);
                    result.push(result_n1)
                }

                result
            } else {
                let mut result = Vec::new();

                let from_n1 =
                    NUMERIC_KEYPAD[(from_coordinates.0 - 1) as usize][from_coordinates.1 as usize];
                let from_n2 =
                    NUMERIC_KEYPAD[from_coordinates.0 as usize][(from_coordinates.1 + 1) as usize];
                for n in self.distance_numeric(from_n1, to) {
                    let mut tmp_result = vec!['^'];
                    tmp_result.extend(n);
                    result.push(tmp_result)
                }

                for n in self.distance_numeric(from_n2, to) {
                    let mut result_n1 = vec!['>'];
                    result_n1.extend(n);
                    result.push(result_n1)
                }

                result
            }
        } else {
            if from_coordinates.1 == to_coordinates.1 {
                vec![vec!['v'; manhattan_distance as usize]]
            } else if from_coordinates.1 > to_coordinates.1 {
                let mut result = Vec::new();

                let from_n1 =
                    NUMERIC_KEYPAD[(from_coordinates.0 + 1) as usize][from_coordinates.1 as usize];
                let from_n2 =
                    NUMERIC_KEYPAD[from_coordinates.0 as usize][(from_coordinates.1 - 1) as usize];
                for n in self.distance_numeric(from_n1, to) {
                    let mut tmp_result = vec!['v'];
                    tmp_result.extend(n);
                    result.push(tmp_result)
                }

                for n in self.distance_numeric(from_n2, to) {
                    let mut result_n1 = vec!['<'];
                    result_n1.extend(n);
                    result.push(result_n1)
                }

                result
            } else {
                let mut result = Vec::new();

                let from_n1 =
                    NUMERIC_KEYPAD[(from_coordinates.0 + 1) as usize][from_coordinates.1 as usize];
                let from_n2 =
                    NUMERIC_KEYPAD[from_coordinates.0 as usize][(from_coordinates.1 + 1) as usize];
                for n in self.distance_numeric(from_n1, to) {
                    let mut tmp_result = vec!['v'];
                    tmp_result.extend(n);
                    result.push(tmp_result)
                }

                for n in self.distance_numeric(from_n2, to) {
                    let mut result_n1 = vec!['>'];
                    result_n1.extend(n);
                    result.push(result_n1)
                }

                result
            }
        }
    }

    fn populate_cache(&mut self) {
        for &i in NUMERIC.iter() {
            for &j in NUMERIC.iter() {
                let distances = self.distance_numeric(i, j);
                self.numeric_cache.insert((i, j), distances);
            }
        }
    }

    pub fn numeric_to_directional(&mut self, numeric: &str) -> Vec<Vec<char>> {
        let mut result: Vec<Vec<char>> = Vec::new();

        let mut chars = vec!['A'];
        chars.extend(numeric.chars().collect::<Vec<char>>());
        for pair in chars.windows(2) {
            let (from, to) = (pair[0], pair[1]);
            let data = self.numeric_cache.get(&(from, to)).unwrap().to_vec();
            if result.is_empty() {
                for d in data.clone() {
                    let mut tmp = vec!['A'];
                    tmp.extend(d);
                    tmp.push('A');
                    result.push(tmp);
                }
            } else {
                let mut new_result = Vec::new();
                for d in data.iter() {
                    for mut r in result.clone() {
                        r.extend(d.clone());
                        r.push('A');
                        new_result.push(r);
                    }
                }
                result = new_result;
            }
        }

        result
    }
}

pub struct Day21 {
    day_number: u32,
    sequences: HashMap<DistanceKey, DistanceVal>,
    cache: HashMap<(Vec<char>, usize), u64>,
    numeric_keypad: NumericKeypad,
    file_path: PathBuf,
}

impl Day21 {
    pub fn new() -> Day21 {
        Day21 {
            day_number: 21,
            sequences: HashMap::from([
                (('A', 'A', 0), vec![(0, String::new())]),
                (('A', '^', 0), vec![(1, String::from("<"))]),
                (('A', '>', 0), vec![(1, String::from("v"))]),
                (
                    ('A', 'v', 0),
                    vec![(2, String::from("v<")), (2, String::from("<v"))],
                ),
                (
                    ('A', '<', 0),
                    vec![(3, String::from("v<<")), (3, String::from("<v<"))],
                ),
                (('^', 'A', 0), vec![(1, String::from(">"))]),
                (('^', '^', 0), vec![(0, String::new())]),
                (
                    ('^', '>', 0),
                    vec![(2, String::from("v>")), (2, String::from(">v"))],
                ),
                (('^', 'v', 0), vec![(1, String::from("v"))]),
                (('^', '<', 0), vec![(2, String::from("v<"))]),
                (
                    ('<', 'A', 0),
                    vec![(3, String::from(">>^")), (3, String::from(">^>"))],
                ),
                (('<', '^', 0), vec![(2, String::from(">^"))]),
                (('<', '>', 0), vec![(2, String::from(">>"))]),
                (('<', 'v', 0), vec![(1, String::from(">"))]),
                (('<', '<', 0), vec![(0, String::new())]),
                (
                    ('v', 'A', 0),
                    vec![(2, String::from(">^")), (2, String::from("^>"))],
                ),
                (('v', '^', 0), vec![(1, String::from("^"))]),
                (('v', '>', 0), vec![(1, String::from(">"))]),
                (('v', 'v', 0), vec![(0, String::new())]),
                (('v', '<', 0), vec![(1, String::from("<"))]),
                (('>', 'A', 0), vec![(1, String::from("^"))]),
                (
                    ('>', '^', 0),
                    vec![(2, String::from("^<")), (2, String::from("<^"))],
                ),
                (('>', '>', 0), vec![(0, String::new())]),
                (('>', 'v', 0), vec![(1, String::from("<"))]),
                (('>', '<', 0), vec![(2, String::from("<<"))]),
            ]),
            cache: HashMap::new(),
            numeric_keypad: NumericKeypad::new(),
            file_path: get_input_path(2024, 21),
        }
    }

    fn sequences(&self, keys: &[char], index: usize) -> Vec<Vec<char>> {
        let mut result = Vec::new();
        fn build_seq(
            keys: &[char],
            index: usize,
            prev_key: char,
            current_path: &Vec<char>,
            result: &mut Vec<Vec<char>>,
            cache: &HashMap<DistanceKey, DistanceVal>,
        ) {
            if index == keys.len() {
                result.push(current_path.clone());
                return;
            }

            for path in cache.get(&(prev_key, keys[index], 0)).unwrap() {
                let mut new_path = current_path.clone();
                new_path.extend(path.1.chars());
                new_path.push('A');
                build_seq(keys, index + 1, keys[index], &new_path, result, cache);
            }
        }

        build_seq(
            keys,
            index,
            keys[(index as i32 - 1) as usize],
            &Vec::new(),
            &mut result,
            &self.sequences,
        );
        result
    }

    fn shortest_seq(&mut self, keys: &[char], depth: usize) -> u64 {
        let mut total = 0;

        if depth == 0 {
            return keys.len() as u64;
        }

        if let Some(total) = self.cache.get(&(keys.to_vec(), depth)) {
            return *total;
        }

        let mut sub_keys = Vec::new();
        let mut current_sub_key = vec!['A'];
        for &k in keys {
            current_sub_key.push(k);
            if k == 'A' {
                sub_keys.push(current_sub_key);
                current_sub_key = vec!['A'];
            }
        }

        for sk in sub_keys {
            let seqs = self.sequences(&sk, 1);
            let mut min = u64::MAX;
            for seq in seqs {
                let tmp = self.shortest_seq(&seq, depth - 1);
                if tmp < min {
                    min = tmp;
                }
            }
            total += min;
        }
        self.cache.insert((keys.to_vec(), depth), total);

        total
    }

    fn solve(&mut self, keys: &str, depth: usize) -> u64 {
        let sequences = self.numeric_keypad.numeric_to_directional(keys);
        let mut min = u64::MAX;
        for seq in sequences {
            let tmp = self.shortest_seq(&seq, depth);
            if tmp < min {
                min = tmp;
            }
        }
        min -= 1;
        min * numeric_part(keys)
    }
}

impl BaseDay for Day21 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let robots = 2;
        let input = self.read_file_into_vec();
        let mut result = 0;
        for line in input.iter() {
            result += self.solve(line, robots);
        }

        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let robots = 25;
        let input = self.read_file_into_vec();
        let mut result = 0;
        for line in input.iter() {
            result += self.solve(line, robots);
        }

        Ok(result.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = Day21::new();
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
    use std::path::Path;

    #[test]
    fn part_1_test() {
        let expected = 126_384.to_string();

        let project_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let mut day = Day21::new();
        let path = project_root.join("data/2024/day_21/example_1.txt");
        day.file_path = path;

        day.setup();
        let result = day.part_1();

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn part_2_test() {
        let expected = 154_115_708_116_294u64.to_string();

        let project_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let mut day = Day21::new();
        let path = project_root.join("data/2024/day_21/example_1.txt");
        day.file_path = path;

        day.setup();
        let result = day.part_2();

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn build_seq_test() {
        let day = Day21::new();

        let expected = vec![
            vec!['v', '<', '<', 'A', '>', '>', '^', 'A'],
            vec!['v', '<', '<', 'A', '>', '^', '>', 'A'],
            vec!['<', 'v', '<', 'A', '>', '>', '^', 'A'],
            vec!['<', 'v', '<', 'A', '>', '^', '>', 'A'],
        ];
        let result = day.sequences(&vec!['A', '<', 'A'], 1);
        assert_eq!(expected, result);
    }

    #[test]
    fn shortest_seq_test() {
        let mut day = Day21::new();

        let expected = 18;
        let result = day.shortest_seq(&vec!['<', 'A'], 2);
        assert_eq!(expected, result);
    }

    #[test]
    fn solve_test() {
        let mut day = Day21::new();

        let expected = 12_172;
        let result = day.solve(&String::from("179A"), 2);
        assert_eq!(expected, result);
    }
}
