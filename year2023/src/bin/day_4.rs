use common::base_day::BaseDay;
use common::file::get_input_path;
use common::time_it;
use common::utils::init_logger;
use log::info;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Card<'input> {
    id: usize,
    winning_numbers: HashSet<&'input str>,
    numbers_to_check: Vec<&'input str>,
}

impl<'input> Card<'input> {
    fn win_new_cards(&self) -> Vec<usize> {
        let winning_numbers = self
            .numbers_to_check
            .iter()
            .filter(|number| self.winning_numbers.contains(*number))
            .count();
        ((self.id + 1)..=(self.id + winning_numbers)).collect()
    }
}

pub struct Day4<'input> {
    day_number: u32,
    input_data: String,
    file_path: PathBuf,
    data: Vec<Card<'input>>,
    cards: HashMap<usize, Card<'input>>,
}

impl<'input> Default for Day4<'input> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'input> Day4<'input> {
    pub fn new() -> Day4<'input> {
        Day4 {
            day_number: 4,
            input_data: String::new(),
            file_path: get_input_path(2023, 4),
            data: Vec::new(),
            cards: HashMap::new(),
        }
    }

    fn parse_input_line(line: &str) -> Card<'_> {
        let (card_info, rest) = line.split_once(": ").expect("Could not find a colon");
        let (winning, to_check) = rest.split_once(" | ").unwrap();

        let id = card_info
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Card {
            id,
            winning_numbers: winning.split_whitespace().collect(),
            numbers_to_check: to_check.split_whitespace().collect(),
        }
    }

    fn process_cards(&self, cards: &[Card]) -> HashMap<usize, Vec<usize>> {
        cards
            .iter()
            .map(|card| (card.id, card.win_new_cards()))
            .collect::<HashMap<_, _>>()
    }

    fn process_card_ids_with_cache(
        &self,
        cards: &[usize],
        cache: &HashMap<usize, Vec<usize>>,
    ) -> Vec<usize> {
        cards
            .iter()
            .flat_map(|card| cache.get(card).unwrap().clone())
            .collect()
    }
}

impl<'input> BaseDay for Day4<'input> {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .data
            .iter()
            .map(|card| {
                card.numbers_to_check
                    .iter()
                    .filter(|number| card.winning_numbers.contains(*number))
                    .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
            })
            .sum::<u32>()
            .to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut cards_processed = self.data.len();

        let cache: HashMap<usize, Vec<usize>> = self.process_cards(&self.data);

        let mut cards = cache.values().flatten().copied().collect::<Vec<_>>();
        while let Some(batch) = (!cards.is_empty()).then_some(cards) {
            cards_processed += batch.len();
            cards = self.process_card_ids_with_cache(&batch, &cache);
        }

        Ok(cards_processed.to_string())
    }

    fn setup(&mut self) {
        self.input_data = fs::read_to_string(&self.file_path).expect("Error reading input!");

        let input_ref: &'input str = unsafe { std::mem::transmute(&*self.input_data) };

        self.data = input_ref
            .lines()
            .map(|line| Day4::parse_input_line(line))
            .collect::<Vec<_>>();

        self.cards = self
            .data
            .iter()
            .map(|card| (card.id, card.clone()))
            .collect();
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut day = Day4::new();
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
mod test_day_4 {
    use super::*;
    use common::file::get_data_dir;

    #[test]
    fn parse_input_line_test() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = Card {
            id: 1,
            winning_numbers: HashSet::from(["41", "48", "83", "86", "17"]),
            numbers_to_check: vec!["83", "86", "6", "31", "17", "9", "48", "53"],
        };

        let result = Day4::parse_input_line(line);
        assert_eq!(expected, result);
    }

    #[test]
    fn part_1_test() -> Result<(), Box<dyn Error>> {
        let mut day = Day4::new();
        day.file_path = get_data_dir(2023, 4).join("example_1.txt");
        day.setup();

        let expected = String::from("13");
        let result = day.part_1()?;
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn win_card_test() -> Result<(), Box<dyn Error>> {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = vec![2, 3, 4, 5];

        let card = Day4::parse_input_line(line);
        assert_eq!(expected, card.win_new_cards());

        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<(), Box<dyn Error>> {
        let mut day = Day4::new();
        day.file_path = get_data_dir(2023, 4).join("example_1.txt");
        day.setup();

        let expected = String::from("30");
        let result = day.part_2()?;
        assert_eq!(expected, result);

        Ok(())
    }
}
