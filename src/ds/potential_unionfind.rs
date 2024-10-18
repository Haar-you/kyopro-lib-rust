//! ポテンシャル付きUnionfind
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/unionfind_with_potential>

use crate::num::one_zero::Zero;
use std::cell::Cell;
use std::ops::{Add, Sub};

pub struct PotentialUnionFind<T> {
    n: usize,
    count: usize,
    parent: Vec<Cell<usize>>,
    depth: Vec<usize>,
    size: Vec<usize>,
    potential: Vec<Cell<T>>,
}

impl<T> PotentialUnionFind<T>
where
    T: Zero + Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            count: n,
            parent: (0..n).map(Cell::new).collect(),
            depth: vec![1; n],
            size: vec![1; n],
            potential: vec![Cell::new(T::zero()); n],
        }
    }

    pub fn root_of(&self, i: usize) -> usize {
        if self.parent[i].get() == i {
            return i;
        }
        let p = self.parent[i].get();
        let p = self.root_of(p);

        let t = self.potential[self.parent[i].get()].get();
        self.potential[i].set(self.potential[i].get() + t);

        self.parent[i].set(p);
        self.parent[i].get()
    }

    pub fn potential_of(&self, i: usize) -> T {
        self.root_of(i);
        self.potential[i].get()
    }

    pub fn is_same(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }

    pub fn diff(&self, i: usize, j: usize) -> Option<T> {
        self.is_same(i, j)
            .then_some(self.potential_of(i) - self.potential_of(j))
    }

    pub fn merge(&mut self, i: usize, j: usize, p: T) -> usize {
        let ri = self.root_of(i);
        let rj = self.root_of(j);

        if ri == rj {
            return ri;
        }

        self.count -= 1;

        if self.depth[ri] < self.depth[rj] {
            self.parent[ri].set(rj);
            self.size[rj] += self.size[ri];

            let p = p - self.potential[i].get() + self.potential[j].get();
            self.potential[ri].set(p);
            j
        } else {
            self.parent[rj].set(ri);
            self.size[ri] += self.size[rj];

            let p = self.potential[i].get() - self.potential[j].get() - p;
            self.potential[rj].set(p);

            if self.depth[ri] == self.depth[rj] {
                self.depth[i] += 1;
            }

            i
        }
    }

    pub fn size_of(&self, i: usize) -> usize {
        self.size[self.root_of(i)]
    }

    pub fn count_groups(&self) -> usize {
        self.count
    }

    pub fn get_groups(&self) -> Vec<Vec<usize>> {
        let mut ret = vec![vec![]; self.n];

        for i in 0..self.n {
            ret[self.root_of(i)].push(i);
        }

        ret.into_iter().filter(|x| !x.is_empty()).collect()
    }
}
