//! Tree depth query
//!
//! # References
//! - [https://niuez.github.io/posts/entry/2019/10/05/002503/](https://niuez.github.io/posts/entry/2019/10/05/002503/)
//! - [https://niuez.github.io/posts/dfs_bfs_et/](https://niuez.github.io/posts/dfs_bfs_et/)
//!
//! # Problems
//! - [yukicoder No.899 γatheree](https://yukicoder.me/problems/no/899)

use crate::tree::*;
use std::collections::VecDeque;

/// 根付き木において、同一の深さの頂点の区間に対して区間クエリができる。
pub struct TreeDepthQuery {
    par: Vec<Option<usize>>,
    depth: Vec<usize>,
    left: Vec<usize>,
    right: Vec<usize>,
    bfs_ord: Vec<Vec<usize>>,
    dfs_ord: Vec<Vec<usize>>,
    ord: Vec<usize>,
}

impl TreeDepthQuery {
    /// 根を`root`とする`tree`を基に、`TreeDepthQuery`を構築する。
    pub fn new<E: TreeEdgeTrait>(tree: &Tree<E>, root: usize) -> Self {
        let size = tree.len();
        let mut this = Self {
            par: vec![None; size],
            depth: vec![0; size],
            left: vec![0; size],
            right: vec![0; size],
            bfs_ord: vec![],
            dfs_ord: vec![],
            ord: vec![0; size],
        };

        this.dfs(tree, root, None, 0, &mut 0);

        let mut q = VecDeque::new();
        q.push_back((root, 0));
        let mut ord = 0;

        while let Some((i, d)) = q.pop_front() {
            if this.bfs_ord.len() <= d {
                this.bfs_ord.push(vec![]);
            }
            this.bfs_ord[d].push(ord);
            this.ord[i] = ord;
            ord += 1;

            for e in tree.nodes[i].neighbors() {
                if this.par[i].is_none_or(|p| p != e.to()) {
                    q.push_back((e.to(), d + 1));
                }
            }
        }

        this
    }

    fn dfs<E: TreeEdgeTrait>(
        &mut self,
        tree: &Tree<E>,
        cur: usize,
        par: Option<usize>,
        d: usize,
        ord: &mut usize,
    ) {
        self.par[cur] = par;
        self.depth[cur] = d;

        if self.dfs_ord.len() <= d {
            self.dfs_ord.push(vec![]);
        }
        self.dfs_ord[d].push(*ord);
        self.left[cur] = *ord;
        *ord += 1;

        for e in tree.nodes[cur].neighbors() {
            if par.is_none_or(|p| p != e.to()) {
                self.dfs(tree, e.to(), Some(cur), d + 1, ord);
            }
        }

        self.right[cur] = *ord;
    }

    /// 頂点`i`から深さ`d`の子孫頂点に対応する区間を返す。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn children_query(&self, i: usize, d: usize) -> Option<(usize, usize)> {
        let d = d + self.depth[i];
        if self.bfs_ord.len() > d {
            let l = match self.dfs_ord[d].binary_search(&self.left[i]) {
                Ok(x) | Err(x) => x,
            };
            let r = match self.dfs_ord[d].binary_search(&self.right[i]) {
                Ok(x) | Err(x) => x,
            };

            if l >= self.bfs_ord[d].len() {
                return None;
            }
            if r == 0 {
                return None;
            }

            Some((self.bfs_ord[d][l], self.bfs_ord[d][r - 1] + 1))
        } else {
            None
        }
    }

    /// 頂点`i`に対応する区間を返す。
    pub fn me_query(&self, i: usize) -> (usize, usize) {
        (self.ord[i], self.ord[i] + 1)
    }

    /// 頂点`i`の`k`個遡った祖先の頂点を返す。
    ///
    /// **Time complexity** $O(k)$
    pub fn ancestor(&self, i: usize, k: usize) -> Option<usize> {
        let mut p = i;
        for _ in 0..k {
            match self.par[p] {
                Some(x) => p = x,
                _ => return None,
            }
        }
        Some(p)
    }
}
