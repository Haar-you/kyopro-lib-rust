use crate::graph::{scc::*, *};

pub struct TwoSat {
    size: usize,
    g: Graph<Edge<(), ()>>,
}

impl TwoSat {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            g: Graph::new(2 * size),
        }
    }

    fn check(&self, x: isize) -> usize {
        assert!(x != 0);
        assert!(x.abs() as usize <= self.size);

        if x > 0 {
            x as usize - 1
        } else {
            x.abs() as usize + self.size - 1
        }
    }

    pub fn add_if(&mut self, a: isize, b: isize) {
        self.g.add_directed(Some(Edge::new(self.check(a), self.check(b), (), ())));
    }

    pub fn add_or(&mut self, a: isize, b: isize) {
        self.add_if(-a, b);
        self.add_if(-b, a);
    }

    pub fn not_coexist(&mut self, a: isize, b: isize) {
        self.add_or(-a, -b);
    }

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
