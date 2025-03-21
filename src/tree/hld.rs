//! 重軽分解
use crate::tree::*;
use std::cmp::max;

/// 重軽分解
#[derive(Clone, Debug)]
pub struct HLD {
    _size: usize,
    par: Vec<Option<usize>>,
    head: Vec<usize>,
    id: Vec<usize>,
    rid: Vec<usize>,
    next: Vec<Option<usize>>,
    end: Vec<usize>,
}

impl HLD {
    /// **Time complexity** $O(n)$
    ///
    /// **Space complexity** $O(n)$
    pub fn new<E: TreeEdgeTrait>(tree: &Tree<E>, root: usize) -> Self {
        let size = tree.len();
        let mut ret = Self {
            _size: size,
            par: vec![None; size],
            head: vec![0; size],
            id: vec![0; size],
            rid: vec![0; size],
            next: vec![None; size],
            end: vec![0; size],
        };

        let mut tr = vec![vec![]; size];
        for (i, nodes) in tree.nodes.iter().enumerate() {
            for e in nodes.neighbors() {
                tr[i].push(e.to());
            }
        }

        ret.dfs_sub(&mut tr, root, None, &mut vec![1; size]);
        ret.dfs_build(&tr, root, &mut 0);
        ret
    }

    fn dfs_sub(
        &mut self,
        tree: &mut [Vec<usize>],
        cur: usize,
        par: Option<usize>,
        sub: &mut Vec<usize>,
    ) {
        self.par[cur] = par;
        tree[cur].retain(|&x| Some(x) != par);

        let mut t = 0;
        let n = tree[cur].len();
        for i in 0..n {
            let to = tree[cur][i];
            self.dfs_sub(tree, to, Some(cur), sub);
            sub[cur] += sub[to];
            if sub[to] > t {
                t = sub[to];
                self.next[cur] = Some(to);
                tree[cur].swap(i, 0);
            }
        }
    }

    fn dfs_build(&mut self, tree: &[Vec<usize>], cur: usize, index: &mut usize) {
        self.id[cur] = *index;
        self.rid[*index] = cur;
        *index += 1;

        for (i, &to) in tree[cur].iter().enumerate() {
            self.head[to] = if i == 0 { self.head[cur] } else { to };
            self.dfs_build(tree, to, index);
        }

        self.end[cur] = *index;
    }

    /// 演算は可換性を仮定する。
    ///
    /// **Time complexity** $O(\log n)$
    pub fn path_query_vertex(&self, mut x: usize, mut y: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        loop {
            if self.id[x] > self.id[y] {
                std::mem::swap(&mut x, &mut y);
            }
            ret.push((max(self.id[self.head[y]], self.id[x]), self.id[y] + 1));
            if self.head[x] == self.head[y] {
                break;
            }
            y = self.par[self.head[y]].unwrap();
        }
        ret
    }

    /// **Time complexity** $O(\log n)$
    pub fn path_query_edge(&self, mut x: usize, mut y: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        loop {
            if self.id[x] > self.id[y] {
                std::mem::swap(&mut x, &mut y);
            }
            if self.head[x] == self.head[y] {
                if x != y {
                    ret.push((self.id[x] + 1, self.id[y] + 1));
                }
                break;
            }
            ret.push((self.id[self.head[y]], self.id[y] + 1));
            y = self.par[self.head[y]].unwrap();
        }
        ret
    }

    /// **Time complexity** $O(1)$
    pub fn subtree_query_vertex(&self, x: usize) -> (usize, usize) {
        (self.id[x], self.end[x])
    }

    /// **Time complexity** $O(1)$
    pub fn subtree_query_edge(&self, x: usize) -> (usize, usize) {
        (self.id[x] + 1, self.end[x])
    }

    /// **Time complexity** $O(1)$
    pub fn parent(&self, x: usize) -> Option<usize> {
        self.par[x]
    }

    /// **Time complexity** $O(1)$
    pub fn get_id(&self, x: usize) -> usize {
        self.id[x]
    }

    /// **Time complexity** $O(1)$
    pub fn get_edge_id(&self, u: usize, v: usize) -> Option<usize> {
        if self.par[u] == Some(v) {
            Some(self.id[u])
        } else if self.par[v] == Some(u) {
            Some(self.id[v])
        } else {
            None
        }
    }

    /// **Time complexity** $O(\log n)$
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        loop {
            if self.id[u] > self.id[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if self.head[u] == self.head[v] {
                return u;
            }
            v = self.par[self.head[v]].unwrap();
        }
    }
}
