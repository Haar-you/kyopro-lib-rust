//! 全頂点間最短経路長
//!
//! # Problems
//! - <https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_C>
//! - <https://atcoder.jp/contests/abc375/tasks/abc375_f>

#![allow(clippy::needless_range_loop)]

use crate::graph::*;
use crate::num::one_zero::Zero;
use std::ops::Add;

enum Result<T> {
    NegativeLoop,
    Valid(Vec<Vec<Option<T>>>),
}

/// グラフの全頂点間の最短距離を管理する。
pub struct WarshallFloyd<T> {
    len: usize,
    result: Result<T>,
}

impl<T> WarshallFloyd<T>
where
    T: Copy + Ord + Add<Output = T> + Zero,
{
    /// `WarshallFloyd<T>`を生成する。
    ///
    /// **Time complexity** $O(n^3)$
    pub fn new<D: Direction, E: EdgeTrait<Weight = T>>(g: &Graph<D, E>) -> Self {
        let zero = E::Weight::zero();
        let n = g.len();
        let mut dist = vec![vec![None; n]; n];

        for i in 0..n {
            dist[i][i] = Some(zero);
        }
        for e in g.edges.iter().flatten() {
            dist[e.from()][e.to()] = Some(e.weight());
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if let (Some(a), Some(b)) = (dist[i][k], dist[k][j]) {
                        let s = a + b;
                        dist[i][j] = dist[i][j].map(|x| x.min(s)).or(Some(s));
                    }
                }
            }
        }

        for i in 0..n {
            if dist[i][i].unwrap() < zero {
                return Self {
                    len: n,
                    result: Result::NegativeLoop,
                };
            }
        }

        Self {
            len: n,
            result: Result::Valid(dist),
        }
    }

    /// `from`から`to`への最短距離を返す。
    pub fn dist(&self, from: usize, to: usize) -> Option<T> {
        match &self.result {
            Result::NegativeLoop => panic!(),
            Result::Valid(dist) => dist[from][to],
        }
    }

    /// 内部で保持している距離テーブルへの参照を返す。
    pub fn table(&self) -> Option<&Vec<Vec<Option<T>>>> {
        match &self.result {
            Result::NegativeLoop => None,
            Result::Valid(dist) => Some(dist),
        }
    }

    /// 負の閉路があれば`true`を返す。
    ///
    /// **Time complexity** $O(1)$
    pub fn has_negative_loop(&self) -> bool {
        matches!(self.result, Result::NegativeLoop)
    }

    /// 有向辺を追加して、最短距離を再計算する。
    ///
    /// **Time complexity** $O(n^2)$
    pub fn add_edge(&mut self, from: usize, to: usize, d: T) {
        match &mut self.result {
            Result::NegativeLoop => {}
            Result::Valid(dist) => {
                for i in 0..self.len {
                    for j in 0..self.len {
                        if let (Some(a), Some(b)) = (dist[i][from], dist[to][j]) {
                            let s = a + b + d;
                            dist[i][j] = dist[i][j].map(|x| x.min(s)).or(Some(s));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_C
        let mut g = Graph::<Directed, _>::new(4);
        g.extend(
            vec![
                (0, 1, 1),
                (0, 2, 5),
                (1, 2, 2),
                (1, 3, 4),
                (2, 3, 1),
                (3, 2, 7),
            ]
            .into_iter()
            .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );

        assert_eq!(
            WarshallFloyd::new(&g).table(),
            Some(&vec![
                vec![Some(0), Some(1), Some(3), Some(4)],
                vec![None, Some(0), Some(2), Some(3)],
                vec![None, None, Some(0), Some(1)],
                vec![None, None, Some(7), Some(0)]
            ])
        );

        let mut g = Graph::<Directed, _>::new(4);
        g.extend(
            vec![
                (0, 1, 1),
                (0, 2, -5),
                (1, 2, 2),
                (1, 3, 4),
                (2, 3, 1),
                (3, 2, 7),
            ]
            .into_iter()
            .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );

        assert_eq!(
            WarshallFloyd::new(&g).table(),
            Some(&vec![
                vec![Some(0), Some(1), Some(-5), Some(-4)],
                vec![None, Some(0), Some(2), Some(3)],
                vec![None, None, Some(0), Some(1)],
                vec![None, None, Some(7), Some(0)]
            ])
        );

        let mut g = Graph::<Directed, _>::new(4);
        g.extend(
            vec![
                (0, 1, 1),
                (0, 2, 5),
                (1, 2, 2),
                (1, 3, 4),
                (2, 3, 1),
                (3, 2, -7),
            ]
            .into_iter()
            .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );

        assert!(WarshallFloyd::new(&g).has_negative_loop());
    }
}
