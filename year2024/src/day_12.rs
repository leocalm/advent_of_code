use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::mem::swap;
use std::path::PathBuf;
use common::base_day::BaseDay;
use common::file::get_input_path;
use crate::day_12::Segment::{Horizontal, Vertical};
use common::grid::{Grid, Point};

#[derive(Debug)]
enum Segment {
    Horizontal { y: i32, x1: i32, x2: i32, regions: (char, char) },
    Vertical { x: i32, y1: i32, y2: i32, regions: (char, char) },
}

pub struct Day12 {
    day_number: u32,
    file_path: PathBuf,
}

impl Day12 {
    pub fn new() -> Day12 {
        Day12 {
            day_number: 12,
            file_path: get_input_path(2024, 12)
        }
    }

    fn fill_area(&self, grid: &mut Grid<char>, areas: &mut HashMap<char, Vec<HashSet<Point>>>, current_position: Point) {
        let value = *grid.get(current_position).unwrap();
        grid.update(current_position, '.');
        areas.entry(value).or_insert(Vec::new());

        let mut current_set = HashSet::new();
        current_set.insert(current_position);

        let mut positions_to_visit = vec![current_position];

        while positions_to_visit.len() > 0 {
            let p = positions_to_visit.pop().unwrap();
            let possible_neighbours = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for possible_neighbour in possible_neighbours {
                let coordinate = p.add(possible_neighbour.0, possible_neighbour.1);
                if grid.get(coordinate).is_some_and(|i| *i == value) {
                    if current_set.contains(&coordinate) {
                        continue;
                    }

                    current_set.insert(coordinate);
                    positions_to_visit.push(coordinate);
                    grid.update(coordinate, '.');
                }
            }
        }

        let s = areas.get_mut(&value).take().unwrap();
        s.push(current_set);
    }

    fn calc_perimeter(&self, area: &HashSet<Point>) -> u64 {
        let mut result: u64 = 0;
        let possible_neighbours:Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for a in area {
            let mut current = 4;
            for p in possible_neighbours.iter() {
                let new_coordinates = a.add(p.0, p.1);
                if new_coordinates.x < 0 || new_coordinates.y < 0 {
                    continue;
                }

                if area.contains(&new_coordinates) {
                    current -= 1;
                }
            }
            result += current;
        }

        result
    }

    fn calc_area(&self, area: &HashSet<Point>) -> u64 {
        area.len() as u64
    }

    fn merge_segments(&self, segments: Vec<Segment>) -> Vec<Segment> {
        let mut result = Vec::new();
        let mut horizontals = HashMap::<i32, Vec<(i32, i32, (char, char))>>::new();
        let mut verticals = HashMap::<i32, Vec<(i32, i32, (char, char))>>::new();

        for seg in segments {
            match seg {
                Horizontal { y, mut x1, mut x2,mut  regions} => {
                    if x1 > x2 {
                        swap(&mut x1, &mut x2);
                        regions = (regions.1, regions.0);
                    }
                    horizontals.entry(y).or_default().push((x1, x2, regions));
                }
                Vertical { x, mut y1, mut y2, mut regions} => {
                    if y1 > y2 {
                        swap(&mut y1, &mut y2);
                        regions = (regions.1, regions.0);
                    }
                    verticals.entry(x).or_default().push((y1, y2, regions));
                }
            }
        }

        for (y, mut ranges) in horizontals {
            ranges.sort_by_key(|r| r.0);

            let mut current_range = ranges[0];
            for range in ranges.into_iter().skip(1) {
                if range.0 <= current_range.1 && (range.2.0 == current_range.2.0 || range.2.1 == current_range.2.1) {
                    current_range.1 = current_range.1.max(range.1);
                } else {
                    result.push(Horizontal { y, x1: current_range.0, x2: current_range.1, regions: current_range.2 });
                    current_range = range;
                }
            }
            result.push(Horizontal { y, x1: current_range.0, x2: current_range.1, regions: current_range.2 });
        }

        for (x, mut ranges) in verticals {
            ranges.sort_by_key(|r| r.0);

            let mut current_range = ranges[0];
            for range in ranges.into_iter().skip(1) {
                if range.0 <= current_range.1 && (range.2.0 == current_range.2.0 || range.2.1 == current_range.2.1)  {
                    current_range.1 = current_range.1.max(range.1);
                } else {
                    result.push(Vertical { x, y1: current_range.0, y2: current_range.1, regions: current_range.2 });
                    current_range = range;
                }
            }
            result.push(Vertical { x, y1: current_range.0, y2: current_range.1, regions: current_range.2 });
        }

        result
    }

    fn calc_sides(&self, area: &HashSet<Point>, grid: &Grid<char>) -> u64 {
        let mut segments = vec![];

        for coordinates in area {
            if coordinates.x == 0 {
                segments.push(Horizontal { y: coordinates.x, x1: coordinates.y, x2: coordinates.y + 1, regions: (' ', *grid.get(Point { x: 0, y: coordinates.y }).unwrap_or(&' '))});
            } else {
                if !area.contains(&Point { x: coordinates.x - 1, y: coordinates.y }) {
                    segments.push(Horizontal { y: coordinates.x, x1: coordinates.y, x2: coordinates.y + 1, regions: (*grid.get(Point { x: coordinates.x - 1, y: coordinates.y }).unwrap_or(&' '), *grid.get(*coordinates).unwrap_or(&' '))});
                }
            }

            if !area.contains(&Point { x: coordinates.x + 1, y: coordinates.y }) {
                segments.push(Horizontal { y: coordinates.x + 1, x1: coordinates.y, x2: coordinates.y + 1, regions: (*grid.get(*coordinates).unwrap_or(&' '), *grid.get(Point { x: coordinates.x + 1, y: coordinates.y }).unwrap_or(&' '))});
            }

            if coordinates.y == 0 {
                segments.push(Vertical { x: coordinates.y, y1: coordinates.x, y2: coordinates.x + 1, regions: (' ', *grid.get(*coordinates).unwrap_or(&' '))});
            } else {
                if !area.contains(&Point { x: coordinates.x, y: coordinates.y - 1 }) {
                    segments.push(Vertical { x: coordinates.y, y1: coordinates.x, y2: coordinates.x + 1, regions: (*grid.get(Point { x: coordinates.x, y: coordinates.y - 1 }).unwrap_or(&' '), *grid.get(*coordinates).unwrap_or(&' '))});
                }
            }

            if !area.contains(&Point { x: coordinates.x, y: coordinates.y + 1 }) {
                segments.push(Vertical { x: coordinates.y + 1, y1: coordinates.x, y2: coordinates.x + 1, regions: (*grid.get(*coordinates).unwrap_or(&' '), *grid.get(Point { x: coordinates.x, y: coordinates.y + 1 }).unwrap_or(&' '))});
            }
        }

        self.merge_segments(segments).len() as u64
    }

    fn calc_price(&self, area: &HashSet<Point>) -> u64 {
        self.calc_perimeter(area) * self.calc_area(area)
    }

    fn calc_price_with_discount(&self, area: &HashSet<Point>, grid: &Grid<char>) -> u64 {
        self.calc_sides(area, grid) * self.calc_area(area)
    }

    fn build_hash_map(&self) -> HashMap<char, Vec<HashSet<Point>>> {
        let mut grid = Grid::from_vector(&self.read_file_into_vec_of_vec());
        let mut areas: HashMap<char, Vec<HashSet<Point>>> = HashMap::new();

        let mut current_position = grid.find_different_than('.');
        while current_position.is_some() {
            self.fill_area(&mut grid, &mut areas, current_position.unwrap());
            current_position = grid.find_different_than('.');
        }

        areas
    }
}

impl BaseDay for Day12 {
    fn get_day_number(&self) -> u32 { self.day_number }

    fn part_1(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        let areas = self.build_hash_map();

        for entry in areas {
            for area in entry.1 {
                let price = self.calc_price(&area);

                result += price;
            }
        }

        Ok(result.to_string())
    }

    fn part_2(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result: u64 = 0;
        let areas = self.build_hash_map();

        let grid = Grid::from_vector(&self.read_file_into_vec_of_vec());
        for entry in areas {
            for area in entry.1 {
                result += self.calc_price_with_discount(&area, &grid);
            }
        }

        Ok(result.to_string())
    }

    fn get_input_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }
}
                