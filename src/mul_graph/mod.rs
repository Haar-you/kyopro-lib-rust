//! 頂点倍加グラフ
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc132/tasks/abc132_e>
//! - <https://atcoder.jp/contests/wupc2012-closed/tasks/wupc2012_5>
//! - <https://yukicoder.me/problems/no/807>
//! - <https://atcoder.jp/contests/abc395/tasks/abc395_e>

pub mod dijkstra;

use std::collections::HashMap;
use std::hash::Hash;

/// [`MulGraph`]の辺
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Edge<V, W> {
    /// 始点
    pub from: V,
    /// 終点
    pub to: V,
    /// 重み
    pub weight: W,
}

impl<V, W> Edge<V, W> {
    /// 始点`from`、終点`to`、重さ`weight`の辺を生成する。
    pub fn new(from: V, to: V, weight: W) -> Self {
        Self { from, to, weight }
    }
}

/// 頂点倍加グラフ
#[derive(Clone, Default, Debug)]
pub struct MulGraph<V, W> {
    nodes: HashMap<V, Vec<Edge<V, W>>>,
}

impl<V, W> MulGraph<V, W>
where
    V: Hash + Eq + Copy,
    W: Copy,
{
    /// 空のグラフを作る。
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// `u`,`v`間に双方向に辺を張る。
    pub fn add_undirected(&mut self, u: V, v: V, weight: W) {
        self.add_directed(u, v, weight);
        self.add_directed(v, u, weight);
    }

    /// `from`から`to`への有向辺を張る。
    pub fn add_directed(&mut self, from: V, to: V, weight: W) {
        self.nodes
            .entry(from)
            .or_default()
            .push(Edge::new(from, to, weight));
    }

    /// 頂点`cur`の隣接辺への参照へのイテレータを返す。
    pub fn neighbours_of(&self, cur: V) -> impl Iterator<Item = &Edge<V, W>> {
        self.nodes.get(&cur).into_iter().flatten()
    }
}
