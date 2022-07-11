//! 橋の列挙

pub use crate::graph::lowlink::*;

/// 橋の列挙
///
/// Time complexity O(V + E)
pub fn bridges(ll: &Lowlink) -> Vec<(usize, usize)> {
    let Lowlink { ord, low, ch, .. } = ll;

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
    use crate::graph::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_3_B
        let mut g = Graph::new(4);
        g.add_undirected(
            vec![(0, 1), (0, 2), (1, 2), (2, 3)]
                .into_iter()
                .map(|(u, v)| Edge::new(u, v, (), ())),
        );
        let mut ans = bridges(&Lowlink::new(&g));
        ans.sort();
        assert_eq!(ans, [(2, 3)]);

        let mut g = Graph::new(5);
        g.add_undirected(
            vec![(0, 1), (1, 2), (2, 3), (3, 4)]
                .into_iter()
                .map(|(u, v)| Edge::new(u, v, (), ())),
        );
        let mut ans = bridges(&Lowlink::new(&g));
        ans.sort();
        assert_eq!(ans, [(0, 1), (1, 2), (2, 3), (3, 4)]);
    }
}
