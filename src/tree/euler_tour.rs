//! Euler tour
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/vertex_add_subtree_sum>

use crate::{misc::is_none_or::IsNoneOr, tree::*};

pub struct EulerTour {
    begin: Vec<usize>,
    end: Vec<usize>,
}

impl EulerTour {
    /// `root`を根として[`EulerTour`]を構築する。
    pub fn new<E: TreeEdgeTrait>(tree: &Tree<E>, root: usize) -> Self {
        let n = tree.len();
        let mut this = Self {
            begin: vec![0; n],
            end: vec![0; n],
        };
        this.dfs(tree, root, None, &mut 0);
        this
    }

    fn dfs<E: TreeEdgeTrait>(
        &mut self,
        tree: &Tree<E>,
        cur: usize,
        par: Option<usize>,
        pos: &mut usize,
    ) {
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
