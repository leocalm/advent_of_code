use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct DSU<T: Copy + Eq + Hash + Debug> {
    parents: HashMap<T, T>,
    sizes: HashMap<T, usize>,
}

impl<T: Copy + Eq + Hash + Debug> DSU<T> {
    pub fn new(points: &Vec<T>) -> Self {
        let mut parents = HashMap::new();
        let mut sizes = HashMap::new();
        for point in points {
            parents.insert(*point, *point);
            sizes.insert(*point, 1);
        }

        Self { parents, sizes }
    }

    pub fn add_new_node(&mut self, node: T) {
        self.parents.insert(node, node);
        self.sizes.insert(node, 1);
    }

    pub fn union(&mut self, node: T, other: T) {
        let other_parent = self.find(other).unwrap();
        let node_parent = self.find(node).unwrap();
        if node_parent == other_parent {
            return;
        }

        if *self.sizes.get(&node_parent).unwrap() < *self.sizes.get(&other_parent).unwrap() {
            self.parents.insert(node_parent, other_parent);
            self.sizes.insert(other_parent, *self.sizes.get(&node_parent).unwrap() + 1);
        } else {
            self.parents.insert(other_parent, node_parent);
            self.sizes.insert(node_parent, *self.sizes.get(&other_parent).unwrap() + 1);
        }
    }

    pub fn find(& mut self, child: T) -> Option<T> {
        if self.parents.get(&child).is_some_and(|&c| c == child) {
            Some(child)
        } else if let Some(parent) = self.parents.get(&child) {
            let root = self.find(*parent).unwrap();
            self.parents.insert(child, root);
            Some(root)
        } else {
            None
        }
    }
}
