//! ポテンシャル付きUnionfind
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/unionfind_with_potential>
//! - <https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group>

use crate::algebra::traits::*;
use std::cell::{Cell, RefCell};

/// ポテンシャル付きUnionfind
pub struct PotentialUnionFind<T> {
    n: usize,
    count: usize,
    parent: Vec<Cell<usize>>,
    depth: Vec<usize>,
    size: Vec<usize>,
    potential: RefCell<Vec<T>>,
    inverse: Option<RefCell<Vec<T>>>,
}

impl<T> PotentialUnionFind<T>
where
    T: AbelianGroup + Clone,
{
    /// 大きさ`n`の[`PotentialUnionFind`]を生成する。(ポテンシャルが可換群のとき)
    pub fn new_commutative(n: usize) -> Self {
        Self {
            n,
            count: n,
            parent: (0..n).map(Cell::new).collect(),
            depth: vec![1; n],
            size: vec![1; n],
            potential: RefCell::new(vec![T::id(); n]),
            inverse: None,
        }
    }
}

impl<T> PotentialUnionFind<T>
where
    T: Group + Clone,
{
    /// 大きさ`n`の[`PotentialUnionFind`]を生成する。(ポテンシャルが可換とは限らない群のとき)
    pub fn new_non_commutative(n: usize) -> Self {
        Self {
            n,
            count: n,
            parent: (0..n).map(Cell::new).collect(),
            depth: vec![1; n],
            size: vec![1; n],
            potential: RefCell::new(vec![T::id(); n]),
            inverse: Some(RefCell::new(vec![T::id(); n])),
        }
    }

    /// `i`の属する素集合の根を返す。
    pub fn root_of(&self, i: usize) -> usize {
        if self.parent[i].get() == i {
            return i;
        }
        let p = self.parent[i].get();
        let p = self.root_of(p);

        let mut potential = self.potential.borrow_mut();
        let t = potential[self.parent[i].get()].clone();
        potential[i].op_assign_l(t);

        if let Some(inv) = self.inverse.as_ref() {
            let mut inv = inv.borrow_mut();
            let t = inv[self.parent[i].get()].clone();
            inv[i].op_assign_r(t);
        }

        self.parent[i].set(p);
        self.parent[i].get()
    }

    /// `i`のポテンシャル($P(i)$)を返す。
    pub fn potential_of(&self, i: usize) -> T {
        self.potential.borrow()[i].clone()
    }

    /// `i`と`j`が同じ素集合に属するならば`true`を返す。
    pub fn is_same(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }

    /// `i`と`j`が同一の素集合に属するとき、ポテンシャルの差($P(i) - P(j)$)を返す。
    pub fn diff(&self, i: usize, j: usize) -> Option<T> {
        self.is_same(i, j).then(|| {
            let pi = self.potential_of(i);
            if let Some(inv) = self.inverse.as_ref() {
                inv.borrow()[j].clone().op(pi)
            } else {
                self.potential_of(j).inv().op(pi)
            }
        })
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

        let mut potential = self.potential.borrow_mut();
        let (pi, pj) = (potential[i].clone(), potential[j].clone());

        if self.depth[ri] < self.depth[rj] {
            self.parent[ri].set(rj);
            self.size[rj] += self.size[ri];

            if let Some(inv) = self.inverse.as_ref() {
                let mut inv = inv.borrow_mut();
                potential[ri] = pj.op(p.clone()).op(inv[i].clone());
                inv[ri] = pi.op(p.inv()).op(inv[j].clone());
            } else {
                potential[ri] = pj.op(p).op(pi.inv());
            }

            j
        } else {
            self.parent[rj].set(ri);
            self.size[ri] += self.size[rj];

            if let Some(inv) = self.inverse.as_ref() {
                let mut inv = inv.borrow_mut();
                potential[rj] = pi.op(p.clone().inv()).op(inv[j].clone());
                inv[rj] = pj.op(p).op(inv[i].clone())
            } else {
                potential[rj] = pi.op(p.inv()).op(pj.inv());
            }

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
