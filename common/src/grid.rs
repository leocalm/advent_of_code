use log::debug;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Clone, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
impl Point {
    pub fn add(&self, x: i32, y: i32) -> Point {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn add_point(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn add_tuple(&self, other: (i32, i32)) -> Point {
        Point {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }

    pub fn manhattan_distance(&self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: HashMap<Point, T>,
    rows: i32,
    cols: i32,
    cache: HashMap<T, HashSet<Point>>,
}

impl<T: PartialEq + Debug + Eq + Hash + Clone + Display> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl<T: PartialEq + Debug + Eq + Hash + Clone + Display> Grid<T> {
    pub fn from_vector(input: &[Vec<T>]) -> Grid<T> {
        let mut data = HashMap::new();
        let mut cache = HashMap::new();

        for (row, row_data) in input.iter().enumerate() {
            for (col, value) in row_data.iter().enumerate() {
                let point = Point {
                    x: row as i32,
                    y: col as i32,
                };
                data.insert(point, value.clone());
                cache
                    .entry(value.clone())
                    .or_insert(HashSet::new())
                    .insert(point);
            }
        }

        Grid {
            data,
            rows: input.len() as i32,
            cols: input[0].len() as i32,
            cache,
        }
    }

    pub fn new() -> Grid<T> {
        Grid {
            data: HashMap::new(),
            rows: 0,
            cols: 0,
            cache: HashMap::new(),
        }
    }

    pub fn insert(&mut self, point: Point, value: T) {
        if self.rows == 0 || point.x > self.rows - 1 {
            self.rows = point.x + 1;
        }

        if self.cols == 0 || point.y > self.cols - 1 {
            self.cols = point.y + 1;
        }

        self.data.insert(point, value.clone());
        self.cache
            .entry(value)
            .or_default()
            .insert(point);
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        self.data.get(&point)
    }

    pub fn find(&self, value: T) -> Option<(&Point, T)> {
        if let Some(point) = self.cache.get(&value) {
            point.iter().map(|p| (p, value.clone())).next()
        } else {
            None
        }
    }

    pub fn print(&self) {
        debug!("\n{}", self);
    }

    pub fn update(&mut self, point: Point, symbol: T) {
        self.data.insert(point, symbol);
    }

    pub fn count_values(&self, value: T) -> usize {
        self.data.values().filter(|x| **x == value).count()
    }

    pub fn filter(&self, value: T) -> Vec<Point> {
        self.data
            .iter()
            .filter(|item| *item.1 == value)
            .map(|item| *item.0)
            .collect()
    }

    pub fn filter_contains(&self, value: &[T]) -> HashSet<Point> {
        self.cache
            .iter()
            .filter(|(k, _)| value.contains(k))
            .flat_map(|(_, v)| v.iter().cloned())
            .collect()
    }

    pub fn filter_different_than(&self, value: T) -> HashMap<T, Vec<Point>> {
        let mut map: HashMap<T, Vec<Point>> = HashMap::new();

        self.data
            .iter()
            .filter(|item| *item.1 != value)
            .for_each(|item| {
                let symbol = item.1.clone();
                let coordinates = *item.0;

                map.entry(symbol).or_default().push(coordinates);
            });

        map
    }

    pub fn find_different_than(&self, value: T) -> Option<Point> {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let point = Point { x: row, y: col };
                if self.data.get(&point).is_some_and(|x| *x != value) {
                    return Some(point);
                }
            }
        }

        None
    }

    pub fn rows(&self) -> i32 {
        self.rows
    }
    pub fn cols(&self) -> i32 {
        self.cols
    }
    pub fn cache(&self) -> &HashMap<T, HashSet<Point>> {
        &self.cache
    }
    pub fn data(&self) -> &HashMap<Point, T> {
        &self.data
    }
}

impl<T: PartialEq + Debug + Eq + Hash + Clone + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = "   ".to_string();
        let mut first_row = "".to_string();
        for col in 0..self.cols {
            first_row.push_str(format!("{}", col % 10).as_str());
        }

        result.push_str(&first_row);
        result.push('\n');
        for row in 0..self.rows - 1 {
            result.push_str(format!("{:2} ", row).as_str());
            for col in 0..self.cols {
                result.push_str(
                    format!("{}", self.data.get(&Point { x: row, y: col }).unwrap()).as_str(),
                );
            }
            result.push('\n');
        }

        result.push_str(format!("{} ", self.rows - 1).as_str());
        for col in 0..self.cols {
            result.push_str(
                format!(
                    "{}",
                    self.data
                        .get(&Point {
                            x: self.rows - 1,
                            y: col
                        })
                        .unwrap()
                )
                    .as_str(),
            );
        }

        write!(f, "{}", result)
    }
}
