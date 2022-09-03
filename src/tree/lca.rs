//! 最小共通祖先

use crate::tree::*;

pub struct DoublingLCA {
    log2n: usize,
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}

impl DoublingLCA {
    pub fn new<T>(tree: &Tree<T>, root: usize) -> Self {
        let n = tree.len();
        let log2n = ((n as f64).log2().ceil() + 1.0) as usize;
        let mut ret = Self {
            log2n,
            parent: vec![vec![None; log2n]; n],
            depth: vec![0; n],
        };

        ret.dfs(tree, root, None, 0);
        for k in 0..log2n - 1 {
            for v in 0..n {
                match ret.parent[v][k] {
                    Some(p) => ret.parent[v][k + 1] = ret.parent[p][k],
                    None => ret.parent[v][k + 1] = None,
                }
            }
        }

        ret
    }

    fn dfs<T>(&mut self, tree: &Tree<T>, cur: usize, par: Option<usize>, d: usize) {
        self.parent[cur][0] = par;
        self.depth[cur] = d;

        for &TreeEdge { to, .. } in tree.nodes[cur].neighbors() {
            if Some(to) != par {
                self.dfs(tree, to, Some(cur), d + 1);
            }
        }
    }

    pub fn ancestor(&self, mut a: usize, mut n: usize) -> Option<usize> {
        let bits = std::mem::size_of::<usize>() * 8;
        while n != 0 {
            let m1 = bits - n.leading_zeros() as usize - 1;
            if let Some(&Some(b)) = self.parent[a].get(m1) {
                a = b;
                n ^= 1 << m1;
            } else {
                return None;
            }
        }

        Some(a)
    }

    pub fn get_lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] >= self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        for k in 0..self.log2n {
            if ((self.depth[b] - self.depth[a]) >> k & 1) != 0 {
                b = self.parent[b][k].unwrap();
            }
        }
        if a == b {
            return a;
        }

        for k in (0..self.log2n).rev() {
            if self.parent[a][k] != self.parent[b][k] {
                a = self.parent[a][k].unwrap();
                b = self.parent[b][k].unwrap();
            }
        }

        self.parent[a][0].unwrap()
    }

    /// s-t最短パス上で、sから見てd番目の頂点を返す。
    pub fn jump(&self, s: usize, t: usize, d: usize) -> Option<usize> {
        let a = self.get_lca(s, t);
        if self.depth[s] - self.depth[a] >= d {
            self.ancestor(s, d)
        } else if self.depth[s] + self.depth[t] - self.depth[a] * 2 >= d {
            self.ancestor(t, self.depth[s] + self.depth[t] - self.depth[a] * 2 - d)
        } else {
            None
        }
    }
}
