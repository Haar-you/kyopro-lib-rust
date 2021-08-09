use crate::graph::template::*;

impl<T> Graph<T>
where
    T: std::ops::Add<Output = T> + Copy + Clone + Ord + From<i32>
{
    /// Time complexity O(n ^ 3)
    pub fn warshall_floyd(&self) -> Option<Vec<Vec<Option<T>>>> {
        let zero = T::from(0);
        let n = self.len();
        let mut dist = vec![vec![None; n]; n];

        for i in 0 .. n {
            dist[i][i] = Some(zero);
            for &Edge { from, to, cost } in &self.edges[i] {
                dist[from][to] = Some(cost);
            }
        }

        for k in 0 .. n {
            for i in 0 .. n {
                for j in 0 .. n {
                    if dist[i][k].is_some() && dist[k][j].is_some() {
                        let s = dist[i][k].unwrap() + dist[k][j].unwrap();
                        dist[i][j] = match dist[i][j] {
                            Some(x) => Some(std::cmp::min(x, s)),
                            _ => Some(s)
                        };
                    }
                }
            }
        }

        for i in 0 .. n {
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
        let mut g = Graph::<i32>::new(4);
        g.from_tuples(&[(0, 1, 1), (0, 2, 5), (1, 2, 2), (1, 3, 4), (2, 3, 1), (3, 2, 7)]);
        let ans = g.warshall_floyd();

        assert_eq!(ans,
                   Some(vec![
                       vec![Some(0), Some(1), Some(3), Some(4)],
                       vec![None, Some(0), Some(2), Some(3)],
                       vec![None, None, Some(0), Some(1)],
                       vec![None, None, Some(7), Some(0)]
                   ]));

        let mut g = Graph::<i32>::new(4);
        g.from_tuples(&[(0, 1, 1), (0, 2, -5), (1, 2, 2), (1, 3, 4), (2, 3, 1), (3, 2, 7)]);
        let ans = g.warshall_floyd();

        assert_eq!(ans,
                   Some(vec![
                       vec![Some(0), Some(1), Some(-5), Some(-4)],
                       vec![None, Some(0), Some(2), Some(3)],
                       vec![None, None, Some(0), Some(1)],
                       vec![None, None, Some(7), Some(0)]
                   ]));

        let mut g = Graph::<i32>::new(4);
        g.from_tuples(&[(0, 1, 1), (0, 2, 5), (1, 2, 2), (1, 3, 4), (2, 3, 1), (3, 2, -7)]);
        let ans = g.warshall_floyd();

        assert_eq!(ans, None);
    }
}
