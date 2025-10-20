//! 閉路をただ一つだけもつN辺N頂点の連結無向グラフ。
//!
//! # References
//! - <https://en.wikipedia.org/wiki/Pseudoforest>
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc266/tasks/abc266_f>

use std::collections::VecDeque;

/// [`PseudoTree`]を構築するための構造体。
pub struct PseudoTreeBuilder {
    edge_num: usize,
    g: Vec<Vec<usize>>,
}

/// 閉路をただ一つだけもつN辺N頂点の連結無向グラフ。
pub struct PseudoTree {
    root: Vec<usize>,
    kind: Vec<Kind>,
}

/// [`PseudoTree`]の頂点の種類
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Kind {
    /// 閉路を構成する頂点。
    Loop,
    /// 閉路以外の頂点。
    Other,
}

impl Kind {
    /// `Loop`ならば`true`を返す。
    pub fn is_loop(self) -> bool {
        matches!(self, Self::Loop)
    }
}

impl PseudoTreeBuilder {
    /// 頂点数`n`の空のグラフを用意する。
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![vec![]; n],
            edge_num: 0,
        }
    }

    /// `u, v`間に無向辺を張る。
    pub fn add(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
        self.edge_num += 1;
    }

    /// [`PseudoTree`]を構築する。
    pub fn build(self) -> PseudoTree {
        assert_eq!(self.edge_num, self.g.len());
        let n = self.g.len();
        let mut indeg = vec![0; n];
        let mut visit = vec![false; n];
        let mut kind = vec![Kind::Loop; n];

        for &to in self.g.iter().flatten() {
            indeg[to] += 1;
        }

        let mut queue: VecDeque<_> = indeg
            .iter()
            .enumerate()
            .filter_map(|(i, &deg)| (deg == 1).then_some(i))
            .collect();

        while let Some(cur) = queue.pop_front() {
            kind[cur] = Kind::Other;
            if visit[cur] {
                continue;
            }
            visit[cur] = true;

            for &to in &self.g[cur] {
                if !visit[to] {
                    indeg[to] -= 1;

                    if indeg[to] == 1 {
                        queue.push_back(to);
                    }
                }
            }
        }

        let mut root = vec![0; n];

        for i in 0..n {
            if kind[i].is_loop() {
                root[i] = i;
                for &to in &self.g[i] {
                    if !kind[to].is_loop() {
                        self.__dfs(to, i, &mut root);
                    }
                }
            }
        }

        PseudoTree { root, kind }
    }

    fn __dfs(&self, cur: usize, par: usize, root: &mut [usize]) {
        root[cur] = root[par];

        for &to in &self.g[cur] {
            if to != par {
                self.__dfs(to, cur, root);
            }
        }
    }
}

impl PseudoTree {
    /// 頂点`i`から辺を辿って最初に到達する閉路頂点を返す。
    pub fn root_of(&self, i: usize) -> usize {
        self.root[i]
    }

    /// 頂点`i`の種類を返す。
    pub fn kind_of(&self, i: usize) -> Kind {
        self.kind[i]
    }
}
