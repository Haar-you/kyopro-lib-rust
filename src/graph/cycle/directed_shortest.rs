//! 有向グラフで単一始点の最短サイクルを求める。
//! # Problems
//! - [ABC 142 F - Pure](https://atcoder.jp/contests/abc142/tasks/abc142_f)
//! - <https://atcoder.jp/contests/abc376/tasks/abc376_d>

use crate::graph::{bfs::*, *};

/// 有向グラフで単一始点の最短サイクルを求める。
///
/// **Time complexity** $O(V + E)$
pub fn directed_shortest_cycle<E: EdgeTrait>(
    g: &Graph<Directed, E>,
    src: usize,
) -> Option<Vec<&E>> {
    let res = bfs(g, Some(src));
    let p = res
        .iter()
        .flatten()
        .flat_map(|(d, e)| {
            if let Some(e) = e {
                g.nodes[e.to()]
                    .edges
                    .iter()
                    .find(|e| e.to() == src)
                    .map(|e_src| (d, e, e_src))
            } else {
                None
            }
        })
        .min_by_key(|(d, _, _)| *d);

    if let Some((_, e, e_src)) = p {
        let mut ret = vec![];
        let mut cur = e.to();

        ret.push(e_src);

        while cur != src {
            let e = res[cur].unwrap().1.unwrap();
            ret.push(e);
            cur = e.from();
        }

        ret.reverse();

        Some(ret)
    } else {
        None
    }
}
