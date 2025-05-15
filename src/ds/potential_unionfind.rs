//! ポテンシャル付きUnionfind
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/unionfind_with_potential>

use crate::num::one_zero::Zero;
use std::cell::Cell;
use std::ops::{Add, Sub};

/// ポテンシャル付きUnionfind
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
    /// 大きさ`n`の[`PotentialUnionFind`]を生成する。
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

    /// `i`の属する素集合の根を返す。
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

    /// `i`のポテンシャル($P(i)$)を返す。
    pub fn potential_of(&self, i: usize) -> T {
        self.potential[i].get()
    }

    /// `i`と`j`が同じ素集合に属するならば`true`を返す。
    pub fn is_same(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }

    /// `i`と`j`が同一の素集合に属するとき、ポテンシャルの差($P(i) - P(j)$)を返す。
    pub fn diff(&self, i: usize, j: usize) -> Option<T> {
        self.is_same(i, j)
            .then_some(self.potential_of(i) - self.potential_of(j))
    }

    /// `i`の属する素集合と`j`の属する素集合を統合する。
    /// 統合後は、$P(i) = P(j) + p$を満たす。
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

    /// `i`の属する素集合の大きさを返す。
    pub fn size_of(&self, i: usize) -> usize {
        self.size[self.root_of(i)]
    }

    /// 素集合の個数を返す。
    pub fn count_groups(&self) -> usize {
        self.count
    }

    /// 素集合をすべて列挙する。
    pub fn get_groups(&self) -> Vec<Vec<usize>> {
        let mut ret = vec![vec![]; self.n];

        for i in 0..self.n {
            ret[self.root_of(i)].push(i);
        }

        ret.into_iter().filter(|x| !x.is_empty()).collect()
    }
}
