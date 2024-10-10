//! 中国人郵便配達問題

#![allow(clippy::needless_range_loop)]

use crate::graph::*;
use crate::num::one_zero::Zero;
use std::ops::Add;

/// **Time complexity O(V²2ⱽ)**
pub fn chinese_postman_problem<T, E: EdgeTrait<Weight = T>>(g: &Graph<Undirected, E>) -> T
where
    T: Copy + Ord + Add<Output = T> + Zero,
{
    let n = g.len();
    let zero = T::zero();

    let mut dist = vec![vec![None; n]; n];

    for i in 0..n {
        dist[i][i] = Some(zero);
    }

    for e in g.edges.iter().flatten() {
        let (from, to, cost) = (e.from(), e.to(), e.weight());
        dist[from][to] = dist[from][to].map(|x| x.min(cost)).or(Some(cost));
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(x), Some(y)) = (dist[i][k], dist[k][j]) {
                    dist[i][j] = dist[i][j].map(|z| z.min(x + y)).or(Some(x + y))
                }
            }
        }
    }

    let odd: Vec<_> = g
        .edges
        .iter()
        .enumerate()
        .filter_map(|(i, es)| (es.len() % 2 == 1).then_some(i))
        .collect();
    let m = odd.len();

    let mut dp = vec![None; 1 << m];
    dp[0] = Some(zero);

    for i in 0..1 << m {
        for j in 0..m {
            for k in 0..j {
                if (i & (1 << j)) == 0 || (i & (1 << k)) == 0 {
                    continue;
                }

                if let Some(d) = dp[i ^ (1 << j) ^ (1 << k)] {
                    let d = d + dist[odd[j]][odd[k]].unwrap();
                    dp[i] = dp[i].map(|x| x.min(d)).or(Some(d));
                }
            }
        }
    }

    g.edges
        .iter()
        .flat_map(|es| {
            es.iter()
                .filter_map(|e| (e.from() <= e.to()).then_some(e.weight()))
        })
        .fold(dp[(1 << m) - 1].unwrap(), |x, y| x + y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/problems/DPL_2_B/
        let mut g = Graph::<Undirected, _>::new(4);
        g.extend(
            vec![(0, 1, 1), (0, 2, 2), (1, 3, 3), (2, 3, 4)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        assert_eq!(chinese_postman_problem(&g), 10);

        let mut g = Graph::<Undirected, _>::new(4);
        g.extend(
            vec![(0, 1, 1), (0, 2, 2), (1, 3, 3), (2, 3, 4), (1, 2, 5)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        assert_eq!(chinese_postman_problem(&g), 18);

        let mut g = Graph::<Undirected, _>::new(2);
        g.extend(
            vec![(0, 1, 1), (0, 1, 2), (0, 1, 3)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        assert_eq!(chinese_postman_problem(&g), 7);
    }
}
