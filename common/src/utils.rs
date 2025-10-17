use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::graph::{Graph, Node};
use crate::grid::{Grid, Point};
use std::collections::VecDeque;
use std::hash::Hash;
use env_logger::Env;

#[allow(dead_code)]
pub const DIFFS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn get_neighbours(grid: &Grid<char>, current_position: Point) -> Vec<Point> {
    let mut result = vec![];
    for diff in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let neighbour_position = current_position.add(diff.0, diff.1);
        if grid.get(neighbour_position).is_some_and(|n| *n == '.') {
            result.push(neighbour_position);
        }
    }

    result
}

pub fn add_corners(grid: &Grid<char>, graph: &mut Graph<Point>, value_to_add: char) {
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let current_position = Point { x: row, y: col };
            let current_value = *grid.get(current_position).unwrap();
            if current_value == value_to_add {
                let neighbours = get_neighbours(grid, current_position);
                let neighbours_count = neighbours.len();
                if neighbours_count == 2 {
                    if neighbours[0].x != neighbours[1].x && neighbours[0].y != neighbours[1].y {
                        graph.add_node(current_position);
                    }
                } else if neighbours_count > 2 {
                    graph.add_node(current_position);
                }
            }
        }
    }
}

pub fn get_edge_cost(grid: &Grid<char>, point_1: Point, point_2: Point, value_to_ignore: char) -> Option<u64> {
    let mut result = 0;

    if point_1.x == point_2.x {
        let range = if point_1.y < point_2.y {
            point_1.y..point_2.y
        } else {
            point_2.y..point_1.y
        };
        for y in range {
            let value = *grid.get(Point { x: point_1.x, y }).unwrap();
            if value == value_to_ignore {
                return None;
            }
            result += 1;
        }
        Some(result)
    } else if point_1.y == point_2.y {
        let range = if point_1.x < point_2.x {
            point_1.x..point_2.x
        } else {
            point_2.x..point_1.x
        };
        for x in range {
            let value = *grid.get(Point { x, y: point_1.y }).unwrap();
            if value == value_to_ignore {
                return None;
            }
            result += 1;
        }
        Some(result)
    } else {
        return None;
    }
}


pub fn add_edges_to_graph(grid: &Grid<char>, graph: &mut Graph<Point>, value_to_ignore: char) {
    let nodes_clone = graph.nodes.clone();
    let combinations = nodes_clone.keys().combinations(2).collect::<HashSet<_>>();
    for combination in combinations {
        let point_1_id = *combination[0];
        let point_2_id = *combination[1];
        let point_1 = graph.get_node(point_1_id).unwrap().value;
        let point_2 = graph.get_node(point_2_id).unwrap().value;

        if (graph.edges.contains_key(&point_1_id) && graph.edges.get(&point_1_id).unwrap().iter().any(|e| e.target == point_2_id)) || (graph.edges.contains_key(&point_2_id) && graph.edges.get(&point_2_id).unwrap().iter().any(|e| e.target == point_1_id)) {
            continue;
        }

        let cost = get_edge_cost(grid, point_1, point_2, value_to_ignore);
        if cost.is_some() {
            graph.add_edge(point_1_id, point_2_id, cost.unwrap());
            graph.add_edge(point_2_id, point_1_id, cost.unwrap());
        }
    }
}


pub fn rebuild_path_counting_nodes(graph: &Graph<Point>, predecessors: &HashMap<u32, Vec<u32>>, start_node: u32, end_node: u32) -> HashSet<Point> {
    fn backtrack(current: u32, start_node: u32, predecessors: &HashMap<u32, Vec<u32>>, visited: &mut HashSet<Point>, nodes: &HashMap<u32, Node<Point>>, visited_nodes: &mut HashSet<(u32, u32)>) {
        for &pred in &predecessors[&current] {
            if visited_nodes.contains(&(current, pred)) {
                continue;
            }

            let current_node = nodes.get(&current).unwrap().value;
            let pred_node = nodes.get(&pred).unwrap().value;
            if current_node.x == pred_node.x {
                let range = if current_node.y < pred_node.y {
                    current_node.y..pred_node.y + 1
                } else {
                    pred_node.y..current_node.y + 1
                };
                for y in range.rev() {
                    visited.insert(Point { x: current_node.x, y });
                }
            } else if current_node.y == pred_node.y {
                let range = if current_node.x < pred_node.x {
                    current_node.x..pred_node.x + 1
                } else {
                    pred_node.x..current_node.x + 1
                };
                for x in range.rev() {
                    visited.insert(Point { x, y: current_node.y });
                }
            } else {
                panic!("Both X and Y are different!")
            }

            backtrack(pred, start_node, predecessors, visited, nodes, visited_nodes);
            visited_nodes.insert((current,pred));
        }
    }

    let mut result = HashSet::new();
    let mut visited_nodes = HashSet::new();
    backtrack(end_node, start_node, predecessors, &mut result, &graph.nodes, &mut visited_nodes);
    result
}

#[allow(dead_code)]
pub fn bfs_distances<T: PartialEq + Eq + Copy>(input: &[Vec<T>], start: Point, wall_symbol: T) -> Vec<Vec<i32>> {
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;
    let mut dist = vec![vec![-1; cols as usize]; rows as usize];
    let mut q = VecDeque::new();

    dist[start.x as usize][start.y as usize] = 0;
    q.push_back(start);

    while let Some(p) = q.pop_front() {
        let d = dist[p.x as usize][p.y as usize];
        for (dx, dy) in DIFFS {
            let np = p.add(dx, dy);
            if np.x < 0 || np.y < 0 || np.x >= rows || np.y >= cols {
                continue;
            }
            let ch = input[np.x as usize][np.y as usize];
            if ch == wall_symbol { continue; }
            if dist[np.x as usize][np.y as usize] != -1 { continue; }

            dist[np.x as usize][np.y as usize] = d + 1;
            q.push_back(np);
        }
    }

    dist
}

pub fn init_logger(){
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

pub fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i32
}

pub fn bron_kerbosch<T>(r: &mut HashSet<T>, p: &mut HashSet<T>, x: &mut HashSet<T>, g: &HashMap<T, HashSet<T>>, cliques: &mut Vec<Vec<T>>)
where
    T: Default + Copy + Eq + PartialEq + Hash + Ord + PartialOrd
{
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            cliques.push(r.iter().sorted().map(|&x| x).collect_vec());
        }
        return;
    }

    let mut pivot = T::default();
    let mut max_neighbours = 0;
    for (k, v) in g {
        if (p.contains(k) || x.contains(k)) && v.len() > max_neighbours {
            max_neighbours = v.len();
            pivot = *k;
        }
    }

    let mut candidates = Vec::new();
    let neighbours = g[&pivot].clone();
    for vertex in p.iter() {
        if !neighbours.contains(vertex) {
            candidates.push(*vertex);
        }
    }

    for v in candidates {
        let mut new_r = r.clone();
        new_r.insert(v);

        let neighbours_v = g[&v].clone();
        let mut new_p = HashSet::new();
        for neighbour in neighbours_v.iter() {
            if p.contains(neighbour) {
                new_p.insert(*neighbour);
            }
        }

        let mut new_x = HashSet::new();
        for neighbour in neighbours_v.iter() {
            if x.contains(neighbour) {
                new_x.insert(*neighbour);
            }
        }

        bron_kerbosch(&mut new_r, &mut new_p, &mut new_x, g, cliques);

        p.remove(&v);
        x.insert(v);
    }
}