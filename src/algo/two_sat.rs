//! 2-SAT
use crate::graph::{scc::*, *};

pub struct TwoSat {
    size: usize,
    g: Graph<Directed, Edge<(), ()>>,
}

impl TwoSat {
    /// **Time complexity O(size)**
    pub fn new(size: usize) -> Self {
        Self {
            size,
            g: Graph::new(2 * size),
        }
    }

    fn check(&self, x: isize) -> usize {
        assert!(x != 0);
        assert!(x.unsigned_abs() <= self.size);

        if x > 0 {
            x as usize - 1
        } else {
            x.unsigned_abs() + self.size - 1
        }
    }

    /// a → b
    pub fn add_if(&mut self, a: isize, b: isize) {
        self.g.add(Edge::new(self.check(a), self.check(b), (), ()));
    }

    /// a ∨ b
    pub fn add_or(&mut self, a: isize, b: isize) {
        self.add_if(-a, b);
        self.add_if(-b, a);
    }

    /// ¬(a ∧ b)
    pub fn not_coexist(&mut self, a: isize, b: isize) {
        self.add_or(-a, -b);
    }

    /// **Time complexity O(size + E)**
    pub fn solve(&self) -> Option<Vec<bool>> {
        let s = SCC::new(&self.g).to_vec();

        for i in 0..self.size {
            if s[i] == s[i + self.size] {
                return None;
            }
        }

        Some((0..self.size).map(|i| s[i] > s[i + self.size]).collect())
    }
}
