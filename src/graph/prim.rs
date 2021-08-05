use std::collections::BinaryHeap;
use std::cmp::Reverse;

use crate::graph::template::*;

impl<T> Graph<T>
where
    T: Ord + Copy
{
    pub fn prim(&self) -> Vec<(usize, usize, T)> {
        let n = self.len();
        let mut visit = vec![false; n];
        let mut ret = vec![];
        let mut heap: BinaryHeap<Reverse<(T, usize, usize)>> = BinaryHeap::new();

        visit[0] = true;
        for &Edge { from, to, cost } in &self.edges[0] {
            heap.push(Reverse((cost, from, to)));
        }

        while let Some(Reverse((d, from, to))) = heap.pop() {
            if visit[from] == visit[to] {
                continue;
            }

            let i = if visit[from] { to } else { from };
            for &Edge { from: u, to: v, cost: c } in &self.edges[i] {
                heap.push(Reverse((c, u, v)));
            }

            visit[i] = true;

            ret.push((from, to, d));
        }

        ret
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = Graph::<i32>::new(6)
            .add_undirected(0, 1, 1)
            .add_undirected(0, 2, 3)
            .add_undirected(1, 2, 1)
            .add_undirected(1, 3, 7)
            .add_undirected(2, 4, 1)
            .add_undirected(1, 4, 3)
            .add_undirected(3, 4, 1)
            .add_undirected(3, 5, 1)
            .add_undirected(4, 5, 6)
            .prim()
            .iter()
            .map(|(_, _, x)| x)
            .sum::<i32>();

        assert_eq!(ans, 5);
    }
}