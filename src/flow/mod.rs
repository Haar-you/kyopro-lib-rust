//! フロー問題

pub mod dinic;
pub mod ford_fulkerson;

pub mod min_cost_flow;

/// 最大フロー問題を扱うトレイト。
pub trait MaxFlow {
    /// 容量の型
    type Cap;
    /// 頂点数`n`の空のグラフを返す。
    fn new(n: usize) -> Self;
    /// 頂点`u`から頂点`v`へ容量`cap`の辺を張る。
    fn add_edge(&mut self, u: usize, v: usize, cap: Self::Cap);
    /// 頂点`s`から頂点`t`への最大フローを求める。
    fn max_flow(&mut self, s: usize, t: usize) -> Self::Cap;
    /// 最大フローを達成するのに通った辺を返す。
    fn get_edges(&self, i: usize) -> Vec<(usize, Self::Cap)>;
    fn reset(&mut self);
}
