use common::base_day::BaseDay;
use common::file::get_input_path;
use common::utils::init_logger;
use itertools::Itertools;
use log::info;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::path::PathBuf;
use std::{fmt, fs};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum OperationType {
    AND,
    OR,
    XOR,
}

impl OperationType {
    fn from_string(s: &str) -> OperationType {
        match s {
            "AND" => OperationType::AND,
            "OR" => OperationType::OR,
            "XOR" => OperationType::XOR,
            _ => panic!("Unknown operation type: {}", s),
        }
    }
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationType::AND => write!(f, "AND"),
            OperationType::OR => write!(f, "OR"),
            OperationType::XOR => write!(f, "XOR"),
        }
    }
}

impl Ord for OperationType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == &Self::AND {
            if other == &Self::AND {
                Ordering::Equal
            } else if other == &Self::OR {
                Ordering::Less
            } else {
                Ordering::Less
            }
        } else if self == &Self::OR {
            if other == &Self::AND {
                Ordering::Greater
            } else if other == &Self::OR {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else {
            if other == &Self::AND {
                Ordering::Greater
            } else if other == &Self::OR {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for OperationType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
struct Gate<'a> {
    wire_1: &'a str,
    wire_2: &'a str,
    output: &'a str,
    operation: OperationType,
}

impl<'a> fmt::Display for Gate<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} = {} {} {}",
            self.output, self.wire_1, self.operation, self.wire_2
        )
    }
}

fn solve_recursion<'a>(wire: &'a str, x: &[u8], y: &[u8], gates: &[Gate<'a>]) -> u8 {
    if wire.starts_with('x') {
        let index: usize = wire[1..=2].parse().unwrap();
        x[index]
    } else if wire.starts_with("y") {
        let index: usize = wire[1..=2].parse().unwrap();
        y[index]
    } else {
        let gate = gates.iter().find(|gate| gate.output == wire).unwrap();
        let wire_1 = solve_recursion(gate.wire_1, x, y, gates);
        let wire_2 = solve_recursion(gate.wire_2, x, y, gates);

        match gate.operation {
            OperationType::AND => wire_1 & wire_2,
            OperationType::OR => wire_1 | wire_2,
            OperationType::XOR => wire_1 ^ wire_2,
        }
    }
}

fn make_wire(prefix: char, bit: usize) -> String {
    format!("{}{:02}", prefix, bit)
}

fn solve<'a>(wires: &HashMap<&'a str, u8>, gates: &[Gate<'a>], output_wires: &[&'a str]) -> String {
    let x = [0; 45]
        .iter()
        .enumerate()
        .map(|(index, _)| wires[make_wire('x', index).as_str()])
        .collect_vec();
    let y = [0; 45]
        .iter()
        .enumerate()
        .map(|(index, _)| wires[make_wire('y', index).as_str()])
        .collect_vec();

    output_wires
        .iter()
        .map(|w| solve_recursion(w, &x, &y, gates))
        .join("")
}

fn create_input<'a>(
    input: &'a [&'a str],
) -> Result<(HashMap<&'a str, u8>, Vec<Gate<'a>>, Vec<&'a str>), Box<dyn Error>> {
    let mut gates = Vec::new();
    let mut wires = HashMap::new();

    for line in input {
        if line.contains("->") {
            let re = Regex::new(r"^(.*)\s(\w{2,3})\s(.*)\s->\s(.*)$")?;
            let captures = re.captures(&line).unwrap();

            gates.push(Gate {
                wire_1: captures.get(1).unwrap().as_str(),
                wire_2: captures.get(3).unwrap().as_str(),
                output: captures.get(4).unwrap().as_str(),
                operation: OperationType::from_string(captures.get(2).unwrap().as_str()),
            });
        } else if !line.is_empty() {
            let (wire, value) = line.split_once(": ").unwrap();
            let parsed_value = value.parse::<u8>()?;

            wires.insert(wire, parsed_value);
        }
    }

    let output_wires = gates
        .iter()
        .filter_map(|gate| {
            if gate.output.starts_with("z") {
                Some(gate.output)
            } else {
                None
            }
        })
        .sorted()
        .rev()
        .collect_vec();

    Ok((wires, gates, output_wires))
}

fn integer(input: &str) -> Result<u64, Box<dyn Error>> {
    Ok(u64::from_str_radix(input, 2)?)
}

pub struct Day24 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day24 {
    pub fn new() -> Self {
        Self {
            day_number: 24,
            file_path: get_input_path(2024, 24),
        }
    }
}

impl BaseDay for Day24 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let file_content = fs::read_to_string(&self.file_path)?;
        let data = file_content.lines().collect_vec();

        let (wires, gates, output_wires) = create_input(&data)?;

        let result = solve(&wires, &gates, &output_wires);
        let int = integer(&result)?;

        Ok(int.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        // Part 2 still coming...
        unimplemented!()
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut day = Day24::new();
    init_logger();
    common::file::download_input_file(2024, day.day_number).await?;

    day.setup();

    // let result_1 = day.part_1()?;
    // info!("Day {} - part 1: {:?}", day.day_number, result_1);

    let result_2 = day.part_2()?;
    info!("Day {} - part 2: {:?}", day.day_number, result_2);

    Ok(())
}
