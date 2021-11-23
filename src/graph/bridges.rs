use crate::graph::{lowlink::*, *};

/// 橋の列挙
///
/// Time complexity O(V + E)
pub fn bridges<E: EdgeTrait>(g: &Graph<E>) -> Vec<(usize, usize)> {
    let Lowlink { ord, low, ch, .. } = Lowlink::new(&g);

    ch.iter()
        .enumerate()
        .map(|(i, es)| {
            es.iter()
                .filter_map(|&j| if ord[i] < low[j] { Some((i, j)) } else { None })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_3_B
        let mut g = Graph::new(4);
        g.add_undirected(
            vec![(0, 1, 1), (0, 2, 1), (1, 2, 1), (2, 3, 1)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ()))
                .collect::<Vec<_>>(),
        );
        let mut ans = bridges(&g);
        ans.sort();
        assert_eq!(ans, [(2, 3)]);

        let mut g = Graph::new(5);
        g.add_undirected(
            vec![(0, 1, 1), (1, 2, 1), (2, 3, 1), (3, 4, 1)]
                .into_iter()
                .map(|(u, v, w)| Edge::new(u, v, w, ()))
                .collect::<Vec<_>>(),
        );
        let mut ans = bridges(&g);
        ans.sort();
        assert_eq!(ans, [(0, 1), (1, 2), (2, 3), (3, 4)]);
    }
}
