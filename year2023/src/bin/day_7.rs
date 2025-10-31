use common::base_day::BaseDay;
use common::file::get_input_path;
use common::time_it;
use common::utils::init_logger;
use itertools::Itertools;
use log::info;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

const CARD_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARD_ORDER_WITH_JOKER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];
const JOKER_SYMBOL: char = 'J';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Card {
    symbol: char,
    use_joker: bool,
}

impl Card {
    fn new(symbol: char, use_joker: bool) -> Self {
        let card_order = if use_joker {
            CARD_ORDER_WITH_JOKER
        } else {
            CARD_ORDER
        };

        if card_order.contains(&symbol) {
            Self { symbol, use_joker }
        } else {
            panic!("Invalid symbol: {}", symbol);
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let card_order = if self.use_joker {
            CARD_ORDER_WITH_JOKER
        } else {
            CARD_ORDER
        };

        let self_index = card_order.iter().position(|&x| x == self.symbol).unwrap();
        let other_index = card_order.iter().position(|&x| x == other.symbol).unwrap();

        self_index.cmp(&other_index)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Hand {
    cards: [Card; 5],
    use_joker: bool,
}

impl Hand {
    fn from_str(cards: &str, use_joker: bool) -> Option<Self> {
        if cards.len() == 5 {
            Some(Self {
                cards: cards
                    .chars()
                    .map(|c| Card::new(c, use_joker))
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("invalid hand"),
                use_joker,
            })
        } else {
            None
        }
    }

    fn hand_type(&self) -> HandType {
        let count: HashMap<char, u32> =
            HashMap::from_iter(self.cards.iter().fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c.symbol).or_insert(0) += 1;
                acc
            }));

        let highest_count = count.values().max().unwrap();
        let hand_type_without_joker = if *highest_count == 5 {
            HandType::FiveOfAKind
        } else if *highest_count == 4 {
            HandType::FourOfAKind
        } else if *highest_count == 3 {
            if count.keys().len() == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if *highest_count == 2 {
            if count.keys().len() == 3 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        };

        if self.use_joker && self.cards.iter().any(|c| c.symbol == JOKER_SYMBOL) {
            match hand_type_without_joker {
                HandType::FiveOfAKind => HandType::FiveOfAKind,
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::FullHouse => HandType::FiveOfAKind,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::TwoPair => {
                    if count[&JOKER_SYMBOL] == 2 {
                        HandType::FourOfAKind
                    } else {
                        HandType::FullHouse
                    }
                }
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::HighCard => HandType::OnePair,
            }
        } else {
            hand_type_without_joker
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_iter(self.cards.iter().map(|c| c.symbol))
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = self.hand_type().cmp(&other.hand_type());
        if result == Ordering::Equal {
            for index in 0..=3 {
                let cmp = self.cards[index].cmp(&other.cards[index]);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            self.cards[4].cmp(&other.cards[4])
        } else {
            result
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day7 {
    day_number: u32,
    file_path: PathBuf,
}

impl Default for Day7 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            day_number: 7,
            file_path: get_input_path(2023, 7),
        }
    }

    fn parse_line(line: &str, use_joker: bool) -> (Hand, u128) {
        let split = line.split_once(' ').unwrap();
        let hand = Hand::from_str(split.0, use_joker).unwrap();
        let bid = split.1.parse::<u128>().unwrap();
        (hand, bid)
    }

    fn winnings(&self, use_joker: bool) -> u128 {
        self.read_file_into_vec()
            .iter()
            .map(|line| Self::parse_line(line, use_joker))
            .sorted_by(|a, b| a.0.cmp(&b.0))
            .enumerate()
            .fold(0, |acc, (index, (_, bid))| acc + (index as u128 + 1) * bid)
    }
}

impl BaseDay for Day7 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.winnings(false).to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.winnings(true).to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = Day7::new();
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
mod test_day_7 {
    use super::*;
    use common::file::get_data_dir;
    use rstest::rstest;

    #[rstest]
    #[case('3', '2', Ordering::Greater)]
    #[case('A', 'K', Ordering::Greater)]
    #[case('A', 'Q', Ordering::Greater)]
    #[case('2', 'A', Ordering::Less)]
    #[case('T', 'T', Ordering::Equal)]
    fn test_card_ordering(
        #[case] symbol_1: char,
        #[case] symbol_2: char,
        #[case] expected: Ordering,
    ) {
        let card_1 = Card {
            symbol: symbol_1,
            use_joker: false,
        };
        let card_2 = Card {
            symbol: symbol_2,
            use_joker: false,
        };
        assert_eq!(expected, card_1.cmp(&card_2));
    }

    #[rstest]
    #[case("AAAAA", HandType::FiveOfAKind)]
    #[case("AAAA2", HandType::FourOfAKind)]
    #[case("AAA55", HandType::FullHouse)]
    #[case("AAJ2J", HandType::TwoPair)]
    #[case("AA32J", HandType::OnePair)]
    #[case("AK42J", HandType::HighCard)]
    #[case("QQQJA", HandType::ThreeOfAKind)]
    fn test_hand_type(#[case] hand: &str, #[case] expected: HandType) {
        let hand = Hand::from_str(hand, false).unwrap();

        assert_eq!(hand.hand_type(), expected);
    }

    #[rstest]
    #[case("AAAAA", HandType::FiveOfAKind)]
    #[case("AAAA2", HandType::FourOfAKind)]
    #[case("AAA55", HandType::FullHouse)]
    #[case("AAJ2J", HandType::FourOfAKind)]
    #[case("AA32J", HandType::ThreeOfAKind)]
    #[case("AK42J", HandType::OnePair)]
    #[case("QQQJA", HandType::FourOfAKind)]
    #[case("55J77", HandType::FullHouse)]
    fn test_hand_type_with_joker(#[case] hand: &str, #[case] expected: HandType) {
        let hand = Hand::from_str(hand, true).unwrap();

        assert_eq!(hand.hand_type(), expected);
    }

    #[rstest]
    #[case("AAAAA", "AAAAA", Ordering::Equal)]
    #[case("AAAAA", "AAAA2", Ordering::Greater)]
    #[case("AAAA2", "AAA55", Ordering::Greater)]
    #[case("AAA55", "AAJ2J", Ordering::Greater)]
    #[case("AAJ2J", "AA32J", Ordering::Greater)]
    #[case("AA32J", "AK42J", Ordering::Greater)]
    #[case("KK677", "KTJJT", Ordering::Greater)]
    #[case("QQQJA", "T55J5", Ordering::Greater)]
    #[case("QQQJA", "KK677", Ordering::Greater)]
    fn test_hand_type_ordering(
        #[case] hand_str_1: &str,
        #[case] hand_str_2: &str,
        #[case] expected: Ordering,
    ) {
        let hand_1 = Hand::from_str(hand_str_1, false).unwrap();
        let hand_2 = Hand::from_str(hand_str_2, false).unwrap();

        assert_eq!(hand_1.cmp(&hand_2), expected);
    }

    #[test]
    fn test_hand_type_ordering_1() {
        let hand_type_1 = HandType::FiveOfAKind;
        let hand_type_2 = HandType::FourOfAKind;

        assert_eq!(hand_type_1.cmp(&hand_type_2), Ordering::Greater);
    }

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let expected = String::from("6440");

        let mut day = Day7::new();
        day.file_path = get_data_dir(2023, 7).join("example_1.txt");

        day.setup();
        let result = day.part_1()?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let expected = String::from("5905");

        let mut day = Day7::new();
        day.file_path = get_data_dir(2023, 7).join("example_1.txt");

        day.setup();
        let result = day.part_2()?;

        assert_eq!(result, expected);

        Ok(())
    }
}
