//! ポテンシャル付きUnionfind
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/unionfind_with_potential>
//! - <https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group>

use crate::algebra::traits::*;
use std::cell::{Cell, RefCell};

/// ポテンシャル付きUnionfind
pub struct PotentialUnionFind<G: Group> {
    group: G,
    n: usize,
    count: usize,
    parent: Vec<Cell<usize>>,
    depth: Vec<usize>,
    size: Vec<usize>,
    potential: RefCell<Vec<G::Element>>,
    inverse: Option<RefCell<Vec<G::Element>>>,
}

impl<G> PotentialUnionFind<G>
where
    G: AbelianGroup,
    G::Element: Clone,
{
    /// 大きさ`n`の[`PotentialUnionFind`]を生成する。(ポテンシャルが可換群のとき)
    pub fn new_commutative(group: G, n: usize) -> Self {
        Self {
            n,
            count: n,
            parent: (0..n).map(Cell::new).collect(),
            depth: vec![1; n],
            size: vec![1; n],
            potential: RefCell::new(vec![group.id(); n]),
            inverse: None,
            group,
        }
    }
}

impl<G> PotentialUnionFind<G>
where
    G: Group,
    G::Element: Clone,
{
    /// 大きさ`n`の[`PotentialUnionFind`]を生成する。(ポテンシャルが可換とは限らない群のとき)
    pub fn new_non_commutative(group: G, n: usize) -> Self {
        Self {
            n,
            count: n,
            parent: (0..n).map(Cell::new).collect(),
            depth: vec![1; n],
            size: vec![1; n],
            potential: RefCell::new(vec![group.id(); n]),
            inverse: Some(RefCell::new(vec![group.id(); n])),
            group,
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
        self.group.op_assign_l(&mut potential[i], t);

        if let Some(inv) = self.inverse.as_ref() {
            let mut inv = inv.borrow_mut();
            let t = inv[self.parent[i].get()].clone();
            self.group.op_assign_r(&mut inv[i], t);
        }

        self.parent[i].set(p);
        self.parent[i].get()
    }

    /// `i`のポテンシャル($P(i)$)を返す。
    pub fn potential_of(&self, i: usize) -> G::Element {
        self.potential.borrow()[i].clone()
    }

    /// `i`と`j`が同じ素集合に属するならば`true`を返す。
    pub fn is_same(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }

    /// `i`と`j`が同一の素集合に属するとき、ポテンシャルの差($P(i) - P(j)$)を返す。
    pub fn diff(&self, i: usize, j: usize) -> Option<G::Element> {
        self.is_same(i, j).then(|| {
            let pi = self.potential_of(i);
            if let Some(inv) = self.inverse.as_ref() {
                self.group.op(inv.borrow()[j].clone(), pi)
            } else {
                self.group.op(self.group.inv(self.potential_of(j)), pi)
            }
        })
    }

    /// `i`の属する素集合と`j`の属する素集合を統合する。
    /// 統合後は、$P(i) = P(j) + p$を満たす。
    pub fn merge(&mut self, i: usize, j: usize, p: G::Element) -> usize {
        let ri = self.root_of(i);
        let rj = self.root_of(j);

        if ri == rj {
            return ri;
        }

        self.count -= 1;

        let mut potential = self.potential.borrow_mut();
        let (pi, pj) = (potential[i].clone(), potential[j].clone());

        let g = &self.group;

        if self.depth[ri] < self.depth[rj] {
            self.parent[ri].set(rj);
            self.size[rj] += self.size[ri];

            if let Some(inv) = self.inverse.as_ref() {
                let mut inv = inv.borrow_mut();
                potential[ri] = g.fold_m([pj, p.clone(), inv[i].clone()]);
                inv[ri] = g.fold_m([pi, g.inv(p), inv[j].clone()]);
            } else {
                potential[ri] = g.fold_m([pj, p, g.inv(pi)]);
            }

            j
        } else {
            self.parent[rj].set(ri);
            self.size[ri] += self.size[rj];

            if let Some(inv) = self.inverse.as_ref() {
                let mut inv = inv.borrow_mut();
                potential[rj] = g.fold_m([pi, g.inv(p.clone()), inv[j].clone()]);
                inv[rj] = g.fold_m([pj, p, inv[i].clone()]);
            } else {
                potential[rj] = g.fold_m([pi, g.inv(p), g.inv(pj)]);
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
