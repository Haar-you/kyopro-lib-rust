//! 最大流 (Dinic)
//!
//! # References
//! - <https://misawa.github.io/others/flow/dinic_time_complexity.html>

pub use crate::flow::*;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    rev: usize,
    cap: u64,
    is_rev: bool,
}

/// Dinic法
#[derive(Clone)]
pub struct Dinic {
    _size: usize,
    edges: Vec<Vec<Edge>>,
    first_edge: Vec<usize>,
    level: Vec<usize>,
}

impl Dinic {
    fn dfs(&mut self, cur: usize, sink: usize, mut min_cap: u64) -> u64 {
        if cur == sink {
            min_cap
        } else {
            let mut f = 0;

            for i in self.first_edge[cur]..self.edges[cur].len() {
                let e = self.edges[cur][i].clone();
                if e.cap > 0 && self.level[e.to] > self.level[cur] {
                    let df = self.dfs(e.to, sink, min_cap.min(e.cap));

                    min_cap -= df;

                    let e = &mut self.edges[cur][i];
                    e.cap -= df;

                    let Edge { to, rev, .. } = *e;
                    self.edges[to][rev].cap += df;

                    f += df;

                    let e = &self.edges[cur][i];
                    if df > 0 && e.cap > 0 {
                        self.first_edge[cur] = i;
                        return f;
                    }
                }
            }

            self.first_edge[cur] = self.edges[cur].len();

            f
        }
    }
}

impl MaxFlow for Dinic {
    type Cap = u64;
    fn new(size: usize) -> Self {
        Self {
            _size: size,
            edges: vec![vec![]; size],
            first_edge: vec![0; size],
            level: vec![0; size],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: Self::Cap) {
        let rev = self.edges[v].len();
        self.edges[u].push(Edge {
            to: v,
            rev,
            cap,
            is_rev: false,
        });
        let rev = self.edges[u].len() - 1;
        self.edges[v].push(Edge {
            to: u,
            rev,
            cap: 0,
            is_rev: true,
        });
    }

    fn max_flow(&mut self, s: usize, t: usize) -> Self::Cap {
        let mut f = 0;

        loop {
            self.level.fill(0);
            self.level[s] = 1;
            let mut q = VecDeque::new();
            q.push_back(s);

            while let Some(cur) = q.pop_front() {
                for e in &self.edges[cur] {
                    if self.level[e.to] == 0 && e.cap > 0 {
                        self.level[e.to] = self.level[cur] + 1;
                        q.push_back(e.to);
                    }
                }
            }

            if self.level[t] == 0 {
                break f;
            }

            self.first_edge.fill(0);
            f += self.dfs(s, t, u64::MAX);
        }
    }

    fn get_edges(&self, i: usize) -> Vec<(usize, u64)> {
        self.edges[i]
            .iter()
            .filter(|e| !e.is_rev)
            .map(|e| (e.to, e.cap))
            .collect()
    }

    fn reset(&mut self) {
        todo!();
    }
}
