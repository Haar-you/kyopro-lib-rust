//! 最小全域木 (Prim)

use crate::graph::*;
use std::{cmp::Reverse, collections::BinaryHeap};

pub fn prim<T: Ord, E: EdgeTrait<Weight = T>>(g: &Graph<Undirected, E>) -> Vec<&E> {
    let n = g.len();
    let mut visit = vec![false; n];
    let mut ret = vec![];
    let mut heap: BinaryHeap<Reverse<(T, usize, usize)>> = BinaryHeap::new();

    visit[0] = true;
    for (index, e) in g.edges[0].iter().enumerate() {
        heap.push(Reverse((e.weight(), e.from(), index)));
    }

    while let Some(Reverse((_, from, index))) = heap.pop() {
        let e = &g.edges[from][index];
        if visit[e.from()] == visit[e.to()] {
            continue;
        }

        let i = if visit[e.from()] { e.to() } else { e.from() };
        for (index, e) in g.edges[i].iter().enumerate() {
            heap.push(Reverse((e.weight(), e.from(), index)));
        }

        visit[i] = true;

        ret.push(e);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut g = Graph::<Undirected, _>::new(6);
        g.extend(
            vec![
                (0, 1, 1),
                (0, 2, 3),
                (1, 2, 1),
                (1, 3, 7),
                (2, 4, 1),
                (1, 4, 3),
                (3, 4, 1),
                (3, 5, 1),
                (4, 5, 6),
            ]
            .into_iter()
            .map(|(u, v, w)| Edge::new(u, v, w, ())),
        );

        let ans = prim(&g).iter().map(|e| e.weight).sum::<i32>();

        assert_eq!(ans, 5);
    }
}
