use common::base_day::BaseDay;
use common::file::get_input_path;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Ord, Eq, PartialOrd, PartialEq)]
struct Block {
    id: Option<u32>,
    size: u32,
}

pub struct Day9 {
    day_number: u32,
    file_path: PathBuf,
    disc: Vec<Block>,
}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {
            day_number: 9,
            file_path: get_input_path(2024, 9),
            disc: Vec::new(),
        }
    }

    fn read_input_into_disk(&mut self, input: &Vec<u32>) {
        self.disc = Vec::new();
        let mut current_id = 0;

        for (index, value) in input.iter().enumerate() {
            if index % 2 == 0 {
                self.disc.push(Block {
                    id: Some(current_id),
                    size: *value,
                });
                current_id += 1;
            } else {
                if *value > 0 {
                    self.disc.push(Block {
                        id: None,
                        size: *value,
                    });
                }
            }
        }
    }

    fn find_first_free_block(&mut self) -> (usize, Block) {
        self.disc
            .clone()
            .into_iter()
            .enumerate()
            .find(|(_, block)| block.id.is_none())
            .unwrap()
    }

    fn move_block(&mut self) -> bool {
        let mut last_index = self.disc.len() - 1;
        let mut last_block = self.disc.get_mut(last_index).unwrap().clone();

        while last_block.id.is_none() {
            last_index -= 1;
            last_block = self.disc.get_mut(last_index).unwrap().clone();
        }

        let mut first_free_block = self.find_first_free_block();
        // println!("last_block: {:?}, first_free_block: {:?}, disc: {:?}", self.disc.len() - 1, first_free_block, self.disc);
        if first_free_block.0 == self.disc.len() - 1 {
            return false;
        }

        self.disc.remove(last_index);
        last_block.size -= 1;
        if last_block.size > 0 {
            self.disc.insert(last_index, last_block);
        }

        self.disc.remove(first_free_block.0);
        first_free_block.1.size -= 1;
        if first_free_block.1.size > 0 {
            self.disc.insert(first_free_block.0, first_free_block.1);
        }

        if self.disc.last().unwrap().id.is_none() {
            let block = self.disc.last_mut().unwrap();
            block.size += 1;
        } else {
            self.disc.push(Block { id: None, size: 1 });
        }

        if first_free_block.0 > 0
            && self.disc.get(first_free_block.0 - 1).unwrap().id == last_block.id
        {
            let block = self.disc.get_mut(first_free_block.0 - 1).unwrap();
            block.size += 1;
        } else {
            self.disc.insert(
                first_free_block.0,
                Block {
                    id: last_block.id,
                    size: 1,
                },
            );
        }

        true
    }

    fn get_first_available_free_space_with_size(&self, size: u32) -> Option<(usize, Block)> {
        for (index, block) in self.disc.iter().enumerate() {
            if block.id.is_none() && block.size >= size {
                return Some((index, block.clone()));
            }
        }

        None
    }

    fn move_whole_file(&mut self, index: usize, file: Block) -> bool {
        if file.id.is_none() {
            return false;
        }

        let last_file = (index, file);
        let free_space = self.get_first_available_free_space_with_size(last_file.1.size);

        if free_space.is_some() && free_space.unwrap().0 < index {
            self.disc.remove(last_file.0);
            self.disc.insert(
                last_file.0,
                Block {
                    id: None,
                    size: last_file.1.size,
                },
            );

            if free_space.unwrap().1.size == last_file.1.size {
                self.disc.remove(free_space.unwrap().0);
            } else {
                let block = self.disc.get_mut(free_space.unwrap().0).unwrap();
                block.size = free_space.unwrap().1.size - last_file.1.size;
            }
            self.disc.insert(free_space.unwrap().0, last_file.1);

            return true;
        }

        false
    }

    fn calculate_checksum(&self) -> u64 {
        let mut result = 0;
        let mut current_index = 0;
        // let disc_as_str = self.disc_to_string();

        for value in self.disc.iter() {
            if value.id.is_some() {
                for _ in 0..value.size {
                    result += current_index as u64 * value.id.unwrap() as u64;
                    current_index += 1;
                }
            } else {
                for _ in 0..value.size {
                    current_index += 1;
                }
            }
        }

        result
    }
}

impl BaseDay for Day9 {
    fn get_day_number(&self) -> u32 {
        self.day_number
    }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let data = self.read_file_into_vec_of_vec();
        let input = data
            .get(0)
            .unwrap()
            .iter()
            .map(|x| x.to_digit(10).unwrap() as u32)
            .collect::<Vec<u32>>();

        self.read_input_into_disk(&input);
        while self.move_block() {}

        Ok(self.calculate_checksum().to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let data = self.read_file_into_vec_of_vec();
        let input = data
            .get(0)
            .unwrap()
            .iter()
            .map(|x| x.to_digit(10).unwrap() as u32)
            .collect::<Vec<u32>>();

        self.read_input_into_disk(&input);
        for block in self.disc.clone().iter().rev() {
            let mut index = 0;
            for b in self.disc.iter() {
                if b.id.is_some() && block.id.is_some() {
                    if b.id.unwrap() == block.id.unwrap() {
                        break;
                    }
                }
                index += 1;
            }
            self.move_whole_file(index, block.clone());
        }
        Ok(self.calculate_checksum().to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
