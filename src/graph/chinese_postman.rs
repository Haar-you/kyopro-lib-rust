//! 中国人郵便配達問題

#![allow(clippy::needless_range_loop)]

use crate::graph::*;
use std::{cmp::min, ops::Add};

/// Time complexity O(V ^ 2 * 2 ^ V)
pub fn chinese_postman_problem<T, E: EdgeTrait<Weight = T>>(g: &Graph<Undirected, E>) -> T
where
    T: Default + Copy + Ord + Add<Output = T>,
{
    let n = g.len();

    let mut dist = vec![vec![None; n]; n];

    for i in 0..n {
        dist[i][i] = Some(T::default());
    }

    for i in 0..n {
        for e in &g.edges[i] {
            let (from, to, cost) = (e.from(), e.to(), e.weight());
            dist[from][to] = Some(dist[from][to].map_or(cost, |x| min(x, cost)));
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(x), Some(y)) = (dist[i][k], dist[k][j]) {
                    dist[i][j] = Some(dist[i][j].map_or(x + y, |z| min(z, x + y)));
                }
            }
        }
    }

    let mut odd = vec![];
    for i in 0..n {
        if g.edges[i].len() % 2 == 1 {
            odd.push(i);
        }
    }

    let m = odd.len();

    let mut dp = vec![None; 1 << m];
    dp[0] = Some(T::default());

    for i in 0..1 << m {
        for j in 0..m {
            for k in 0..j {
                if (i & (1 << j)) == 0 || (i & (1 << k)) == 0 {
                    continue;
                }

                if let Some(d) = dp[i ^ (1 << j) ^ (1 << k)] {
                    let d = d + dist[odd[j]][odd[k]].unwrap();
                    dp[i] = Some(dp[i].map_or(d, |x| min(x, d)));
                }
            }
        }
    }

    let mut ret = T::default();

    for i in 0..n {
        for e in &g.edges[i] {
            let (from, to, cost) = (e.from(), e.to(), e.weight());
            if from <= to {
                ret = ret + cost;
            }
        }
    }

    ret + dp[(1 << m) - 1].unwrap()
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
