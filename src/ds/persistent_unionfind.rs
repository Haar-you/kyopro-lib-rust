//! 永続UnionFind
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/persistent_unionfind>

use crate::ds::persistent_array::PersistentArray;

/// 永続UnionFind
#[derive(Clone)]
pub struct PersistentUnionFind {
    par: PersistentArray<isize>,
}

impl PersistentUnionFind {
    /// 大きさ`n`の[`PersistentUnionFind`]を返す。
    pub fn new(n: usize) -> Self {
        Self {
            par: PersistentArray::new(n, -1),
        }
    }

    /// `i`の属する集合の根を返す。
    pub fn root_of(&self, i: usize) -> usize {
        let p = self.par[i];
        if p < 0 {
            i
        } else {
            self.root_of(p as usize)
        }
    }

    /// `i`の属する集合の大きさを返す。
    pub fn size_of(&self, i: usize) -> usize {
        (-self.par[self.root_of(i)]) as usize
    }

    /// `i`と`j`が同じ集合に属するならば、`true`を返す。
    pub fn is_same(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }

    /// `i`の属する集合と`j`の属する集合を統合する。
    pub fn merge(&self, i: usize, j: usize) -> Self {
        let i = self.root_of(i);
        let j = self.root_of(j);

        if i == j {
            return self.clone();
        }

        let size_i = self.size_of(i);
        let size_j = self.size_of(j);

        let mut par = self.par.clone();

        if size_i > size_j {
            par = par.set(i, -((size_i + size_j) as isize));
            par = par.set(j, i as isize);
        } else {
            par = par.set(j, -((size_i + size_j) as isize));
            par = par.set(i, j as isize);
        }

        Self { par }
    }
}
