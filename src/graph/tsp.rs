use crate::graph::template::*;
use std::{cmp::min, ops::Add};

impl<T: Default + Copy + Ord + Add<Output = T>> Graph<T> {
    pub fn tsp(&self, src: usize) -> Option<T> {
        let n = self.len();
        let mut dp = vec![vec![None; 1 << n]; n];

        for &Edge { to, cost, .. } in &self.edges[src] {
            dp[to][1 << to] = match dp[to][1 << to] {
                None => Some(cost),
                Some(x) => Some(min(x, cost)),
            };
        }

        for s in 1..1 << n {
            for i in 0..n {
                if (s & (1 << i)) == 0 {
                    continue;
                }

                for &Edge { to, cost, .. } in &self.edges[i] {
                    if s & (1 << to) != 0 {
                        continue;
                    }

                    if let Some(x) = dp[i][s] {
                        dp[to][s | (1 << to)] = match dp[to][s | (1 << to)] {
                            None => Some(x + cost),
                            Some(y) => Some(min(y, x + cost)),
                        };
                    }
                }
            }
        }

        dp[src][(1 << n) - 1]
    }
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
        assert_eq!(g.tsp(0), Some(16));

        let g = Graph::<u64>::from_tuples(3, &[(0, 1, 1), (1, 2, 1), (0, 2, 1)]);
        assert_eq!(g.tsp(0), None);
    }
}
