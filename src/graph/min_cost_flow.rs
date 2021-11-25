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

#[derive(Clone)]
pub struct MinCostFlow {
    size: usize,
    edges: Vec<Vec<Edge>>,
}

impl MinCostFlow {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            edges: vec![vec![]; size],
        }
    }

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

    pub fn min_cost_flow(&mut self, s: usize, t: usize, f: u64) -> Result<i64, (u64, i64)> {
        let mut ret = 0;
        let mut flow = f;
        let mut h = vec![0; self.size];
        let mut prev = vec![(0, 0); self.size];
        let mut pq = BinaryHeap::<Reverse<(i64, usize)>>::new();

        while flow > 0 {
            let mut cost = vec![None; self.size];

            cost[s] = Some(0);
            pq.push(Reverse((0, s)));

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

            if cost[t].is_none() {
                break;
            }

            for i in 0..self.size {
                if let Some(x) = cost[i] {
                    h[i] += x;
                }
            }

            let mut df = flow;
            let mut cur = t;
            while cur != s {
                df = min(df, self.edges[prev[cur].0][prev[cur].1].cap);
                cur = prev[cur].0;
            }

            flow -= df;
            ret += df as i64 * h[t];

            let mut cur = t;
            while cur != s {
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
}
