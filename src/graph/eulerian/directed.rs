//! 有向グラフの(準)Eulerグラフの判定
use crate::graph::*;

/// 有向グラフでの一筆書き
#[derive(Clone)]
pub struct DirectedEulerianTrail<E: EdgeTrait> {
    size: usize,
    edge_count: usize,
    graph: Vec<Vec<E>>,
    indeg: Vec<i32>,
    outdeg: Vec<i32>,
}

impl<E: EdgeTrait + Clone> DirectedEulerianTrail<E> {
    /// 頂点数`size`のグラフを用意する。
    pub fn new(size: usize) -> Self {
        Self {
            size,
            edge_count: 0,
            graph: vec![vec![]; size],
            indeg: vec![0; size],
            outdeg: vec![0; size],
        }
    }

    /// 有向辺`e`を追加する。
    pub fn add_edge(&mut self, e: E) {
        self.indeg[e.to()] += 1;
        self.outdeg[e.from()] += 1;
        self.graph[e.from()].push(e);
        self.edge_count += 1;
    }

    fn dfs(&mut self, cur: usize, vs: &mut Vec<usize>, es: &mut Vec<E>) {
        while let Some(e) = self.graph[cur].pop() {
            let next = e.to();
            self.dfs(next, vs, es);
            es.push(e);
        }

        vs.push(cur);
    }

    /// グラフが一筆書き可能なら、その頂点列と辺列を`Some`に包んで返す。
    pub fn solve(mut self) -> Option<(Vec<usize>, Vec<E>)> {
        let mut in_count = 0;
        let mut out_count = 0;
        let mut start = None;

        for i in 0..self.size {
            match self.outdeg[i] - self.indeg[i] {
                0 => {}
                1 => {
                    out_count += 1;
                    start = Some(i);
                }
                -1 => in_count += 1,
                _ => return None,
            }
            if start.is_none() && !self.graph[i].is_empty() {
                start = Some(i);
            }
        }

        let start = start.unwrap_or(0);

        if !(in_count == 0 && out_count == 0 || in_count == 1 && out_count == 1) {
            return None;
        }

        let mut vs = vec![];
        let mut es = vec![];
        self.dfs(start, &mut vs, &mut es);
        if vs.len() == self.edge_count + 1 && es.len() == self.edge_count {
            vs.reverse();
            es.reverse();

            Some((vs, es))
        } else {
            None
        }
    }
}
