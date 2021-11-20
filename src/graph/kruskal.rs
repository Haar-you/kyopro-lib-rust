use crate::{ds::unionfind::UnionFind, graph::*};

pub fn kruskal<T: Ord, E: Clone + EdgeTrait<Weight = T>>(g: &Graph<E>) -> Vec<&E> {
    let n = g.len();
    let mut edges = vec![];
    for es in &g.edges {
        for e in es {
            edges.push(e);
        }
    }

    edges.sort_by(|a, b| a.weight().cmp(&b.weight()));

    let mut uf = UnionFind::new(n);
    let mut ret = vec![];

    for e in edges {
        let (u, v) = (e.from(), e.to());
        if !uf.is_same(u, v) {
            uf.merge(u, v);
            ret.push(e);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::*;

    #[test]
    fn test() {
        let mut g = Graph::new(6);
        g.add_undirected(
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
            .map(|(u, v, w)| Edge::new(u, v, w, ()))
            .collect::<Vec<_>>(),
        );

        let ans = kruskal(&g).iter().map(|e| e.weight).sum::<i32>();

        assert_eq!(ans, 5);
    }
}
