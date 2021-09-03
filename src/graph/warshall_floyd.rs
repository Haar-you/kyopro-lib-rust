use crate::graph::template::*;
use std::{cmp::min, ops::Add};

impl<T> Graph<T>
where
    T: Add<Output = T> + Copy + Clone + Ord + Default,
{
    /// Time complexity O(n ^ 3)
    pub fn warshall_floyd(&self) -> Option<Vec<Vec<Option<T>>>> {
        let zero = T::default();
        let n = self.len();
        let mut dist = vec![vec![None; n]; n];

        for i in 0..n {
            dist[i][i] = Some(zero);
            for &Edge { from, to, cost } in &self.edges[i] {
                dist[from][to] = Some(cost);
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][k].is_some() && dist[k][j].is_some() {
                        let s = dist[i][k].unwrap() + dist[k][j].unwrap();
                        dist[i][j] = Some(dist[i][j].map_or(s, |x| min(x, s)));
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_C
        let g = Graph::<i32>::from_tuples(
            4,
            &[
                (0, 1, 1),
                (0, 2, 5),
                (1, 2, 2),
                (1, 3, 4),
                (2, 3, 1),
                (3, 2, 7),
            ],
        );
        let ans = g.warshall_floyd();

        assert_eq!(
            ans,
            Some(vec![
                vec![Some(0), Some(1), Some(3), Some(4)],
                vec![None, Some(0), Some(2), Some(3)],
                vec![None, None, Some(0), Some(1)],
                vec![None, None, Some(7), Some(0)]
            ])
        );

        let g = Graph::<i32>::from_tuples(
            4,
            &[
                (0, 1, 1),
                (0, 2, -5),
                (1, 2, 2),
                (1, 3, 4),
                (2, 3, 1),
                (3, 2, 7),
            ],
        );
        let ans = g.warshall_floyd();

        assert_eq!(
            ans,
            Some(vec![
                vec![Some(0), Some(1), Some(-5), Some(-4)],
                vec![None, Some(0), Some(2), Some(3)],
                vec![None, None, Some(0), Some(1)],
                vec![None, None, Some(7), Some(0)]
            ])
        );

        let g = Graph::<i32>::from_tuples(
            4,
            &[
                (0, 1, 1),
                (0, 2, 5),
                (1, 2, 2),
                (1, 3, 4),
                (2, 3, 1),
                (3, 2, -7),
            ],
        );
        let ans = g.warshall_floyd();

        assert_eq!(ans, None);
    }
}
