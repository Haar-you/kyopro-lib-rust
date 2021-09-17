use crate::graph::template::*;
use std::{cmp::min, ops::Add};

pub fn tsp<T>(g: &Graph<T>, src: usize) -> Option<T>
where
    T: Default + Copy + Ord + Add<Output = T>,
{
    let n = g.len();
    let mut dp = vec![vec![None; 1 << n]; n];

    for &Edge { to, cost, .. } in &g.edges[src] {
        dp[to][1 << to] = Some(dp[to][1 << to].map_or(cost, |x| min(x, cost)));
    }

    for s in 1..1 << n {
        for i in 0..n {
            if (s & (1 << i)) == 0 {
                continue;
            }

            for &Edge { to, cost, .. } in &g.edges[i] {
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

        let g = Graph::<u64>::from_tuples(
            4,
            &[
                (0, 1, 2),
                (1, 2, 3),
                (1, 3, 9),
                (2, 0, 1),
                (2, 3, 6),
                (3, 2, 4),
            ],
        );
        assert_eq!(tsp(&g, 0), Some(16));

        let g = Graph::<u64>::from_tuples(3, &[(0, 1, 1), (1, 2, 1), (0, 2, 1)]);
        assert_eq!(tsp(&g, 0), None);
    }
}
