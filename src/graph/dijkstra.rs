//! 非負重み付き最短経路 (Dijkstra)

use crate::graph::*;
use crate::traits::{num::Unsigned, one_zero::Zero};
use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

/// Time complexity O((E + V) log V)
pub fn dijkstra<D: Direction, T, E: EdgeTrait<Weight = T>>(
    g: &Graph<D, E>,
    src: &[usize],
) -> Vec<Option<T>>
where
    T: Add<Output = T> + Copy + Ord + Zero<Output = T> + Unsigned,
{
    let zero = T::zero();
    let n = g.len();
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

        for e in &g.edges[u] {
            let (to, cost) = (e.to(), e.weight());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grl_1_a() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_A

        // sample 1
        let mut graph = Graph::<Directed, Edge<u32, ()>>::new(4);
        graph.extend(
            vec![(0, 1, 1), (0, 2, 4), (1, 2, 2), (2, 3, 1), (1, 3, 5)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        let ans = dijkstra(&graph, &[0]);

        assert_eq!(ans, [Some(0), Some(1), Some(3), Some(4)]);

        // sample 2
        let mut graph = Graph::<Directed, Edge<u32, ()>>::new(4);
        graph.extend(
            vec![
                (0, 1, 1),
                (0, 2, 4),
                (2, 0, 1),
                (1, 2, 2),
                (3, 1, 1),
                (3, 2, 5),
            ]
            .into_iter()
            .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );
        let ans = dijkstra(&graph, &[1]);

        assert_eq!(ans, [Some(3), Some(0), Some(2), None]);
    }
}
