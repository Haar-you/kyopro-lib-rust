use crate::graph::*;
use std::{cmp::min, ops::Add};

pub fn tsp<T, E: EdgeTrait<Weight = T>>(g: &Graph<E>, src: usize) -> Option<T>
where
    T: Default + Copy + Ord + Add<Output = T>,
{
    let n = g.len();
    let mut dp = vec![vec![None; 1 << n]; n];

    for e in &g.edges[src] {
        let (to, cost) = (e.to(), e.weight());
        dp[to][1 << to] = Some(dp[to][1 << to].map_or(cost, |x| min(x, cost)));
    }

    for s in 1..1 << n {
        for i in 0..n {
            if (s & (1 << i)) == 0 {
                continue;
            }

            for e in &g.edges[i] {
                let (to, cost) = (e.to(), e.weight());
                if s & (1 << to) != 0 {
                    continue;
                }

                if let Some(x) = dp[i][s] {
                    dp[to][s | (1 << to)] =
                        Some(dp[to][s | (1 << to)].map_or(x + cost, |y| min(y, x + cost)));
                }
            }
        }
    }

    dp[src][(1 << n) - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/2/DPL_2_A

        let mut g = Graph::new(4);
        g.add_directed(
            vec![
                (0, 1, 2),
                (1, 2, 3),
                (1, 3, 9),
                (2, 0, 1),
                (2, 3, 6),
                (3, 2, 4),
            ]
            .into_iter()
            .map(|(u, v, w)| Edge::new(u, v, w, ()))
            .collect::<Vec<_>>(),
        );
        assert_eq!(tsp(&g, 0), Some(16));

        let mut g = Graph::new(3);
        g.add_directed(
            vec![(0, 1, 1), (1, 2, 1), (0, 2, 1)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ()))
                .collect::<Vec<_>>(),
        );
        assert_eq!(tsp(&g, 0), None);
    }
}
