//! Lowlink
use crate::graph::*;
use std::cmp::min;

/// Lowlink
#[derive(Debug, Clone)]
pub struct Lowlink {
    /// グラフの頂点数
    pub size: usize,
    /// DFSで頂点を訪れた順番
    pub ord: Vec<usize>,
    /// DFS木を葉方向に0回以上、後退辺を1回以下辿って到達可能な頂点の`ord`の最小値
    pub low: Vec<usize>,
    /// DFS木での親ノード
    pub par: Vec<Option<usize>>,
    /// DFS木での子ノード
    pub ch: Vec<Vec<usize>>,
    /// par, chのどちらにも属さないノード
    pub back: Vec<Vec<usize>>,
}

impl Lowlink {
    /// 無向グラフから[`Lowlink`]を構築する。
    pub fn new<E: EdgeTrait>(g: &Graph<Undirected, E>) -> Self {
        let n = g.len();
        let mut this = Self {
            size: n,
            ord: vec![0; n],
            low: vec![0; n],
            par: vec![None; n],
            ch: vec![vec![]; n],
            back: vec![vec![]; n],
        };

        let mut index = 0;
        let mut check = vec![false; n];
        for i in 0..n {
            this.dfs(g, i, None, &mut index, &mut check);
        }

        this
    }

    fn dfs<E: EdgeTrait>(
        &mut self,
        g: &Graph<Undirected, E>,
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
        let mut count_par = 0;

        for e in g.nodes[cur].edges.iter() {
            let to = e.to();
            if par.is_some_and(|p| p == to) {
                count_par += 1;
                if count_par == 1 {
                    continue;
                }
            }

            if !check[to] {
                self.ch[cur].push(to);
                self.dfs(g, to, Some(cur), index, check);
                self.low[cur] = min(self.low[cur], self.low[to]);
            } else {
                self.back[cur].push(to);
            }

            self.low[cur] = min(self.low[cur], self.ord[to]);
        }
    }
}
