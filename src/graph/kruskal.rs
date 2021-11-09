use crate::{ds::unionfind::UnionFind, graph::*};

pub fn kruskal<T: Copy + Ord>(g: &Graph<T>) -> Vec<(usize, usize, T)> {
    let n = g.len();
    let mut edges = vec![];
    for i in 0..n {
        for &Edge { from, to, cost } in &g.edges[i] {
            edges.push((from, to, cost));
        }
    }

    edges.sort_by(|a, b| a.2.cmp(&b.2));

    let mut uf = UnionFind::new(n);
    let mut ret = vec![];

    for (u, v, c) in edges {
        if !uf.is_same(u, v) {
            uf.merge(u, v);
            ret.push((u, v, c));
        }
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

        let ans = kruskal(&g).iter().map(|(_, _, x)| x).sum::<i32>();

        assert_eq!(ans, 5);
    }
}
