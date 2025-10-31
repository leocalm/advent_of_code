use common::base_day::BaseDay;
use common::file::get_input_path;
use common::time_it;
use common::utils::init_logger;
use log::info;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug)]
struct Race {
    time: u128,
    record: u128,
}

impl Race {
    fn new(time: u128, record: u128) -> Self {
        Self { time, record }
    }

    #[allow(dead_code)]
    fn distance(&self, hold_the_button_for: u128) -> u128 {
        if hold_the_button_for == 0 || hold_the_button_for == self.time {
            0
        } else {
            let remaining = self.time - hold_the_button_for;
            let speed = hold_the_button_for;

            remaining * speed
        }
    }

    /// Mathematical proof for the binary search & symmetry approach
    /// ===========================================================
    ///
    /// We must count integer h such that:
    ///
    ///     distance(h) = h * (T - h) > R,
    ///     where T = total race time and 0 < h < T.
    ///
    /// Define f(h) = h * (T - h).
    ///
    /// 1) Shape of f(h)
    /// ----------------
    /// f(h) is a quadratic function:
    ///
    ///     f(h) = -h² + Th
    ///
    /// This is a concave parabola opening downward.
    /// Maximum is at h = T/2. Therefore:
    /// - f(h) strictly increases for h in [0, T/2]
    /// - f(h) strictly decreases for h in [T/2, T]
    ///
    /// Thus the valid region {h | f(h) > R} is a **single continuous interval**.
    ///
    /// 2) Symmetry
    /// -----------
    /// f(h) = f(T - h). Therefore:
    ///
    ///     If h_low is the smallest h such that f(h) > R,
    ///     then the largest h is h_high = T - h_low.
    ///
    /// So the number of valid integer h values is:
    ///
    ///     count = h_high - h_low + 1
    ///           = (T - h_low) - h_low + 1
    ///           = T - 2*h_low + 1
    ///
    /// 3) Why we only binary-search on [1, floor((T-1)/2)]
    /// ---------------------------------------------------
    /// Since f is strictly increasing on [0, T/2]:
    ///
    /// - The first solution must lie in [1, T/2].
    /// - Once the smallest h (h_low) is found, the largest is T - h_low.
    ///
    /// So we only need to binary-search the left half of the parabola.
    ///
    /// 4) Integer safety
    /// -----------------
    /// This avoids floating-point rounding issues from the quadratic formula,
    /// which can shift ceil/floor boundaries and give incorrect counts.
    ///
    /// -----------------
    /// This was done with help from chatgpt. I knew it could be done better than brute force, but
    /// needed help figuring out how :D
    fn distance_analytical(&self) -> u128 {
        let time = self.time;
        let record = self.record;

        // No possible positive h if T < 2 (there's no hold length in 1..T-1)
        if time <= 1 {
            return 0;
        }

        // Search only in [1..=T/2] (inclusive). If T is even, h == T/2 gives 0 product (T/2)*(T - T/2) = (T/2)^2,
        // still valid to check; searching up to T/2 is safe.
        let mut low: u128 = 1;
        // limit = floor((T - 1) / 2) is safe but using T/2 also works; use (T - 1)/2 to avoid checking middle twice
        let mut high: u128 = (time - 1) / 2;

        // If hi is 0 there is no h in the range
        if high < low {
            return 0;
        }

        // Binary search for smallest h in [lo..=hi] such that h * (T - h) > R
        let mut found: Option<u128> = None;
        while low <= high {
            let mid = (low + high) / 2;
            let prod = mid.saturating_mul(time - mid); // using u128 so saturating_mul not strictly needed, but safe
            if prod > record {
                found = Some(mid);
                if mid == 0 {
                    break;
                } // guard against infinite loop though mid>=1 here
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        let h_low = match found {
            None => return 0,
            Some(v) => v,
        };

        // number of integer h values in (r1, r2) equals T - 2*h_low + 1
        time - 2 * h_low + 1
    }
}

pub struct Day6 {
    day_number: u32,
    file_path: PathBuf,
}

impl Default for Day6 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {
            day_number: 6,
            file_path: get_input_path(2023, 6),
        }
    }

    fn parse_line(line: &str) -> Vec<u128> {
        line.split_once(':')
            .expect("Error parsing time input! Didn't find a colon")
            .1
            .split_whitespace()
            .map(|t| {
                t.parse()
                    .expect("Error parsing time input! Error parsing time to u128")
            })
            .collect::<Vec<u128>>()
    }

    fn parse_input(lines: &[String]) -> (Vec<u128>, Vec<u128>) {
        (Self::parse_line(&lines[0]), Self::parse_line(&lines[1]))
    }

    fn parse_input_line_part_2(line: &str) -> u128 {
        line.split_once(':')
            .expect("Error parsing time input! Didn't find a colon")
            .1
            .split_whitespace()
            .collect::<String>()
            .parse::<u128>()
            .expect("Error parsing time input! Error parsing value to u128")
    }

    fn parse_input_part_2(lines: &[String]) -> Race {
        Race::new(
            Self::parse_input_line_part_2(&lines[0]),
            Self::parse_input_line_part_2(&lines[1]),
        )
    }
    #[allow(dead_code)]
    fn count_ways_to_win_race(race: Race) -> u128 {
        (1..=race.time)
            .map(|hold_the_button_for| race.distance(hold_the_button_for))
            .filter(|distance| *distance > race.record)
            .count() as u128
    }
}

impl BaseDay for Day6 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let (times, records) = Self::parse_input(&self.read_file_into_vec());

        Ok((0..times.len())
            .map(|race_index| {
                Race::new(times[race_index], records[race_index]).distance_analytical()
            })
            .product::<u128>()
            .to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let race = Self::parse_input_part_2(&self.read_file_into_vec());

        Ok(race.distance_analytical().to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut day = Day6::new();
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
mod day_6_tests {
    use super::*;
    use common::file::get_data_dir;

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let expected = String::from("288");

        let mut day = Day6::new();
        day.file_path = get_data_dir(2023, 6).join("example_1.txt");

        day.setup();
        let result = day.part_1()?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let expected = String::from("71503");

        let mut day = Day6::new();
        day.file_path = get_data_dir(2023, 6).join("example_1.txt");

        day.setup();
        let result = day.part_2()?;

        assert_eq!(result, expected);

        Ok(())
    }
}
