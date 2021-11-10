use crate::graph::{lowlink::*, *};

/// 橋の列挙
///
/// Time complexity O(V + E)
pub fn bridges<T>(g: &Graph<T>) -> Vec<(usize, usize)> {
    let Lowlink {
        ord, low, par, ch, ..
    } = Lowlink::new(&g);

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
        let g =
            Graph::<u32>::from_tuples_undirected(4, &[(0, 1, 1), (0, 2, 1), (1, 2, 1), (2, 3, 1)]);
        let mut ans = bridges(&g);
        ans.sort();
        assert_eq!(ans, [(2, 3)]);

        let g =
            Graph::<u32>::from_tuples_undirected(5, &[(0, 1, 1), (1, 2, 1), (2, 3, 1), (3, 4, 1)]);
        let mut ans = bridges(&g);
        ans.sort();
        assert_eq!(ans, [(0, 1), (1, 2), (2, 3), (3, 4)]);
    }
}
