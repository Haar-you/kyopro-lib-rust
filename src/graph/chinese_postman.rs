use crate::graph::template::*;
use std::{cmp::min, ops::Add};

impl<T: Default + Copy + Ord + Add<Output = T>> Graph<T> {
    /// Time complexity O(V ^ 2 * 2 ^ V)
    pub fn chinese_postman_problem(&self) -> T {
        let n = self.len();

        let mut dist = vec![vec![None; n]; n];

        for i in 0..n {
            dist[i][i] = Some(T::default());
        }

        for i in 0..n {
            for &Edge { from, to, cost } in &self.edges[i] {
                dist[from][to] = match dist[from][to] {
                    None => Some(cost),
                    Some(x) => Some(min(x, cost)),
                };
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if let Some(x) = dist[i][k] {
                        if let Some(y) = dist[k][j] {
                            dist[i][j] = match dist[i][j] {
                                None => Some(x + y),
                                Some(z) => Some(min(z, x + y)),
                            };
                        }
                    }
                }
            }
        }

        let mut odd = vec![];
        for i in 0..n {
            if self.edges[i].len() % 2 == 1 {
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
                        dp[i] = match dp[i] {
                            None => Some(d),
                            Some(x) => Some(min(x, d)),
                        };
                    }
                }
            }
        }

        let mut ret = T::default();

        for i in 0..n {
            for &Edge { from, to, cost } in &self.edges[i] {
                if from <= to {
                    ret = ret + cost;
                }
            }
        }

        ret + dp[(1 << m) - 1].unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/problems/DPL_2_B
        let g = Graph::from_tuples_undirected(4, &[(0, 1, 1), (0, 2, 2), (1, 3, 3), (2, 3, 4)]);
        assert_eq!(g.chinese_postman_problem(), 10);

        let g = Graph::from_tuples_undirected(
            4,
            &[(0, 1, 1), (0, 2, 2), (1, 3, 3), (2, 3, 4), (1, 2, 5)],
        );
        assert_eq!(g.chinese_postman_problem(), 18);

        let g = Graph::from_tuples_undirected(2, &[(0, 1, 1), (0, 1, 2), (0, 1, 3)]);
        assert_eq!(g.chinese_postman_problem(), 7);
    }
}
