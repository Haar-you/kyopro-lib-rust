//! Euler tour
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/vertex_add_subtree_sum>

use std::marker::PhantomData;

use crate::tree::*;

/// Euler tour
pub struct EulerTour<W, I> {
    begin: Vec<usize>,
    end: Vec<usize>,
    _phantom: PhantomData<(W, I)>,
}

impl<W, I> EulerTour<W, I> {
    /// `root`を根として[`EulerTour`]を構築する。
    pub fn new(tree: &Tree<W, I>, root: usize) -> Self {
        let n = tree.len();
        let mut this = Self {
            begin: vec![0; n],
            end: vec![0; n],
            _phantom: PhantomData,
        };
        this.dfs(tree, root, None, &mut 0);
        this
    }

    fn dfs(&mut self, tree: &Tree<W, I>, cur: usize, par: Option<usize>, pos: &mut usize) {
        self.begin[cur] = *pos;
        *pos += 1;

        for e in tree.nodes[cur].neighbors() {
            if par.is_none_or(|p| p != e.to()) {
                self.dfs(tree, e.to(), Some(cur), pos);
            }
        }

        self.end[cur] = *pos;
    }

    /// 頂点`i`の部分木に対応する範囲を返す。
    pub fn subtree_query(&self, i: usize) -> (usize, usize) {
        (self.begin[i], self.end[i])
    }

    /// 頂点`i`に対応する番号を返す。
    pub fn point_query(&self, i: usize) -> usize {
        self.begin[i]
    }
}
