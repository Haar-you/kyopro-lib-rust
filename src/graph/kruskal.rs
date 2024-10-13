//! 最小全域木 (Kruskal)

use crate::{ds::unionfind::UnionFind, graph::*};

/// Kruskal法
///
/// グラフが連結ならばSomeに包んで最小全域木の辺集合を返す。
/// 非連結ならばNoneを返す。
///
/// **Time complexity O(E log E)**
pub fn kruskal<T: Ord + Copy, E: EdgeTrait<Weight = T>>(
    g: &Graph<Undirected, E>,
) -> Option<Vec<&E>> {
    let n = g.len();
    let mut edges = g
        .edges
        .iter()
        .flatten()
        .map(|e| (e, e.weight()))
        .collect::<Vec<_>>();
    edges.sort_unstable_by_key(|&(_, c)| c);

    let mut uf = UnionFind::new(n);
    let mut ret = vec![];

    for (e, _) in edges {
        let (u, v) = (e.from(), e.to());
        if !uf.is_same(u, v) {
            uf.merge(u, v);
            ret.push(e);
        }
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

        let ans = kruskal(&g).unwrap().iter().map(|e| e.weight).sum::<i32>();

        assert_eq!(ans, 5);
    }
}
