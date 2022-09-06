use crate::tree::*;
use std::collections::VecDeque;

/// Tree depth query
///
/// # References
/// - [https://niuez.github.io/posts/entry/2019/10/05/002503/](https://niuez.github.io/posts/entry/2019/10/05/002503/)
/// - [https://niuez.github.io/posts/dfs_bfs_et/](https://niuez.github.io/posts/dfs_bfs_et/)
///
/// # Problems
/// - [yukicoder No.899 γatheree](https://yukicoder.me/problems/no/899)

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
    pub fn new<E: TreeEdgeTrait>(tree: &Tree<E>, root: usize) -> Self {
        let size = tree.len();
        let mut ret = Self {
            par: vec![None; size],
            depth: vec![0; size],
            left: vec![0; size],
            right: vec![0; size],
            bfs_ord: vec![],
            dfs_ord: vec![],
            ord: vec![0; size],
        };

        ret.dfs(&tree, root, None, 0, &mut 0);

        let mut q = VecDeque::new();
        q.push_back((root, 0));
        let mut ord = 0;

        while let Some((i, d)) = q.pop_front() {
            if ret.bfs_ord.len() <= d {
                ret.bfs_ord.push(vec![]);
            }
            ret.bfs_ord[d].push(ord);
            ret.ord[i] = ord;
            ord += 1;

            for e in tree.nodes[i].neighbors() {
                if Some(e.to()) != ret.par[i] {
                    q.push_back((e.to(), d + 1));
                }
            }
        }

        ret
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
            if Some(e.to()) != par {
                self.dfs(tree, e.to(), Some(cur), d + 1, ord);
            }
        }

        self.right[cur] = *ord;
    }

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

    pub fn me_query(&self, i: usize) -> (usize, usize) {
        (self.ord[i], self.ord[i] + 1)
    }

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
