use crate::graph::*;
use std::{cmp::Reverse, collections::BinaryHeap};

pub fn prim<T: Copy + Ord>(g: &Graph<T>) -> Vec<(usize, usize, T)> {
    let n = g.len();
    let mut visit = vec![false; n];
    let mut ret = vec![];
    let mut heap: BinaryHeap<Reverse<(T, usize, usize)>> = BinaryHeap::new();

    visit[0] = true;
    for &Edge { from, to, cost } in &g.edges[0] {
        heap.push(Reverse((cost, from, to)));
    }

    while let Some(Reverse((d, from, to))) = heap.pop() {
        if visit[from] == visit[to] {
            continue;
        }

        let i = if visit[from] { to } else { from };
        for &Edge {
            from: u,
            to: v,
            cost: c,
        } in &g.edges[i]
        {
            heap.push(Reverse((c, u, v)));
        }

        visit[i] = true;

        ret.push((from, to, d));
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let g = Graph::<i32>::from_tuples_undirected(
            6,
            &[
                (0, 1, 1),
                (0, 2, 3),
                (1, 2, 1),
                (1, 3, 7),
                (2, 4, 1),
                (1, 4, 3),
                (3, 4, 1),
                (3, 5, 1),
                (4, 5, 6),
            ],
        );

        let ans = prim(&g).iter().map(|(_, _, x)| x).sum::<i32>();

        assert_eq!(ans, 5);
    }
}
