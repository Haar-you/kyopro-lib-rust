//! 最小費用流

use std::{
    cmp::{min, Reverse},
    collections::BinaryHeap,
};

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    rev: usize,
    cap: u64,
    cost: i64,
    is_rev: bool,
}

/// 最小費用流
#[derive(Clone)]
pub struct MinCostFlow {
    size: usize,
    edges: Vec<Vec<Edge>>,
}

impl MinCostFlow {
    /// 頂点数`size`の空の[`MinCostFlow`]を返す。
    pub fn new(size: usize) -> Self {
        Self {
            size,
            edges: vec![vec![]; size],
        }
    }

    /// 頂点`u`から頂点`v`に容量`cap`・費用`cost`の辺を張る。
    pub fn add_edge(&mut self, u: usize, v: usize, cap: u64, cost: i64) {
        let rev = self.edges[v].len();
        self.edges[u].push(Edge {
            to: v,
            rev,
            cap,
            cost,
            is_rev: false,
        });
        let rev = self.edges[u].len() - 1;
        self.edges[v].push(Edge {
            to: u,
            rev,
            cap: 0,
            cost: -cost,
            is_rev: true,
        });
    }

    /// 始点`src`から終点`sink`へ流量`f`を流して、その最小費用を求める。
    ///
    /// 流量`f`を流しきれるとき、`Ok`に最小費用を包んで返す。
    /// そうでないとき、`Err`に流せた流量とその最小費用のペアを包んで返す。
    pub fn min_cost_flow(&mut self, src: usize, sink: usize, f: u64) -> Result<i64, (u64, i64)> {
        let mut ret = 0;
        let mut flow = f;
        let mut h = vec![0; self.size];
        let mut prev = vec![(0, 0); self.size];
        let mut pq = BinaryHeap::<Reverse<(i64, usize)>>::new();

        while flow > 0 {
            let mut cost = vec![None; self.size];

            cost[src] = Some(0);
            pq.push(Reverse((0, src)));

            while let Some(Reverse((c, v))) = pq.pop() {
                if cost[v].unwrap() < c {
                    continue;
                }

                for (i, e) in self.edges[v].iter().enumerate() {
                    if e.cap > 0
                        && (cost[e.to].is_none()
                            || cost[e.to].unwrap() + h[e.to] > cost[v].unwrap() + h[v] + e.cost)
                    {
                        cost[e.to] = Some(cost[v].unwrap() + e.cost + h[v] - h[e.to]);
                        prev[e.to] = (v, i);
                        pq.push(Reverse((cost[e.to].unwrap(), e.to)));
                    }
                }
            }

            if cost[sink].is_none() {
                break;
            }

            for i in 0..self.size {
                if let Some(x) = cost[i] {
                    h[i] += x;
                }
            }

            let mut df = flow;
            let mut cur = sink;
            while cur != src {
                df = min(df, self.edges[prev[cur].0][prev[cur].1].cap);
                cur = prev[cur].0;
            }

            flow -= df;
            ret += df as i64 * h[sink];

            let mut cur = sink;
            while cur != src {
                let e = &mut self.edges[prev[cur].0][prev[cur].1];
                e.cap -= df;
                let rev = e.rev;
                self.edges[cur][rev].cap += df;
                cur = prev[cur].0;
            }
        }

        if flow == 0 {
            Ok(ret)
        } else {
            Err((f - flow, ret))
        }
    }

    /// 頂点`i`に接続する辺を、`(終点, 流量, 費用)`の形で返す。
    pub fn get_edges(&self, i: usize) -> Vec<(usize, u64, i64)> {
        self.edges[i]
            .iter()
            .filter(|e| !e.is_rev)
            .map(|e| (e.to, e.cap, e.cost))
            .collect()
    }
}
