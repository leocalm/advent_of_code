use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq, Clone, Hash, PartialOrd, Ord, Default, Copy)]
pub struct Node<T> {
    pub value: T,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, PartialOrd, Ord, Default, Copy)]
pub struct Edge {
    pub source: u32,
    pub target: u32,
    pub weight: u64,
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Graph<T: PartialEq + Eq + Copy + Hash> {
    current_node_id: u32,
    pub nodes: HashMap<u32, Node<T>>,
    pub nodes_cache: HashMap<Node<T>, u32>,
    pub edges: HashMap<u32, HashSet<Edge>>,
    pub simple_edges: HashMap<u32, HashSet<u32>>,
}

#[allow(dead_code)]
impl<T: PartialEq + Eq + Copy + Hash> Graph<T> {
    pub fn new() -> Self {
        Graph {
            current_node_id: 0,
            nodes: HashMap::new(),
            edges: HashMap::new(),
            nodes_cache: HashMap::new(),
            simple_edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, value: T) -> u32 {
        let new_node = Node { value };
        let existing = self.nodes.iter().filter(|(_, node)| **node == new_node).map(|(id, _)| *id).collect::<Vec<_>>();
        if existing.is_empty() {
            let id = self.current_node_id;
            self.current_node_id += 1;

            self.nodes.insert(id, new_node);
            self.nodes_cache.insert(new_node, id);

            id
        } else {
            *existing.first().unwrap()
        }
    }

    pub fn add_edge(&mut self, source: u32, target: u32, weight: u64) -> Edge {
        let new_edge = Edge { source, target, weight };
        self.edges.entry(source).or_insert_with(HashSet::new).insert(new_edge.clone());
        new_edge
    }

    pub fn add_simple_edge(&mut self, source: u32, target: u32) {
        self.simple_edges.entry(source).or_insert_with(HashSet::new).insert(target);
    }

    pub fn dijkstra(&self, start_node: u32) -> (HashMap<u32, u64>, HashMap<u32, Vec<u32>>) {
        let mut result: HashMap<u32, u64> = self.nodes.keys().map(|&id| (id, u64::MAX)).collect();
        let mut predecessors: HashMap<u32, Vec<u32>> = self.nodes.keys().map(|&id| (id, vec![])).collect();
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0u64), start_node));

        result.insert(start_node, 0);

        while let Some((Reverse(current_dist), node)) = heap.pop() {
            if result.contains_key(&node) && current_dist > result[&node] {
                continue;
            }

            if let Some(neighbors) = self.edges.get(&node) {
                for edge in neighbors {
                    let new_dist: u64 = current_dist.saturating_add(edge.weight);

                    if new_dist < *result.get(&edge.target).expect(format!("Error getting result for {}", edge.target).as_str()) {
                        result.insert(edge.target, new_dist);
                        predecessors.get_mut(&edge.target).unwrap().push(node);
                        heap.push((Reverse(new_dist), edge.target));
                    }
                }
            }
        }

        (result, predecessors)
    }

    pub fn get_node(&self, node_id: u32) -> Option<&Node<T>> {
        self.nodes.get(&node_id)
    }

    pub fn get_nodes(&self) -> &HashMap<u32, Node<T>> { &self.nodes }

    pub fn find_node(&self, value: T) -> Option<u32> {
        self.nodes.iter().find(|(_, v)| **v == Node { value }).map(|(k, _)| *k)
    }

    pub fn remove_node(&mut self, node_id: u32) {
        self.nodes.remove(&node_id);
    }

    pub fn remove_edge(&mut self, edge: &Edge) {
        let edges = self.edges.get_mut(&edge.source).unwrap();
        edges.remove(edge);
    }

    pub fn get_node_id(&self, node: Node<T>) -> Option<u32> {
        self.nodes_cache.get(&node).map(|id| *id)
    }

    pub fn get_connected_nodes(&self, node_id: u32) -> HashSet<u32> {
        let mut result = HashSet::new();
        for edge in self.edges.get(&node_id).unwrap() {
            result.insert(edge.target);
        }

        result
    }
}