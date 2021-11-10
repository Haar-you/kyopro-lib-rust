use crate::graph::*;
use std::cmp::min;

pub struct Lowlink {
    pub size: usize,
    pub ord: Vec<usize>,
    pub low: Vec<usize>,
    pub par: Vec<Option<usize>>,
    pub ch: Vec<Vec<usize>>,
}

impl Lowlink {
    pub fn new<T>(g: &Graph<T>) -> Self {
        let n = g.len();
        let mut ret = Self {
            size: n,
            ord: vec![0; n],
            low: vec![0; n],
            par: vec![None; n],
            ch: vec![vec![]; n],
        };

        let mut index = 0;
        let mut check = vec![false; n];
        for i in 0..n {
            ret.dfs(&g, i, None, &mut index, &mut check);
        }

        ret
    }

    fn dfs<T>(
        &mut self,
        g: &Graph<T>,
        cur: usize,
        par: Option<usize>,
        index: &mut usize,
        check: &mut [bool],
    ) {
        if check[cur] {
            return;
        }
        check[cur] = true;

        self.par[cur] = par;
        self.ord[cur] = *index;
        self.low[cur] = *index;
        *index += 1;

        for &Edge { to, .. } in &g.edges[cur] {
            if par.map_or(false, |p| p == to) {
                continue;
            }

            if !check[to] {
                self.ch[cur].push(to);
                self.dfs(g, to, Some(cur), index, check);
                self.low[cur] = min(self.low[cur], self.low[to]);
            }

            self.low[cur] = min(self.low[cur], self.ord[to]);
        }
    }
}
