use common::base_day::BaseDay;
use common::file::get_input_path;
use common::utils::init_logger;
use log::info;
use std::array;
use std::error::Error;
use std::path::PathBuf;

pub struct Day25 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day25 {
    pub fn new() -> Day25 {
        Day25 {
            day_number: 25,
            file_path: get_input_path(2024, 25),
        }
    }
}

impl BaseDay for Day25 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        self.read_file_into_vec_of_vec()
            .chunks(8)
            .for_each(|slice| {
                let line = &slice[0];
                let data: [u32; 5] = array::from_fn(|i| {
                    [0u32; 5][i]
                        + slice[1..=5]
                            .iter()
                            .map(|l| if l[i] == '#' { 1u32 } else { 0u32 })
                            .sum::<u32>()
                });

                if line == &['#'; 5] {
                    &mut locks
                } else {
                    &mut keys
                }
                .push(data);
            });

        let count = locks.iter().fold(0, |lock_acc, lock| {
            lock_acc
                + keys.iter().fold(0, |key_acc, key| {
                    key_acc
                        + if array::from_fn::<u32, 5, _>(|i| lock[i] + key[i])
                            .iter()
                            .any(|&s| s > 5)
                        {
                            0
                        } else {
                            1
                        }
                })
        });

        Ok(count.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(String::new())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut day = Day25::new();
    init_logger();
    common::file::download_input_file(2024, day.day_number).await?;

    let result_1 = day.part_1()?;
    info!("Day {} - part 1: {:?}", day.day_number, result_1);

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
        let expected = String::from("3");

        let dir = get_data_dir(2024, 25);

        let mut day = Day25::new();
        day.file_path = dir.join("example.txt");
        day.setup();

        let result = day.part_1()?;

        assert_eq!(expected, result);

        Ok(())
    }
}
