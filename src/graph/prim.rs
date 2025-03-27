//! 最小全域木 (Prim)

use crate::graph::*;
use std::{cmp::Reverse, collections::BinaryHeap};

/// Prim法
///
/// グラフが連結ならばSomeに包んで最小全域木の辺集合を返す。
/// 非連結ならばNoneを返す。
pub fn prim<E: EdgeTrait>(g: &Graph<Undirected, E>) -> Option<Vec<&E>>
where
    E::Weight: Ord,
{
    let n = g.len();
    let mut visit = vec![false; n];
    let mut ret = vec![];
    let mut heap = BinaryHeap::new();

    visit[0] = true;
    for (index, e) in g.nodes[0].edges.iter().enumerate() {
        heap.push(Reverse((e.weight(), e.from(), index)));
    }

    while let Some(Reverse((_, from, index))) = heap.pop() {
        let e = &g.nodes[from].edges[index];
        if visit[e.from()] == visit[e.to()] {
            continue;
        }

        let i = if visit[e.from()] { e.to() } else { e.from() };
        for (index, e) in g.nodes[i].edges.iter().enumerate() {
            heap.push(Reverse((e.weight(), e.from(), index)));
        }

        visit[i] = true;

        ret.push(e);
    }

    (ret.len() == n - 1).then_some(ret)
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

        let ans = prim(&g).unwrap().iter().map(|e| e.weight).sum::<i32>();

        assert_eq!(ans, 5);
    }
}
