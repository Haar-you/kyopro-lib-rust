//! 橋の列挙

pub use crate::graph::lowlink::*;

/// 橋の列挙
///
/// **Time complexity** $O(V + E)$
pub fn bridges(ll: &Lowlink) -> Vec<(usize, usize)> {
    let Lowlink { ord, low, ch, .. } = ll;

    ch.iter()
        .enumerate()
        .flat_map(|(i, es)| {
            es.iter()
                .filter_map(|&j| (ord[i] < low[j]).then_some((i, j)))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_3_B
        let mut g = Graph::<Undirected, _>::new(4);
        g.extend(
            vec![(0, 1), (0, 2), (1, 2), (2, 3)]
                .into_iter()
                .map(|(u, v)| Edge::new(u, v, (), ())),
        );
        let mut ans = bridges(&Lowlink::new(&g));
        ans.sort();
        assert_eq!(ans, [(2, 3)]);

        let mut g = Graph::<Undirected, _>::new(5);
        g.extend(
            vec![(0, 1), (1, 2), (2, 3), (3, 4)]
                .into_iter()
                .map(|(u, v)| Edge::new(u, v, (), ())),
        );
        let mut ans = bridges(&Lowlink::new(&g));
        ans.sort();
        assert_eq!(ans, [(0, 1), (1, 2), (2, 3), (3, 4)]);
    }
}
