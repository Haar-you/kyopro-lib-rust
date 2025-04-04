//! 最小共通祖先

use crate::{misc::is_none_or::IsNoneOr, tree::*};

/// ダブリングによる最小共通祖先
pub struct DoublingLCA {
    log2n: usize,
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}

impl DoublingLCA {
    /// **Time complexity** $O(n \log n)$
    ///
    /// **Space complexity** $O(n \log n)$
    pub fn new<E: TreeEdgeTrait>(tree: &Tree<E>, root: usize) -> Self {
        let n = tree.len();
        let log2n = n.next_power_of_two().trailing_zeros() as usize + 1;
        let mut this = Self {
            log2n,
            parent: vec![vec![None; log2n]; n],
            depth: vec![0; n],
        };

        let mut stack = vec![];
        stack.push((root, None, 0));

        while let Some((cur, par, d)) = stack.pop() {
            this.parent[cur][0] = par;
            this.depth[cur] = d;

            tree.nodes[cur]
                .neighbors()
                .filter(|e| par.is_none_or(|p| p != e.to()))
                .for_each(|e| stack.push((e.to(), Some(cur), d + 1)));
        }

        for k in 0..log2n - 1 {
            for v in 0..n {
                match this.parent[v][k] {
                    Some(p) => this.parent[v][k + 1] = this.parent[p][k],
                    None => this.parent[v][k + 1] = None,
                }
            }
        }

        this
    }

    /// `a`の`n`個上の祖先を求める。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn ancestor(&self, mut a: usize, mut n: usize) -> Option<usize> {
        while n != 0 {
            let m1 = usize::BITS as usize - n.leading_zeros() as usize - 1;
            if let Some(&Some(b)) = self.parent[a].get(m1) {
                a = b;
                n ^= 1 << m1;
            } else {
                return None;
            }
        }

        Some(a)
    }

    /// `a`と`b`の最小共通祖先を求める。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] >= self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        for k in 0..self.log2n {
            if (((self.depth[b] - self.depth[a]) >> k) & 1) != 0 {
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
    ///
    /// **Time complexity** $O(\log n)$
    pub fn jump(&self, s: usize, t: usize, d: usize) -> Option<usize> {
        let a = self.lca(s, t);
        if self.depth[s] - self.depth[a] >= d {
            self.ancestor(s, d)
        } else if self.depth[s] + self.depth[t] - self.depth[a] * 2 >= d {
            self.ancestor(t, self.depth[s] + self.depth[t] - self.depth[a] * 2 - d)
        } else {
            None
        }
    }
}
