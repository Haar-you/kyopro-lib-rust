//! 二部マッチング

use crate::flow::MaxFlow;

/// 最大フローによる最大二部マッチング
pub struct BipartiteMatching<F> {
    left: usize,
    right: usize,
    flow: F,
    s: usize,
    t: usize,
}

impl<F: MaxFlow<Cap = u64>> BipartiteMatching<F> {
    /// 左側の頂点数が`left`、右側の頂点数が`right`の空の二部グラフを用意する。
    pub fn new(left: usize, right: usize) -> Self {
        let mut flow = F::new(left + right + 2);
        let s = left + right;
        let t = s + 1;

        for i in 0..left {
            flow.add_edge(s, i, 1);
        }
        for i in 0..right {
            flow.add_edge(left + i, t, 1);
        }

        Self {
            left,
            right,
            flow,
            s,
            t,
        }
    }

    /// 左側`i`と右側`j`に辺を張る。
    pub fn add_edge(&mut self, i: usize, j: usize) {
        assert!(i < self.left);
        assert!(j < self.right);
        self.flow.add_edge(i, self.left + j, 1);
    }

    /// 最大マッチングの辺数を返す。
    pub fn matching(&mut self) -> u64 {
        self.flow.max_flow(self.s, self.t)
    }
}
