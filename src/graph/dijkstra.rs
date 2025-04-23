//! 非負重み付き最短経路 (Dijkstra)

use crate::graph::*;
use crate::num::{one_zero::Zero, traits::Unsigned};
use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

/// ダイクストラ法
pub struct Dijkstra<'a, W, E> {
    dist: Vec<Option<W>>,
    prev: Vec<Option<&'a E>>,
}

impl<'a, E> Dijkstra<'a, E::Weight, E>
where
    E: EdgeTrait,
    E::Weight: Add<Output = E::Weight> + Copy + Ord + Zero + Unsigned,
{
    /// グラフ`g`上で、始点から各頂点への最短パスを求める。
    ///
    /// **Time complexity** $O((E + V) \log V)$
    pub fn new(g: &'a Graph<impl Direction, E>, src: &[usize]) -> Self {
        let zero = E::Weight::zero();
        let n = g.len();
        let mut dist = vec![None; n];
        let mut heap = BinaryHeap::new();
        let mut check = vec![false; n];
        let mut prev = vec![None; n];

        for &u in src {
            dist[u] = Some(zero);
            heap.push(Reverse((zero, u)));
        }

        while let Some(Reverse((d, u))) = heap.pop() {
            if check[u] {
                continue;
            }
            check[u] = true;

            for e in g.nodes[u].edges.iter() {
                let (to, cost) = (e.to(), e.weight());

                match dist[to] {
                    Some(d2) if d2 <= d + cost => {}
                    _ => {
                        let d = d + cost;
                        dist[to] = Some(d);
                        prev[to] = Some(e);
                        if !check[to] {
                            heap.push(Reverse((d, to)));
                        }
                    }
                }
            }
        }

        Self { dist, prev }
    }

    /// 最短距離の配列への参照を返す。
    pub fn min_dist_table(&self) -> &[Option<E::Weight>] {
        &self.dist
    }

    /// `to`への最短距離を返す。
    /// 到達不可能ならば、`None`を返す。
    pub fn min_dist_to(&self, to: usize) -> Option<E::Weight> {
        self.dist[to]
    }

    /// `to`への最短パスを返す。
    /// 到達不可能ならば、`None`を返す。
    pub fn min_path_to(&self, mut to: usize) -> Option<Vec<&E>> {
        self.dist[to]?;
        let mut ret = vec![];
        loop {
            let Some(e) = self.prev[to] else { break };
            ret.push(e);
            to = e.from();
        }
        ret.reverse();
        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grl_1_a() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_A

        // sample 1
        let mut graph = Graph::<Directed, Edge<u32, ()>>::new(4);
        graph.extend(
            vec![(0, 1, 1), (0, 2, 4), (1, 2, 2), (2, 3, 1), (1, 3, 5)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        let ans = Dijkstra::new(&graph, &[0]);

        assert_eq!(ans.min_dist_table(), [Some(0), Some(1), Some(3), Some(4)]);

        // sample 2
        let mut graph = Graph::<Directed, Edge<u32, ()>>::new(4);
        graph.extend(
            vec![
                (0, 1, 1),
                (0, 2, 4),
                (2, 0, 1),
                (1, 2, 2),
                (3, 1, 1),
                (3, 2, 5),
            ]
            .into_iter()
            .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        let ans = Dijkstra::new(&graph, &[1]);

        assert_eq!(ans.min_dist_table(), [Some(3), Some(0), Some(2), None]);
    }
}
