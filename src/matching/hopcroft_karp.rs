//! 二部マッチング (Hopcroft-Karp)

use std::collections::VecDeque;

/// Hopcroft-Karp法によって、最大二部マッチングを求める。
pub struct HopcroftKarp {
    left: usize,
    right: usize,
    edges: Vec<Vec<usize>>,
    pair_left: Vec<usize>,
    pair_right: Vec<usize>,
    dist: Vec<usize>,
}

const INF: usize = usize::MAX;

impl HopcroftKarp {
    /// 左側の頂点数が`left`、右側の頂点数が`right`の空の二部グラフを用意する。
    pub fn new(left: usize, right: usize) -> Self {
        Self {
            left,
            right,
            edges: vec![vec![]; left + 1],
            pair_left: vec![0; left + 1],
            pair_right: vec![0; right + 1],
            dist: vec![INF; left + 1],
        }
    }

    /// 左側`i`と右側`j`に辺を張る。
    pub fn add_edge(&mut self, i: usize, j: usize) {
        assert!(i < self.left);
        assert!(j < self.right);
        self.edges[i + 1].push(j + 1);
    }

    fn bfs(&mut self) -> bool {
        let mut q = VecDeque::new();

        self.dist.fill(INF);
        for l in 1..=self.left {
            if self.pair_left[l] == 0 {
                q.push_back(l);
                self.dist[l] = 0;
            }
        }

        while let Some(l) = q.pop_front() {
            if self.dist[l] < self.dist[0] {
                for &r in &self.edges[l] {
                    let t = self.pair_right[r];
                    if self.dist[t] == INF {
                        self.dist[t] = self.dist[l] + 1;
                        q.push_back(t);
                    }
                }
            }
        }

        self.dist[0] != INF
    }

    fn dfs(&mut self, cur: usize) -> bool {
        if cur != 0 {
            let l = cur;
            for i in 0..self.edges[l].len() {
                let r = self.edges[l][i];

                if self.dist[self.pair_right[r]] == self.dist[l] + 1 && self.dfs(self.pair_right[r])
                {
                    self.pair_right[r] = l;
                    self.pair_left[l] = r;
                    return true;
                }
            }

            self.dist[cur] = INF;
            return false;
        }

        true
    }

    /// 最大マッチングの辺集合を返す。
    pub fn matching(&mut self) -> Vec<(usize, usize)> {
        while self.bfs() {
            for l in 1..=self.left {
                if self.pair_left[l] == 0 {
                    self.dfs(l);
                }
            }
        }

        (1..=self.left)
            .filter_map(|l| {
                let r = self.pair_left[l];
                (r != 0).then(|| (l - 1, r - 1))
            })
            .collect()
    }
}
