use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

use crate::graph::template::*;

impl<T> Graph<T>
where
    T: Add<Output = T> + Copy + Ord + From<i32>,
{
    /// Time complexity O((E + V) log V)
    pub fn dijkstra(&self, src: &[usize]) -> Vec<Option<T>> {
        let zero = T::from(0);
        let n = self.len();
        let mut ret = vec![None; n];
        let mut heap: BinaryHeap<Reverse<(T, usize)>> = BinaryHeap::new();
        let mut check = vec![false; n];

        for &u in src {
            ret[u] = Some(zero);
            heap.push(Reverse((zero, u)));
        }

        while let Some(Reverse((d, u))) = heap.pop() {
            if check[u] {
                continue;
            }
            check[u] = true;

            for &Edge { to, cost, .. } in &self.edges[u] {
                if let Some(ref d2) = ret[to] {
                    if *d2 > d + cost {
                        let d = d + cost;
                        ret[to] = Some(d);
                        if !check[to] {
                            heap.push(Reverse((d, to)));
                        }
                    }
                } else {
                    let d = d + cost;
                    ret[to] = Some(d);
                    heap.push(Reverse((d, to)));
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grl_1_a() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_A

        // sample 1
        let graph =
            Graph::<i32>::from_tuples(4, &[(0, 1, 1), (0, 2, 4), (1, 2, 2), (2, 3, 1), (1, 3, 5)]);
        let ans = graph.dijkstra(&[0]);

        assert_eq!(ans, [Some(0), Some(1), Some(3), Some(4)]);

        // sample 2
        let graph = Graph::<i32>::from_tuples(
            4,
            &[
                (0, 1, 1),
                (0, 2, 4),
                (2, 0, 1),
                (1, 2, 2),
                (3, 1, 1),
                (3, 2, 5),
            ],
        );
        let ans = graph.dijkstra(&[1]);

        assert_eq!(ans, [Some(3), Some(0), Some(2), None]);
    }
}
