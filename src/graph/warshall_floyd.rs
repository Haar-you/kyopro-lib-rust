//! 全頂点間最短経路長

#![allow(clippy::needless_range_loop)]

use crate::graph::*;
use crate::num::one_zero::Zero;
use std::ops::Add;

/// **Time complexity O(n³)**
pub fn warshall_floyd<D: Direction, T, E: EdgeTrait<Weight = T>>(
    g: &Graph<D, E>,
) -> Option<Vec<Vec<Option<T>>>>
where
    T: Copy + Ord + Add<Output = T> + Zero,
{
    let zero = T::zero();
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
            return None;
        }
    }

    Some(dist)
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
            warshall_floyd(&g),
            Some(vec![
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
            warshall_floyd(&g),
            Some(vec![
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

        assert_eq!(warshall_floyd(&g), None);
    }
}
