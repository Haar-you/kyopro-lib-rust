//! 最小Steiner木
//!
//! # References
//! - <https://www.slideshare.net/slideshow/ss-12131479/12131479>

use std::collections::VecDeque;

use crate::algo::enum_bit::subset_asc::subset_asc;
pub use crate::graph::core::*;

#[derive(Clone, Copy)]
enum Restore {
    Merge(usize),
    Edge(usize, usize),
    None,
}

/// 最小Steiner木を求める。
///
/// - `graph` : $n$頂点の無向グラフ
/// - `terminals` : `graph`の頂点集合の大きさ$k$の部分集合
///
/// **Time complexity** $O(3^k n + 2^k n^2)$
pub fn min_steiner_tree<I>(
    graph: &UndirectedGraph<u64, I>,
    terminals: Vec<usize>,
) -> Vec<&Edge<u64, I>> {
    let n = graph.len();
    let k = terminals.len();

    let mut g: Vec<Vec<Option<&Edge<_, _>>>> = vec![vec![None; n]; n];
    for (u, gu) in g.iter_mut().enumerate() {
        for e in graph.node_of(u) {
            let v = e.to();
            if u != v && gu[v].is_none_or(|e2| e2.weight() > e.weight()) {
                gu[v] = Some(e);
            }
        }
    }

    let mut dp: Vec<Vec<Option<u64>>> = vec![vec![None; n]; 1 << k];
    let mut from: Vec<Vec<Restore>> = vec![vec![Restore::None; n]; 1 << k];

    for u in 0..n {
        dp[0][u] = Some(0);
    }
    for (i, &u) in terminals.iter().enumerate() {
        for e in g[u].iter().flatten() {
            dp[1 << i][e.to()] = Some(e.weight());
            from[1 << i][e.to()] = Restore::Edge(e.to(), u);
        }
        dp[1 << i][u] = Some(0);
    }

    for s in 0_usize..1 << k {
        for u in 0..n {
            for t in subset_asc(s) {
                if t == 0 || t == s {
                    continue;
                }
                if let (Some(y), Some(z)) = (dp[t][u], dp[s ^ t][u])
                    && dp[s][u].is_none_or(|x| x > y + z)
                {
                    dp[s][u] = Some(y + z);
                    from[s][u] = Restore::Merge(t);
                }
            }
        }

        let mut q = VecDeque::new();
        for u in 0..n {
            if let Some(x) = dp[s][u] {
                q.push_back((u, x));
            }
        }

        while let Some((u, y)) = q.pop_front() {
            if dp[s][u].is_some_and(|x| x < y) {
                continue;
            }

            for e in g[u].iter().flatten() {
                if dp[s][e.to()].is_none_or(|x| x > y + e.weight()) {
                    dp[s][e.to()] = Some(y + e.weight());
                    from[s][e.to()] = Restore::Edge(e.to(), u);
                    q.push_back((e.to(), y + e.weight()));
                }
            }
        }
    }

    let s = (1 << k) - 1;
    let (u, _) = dp[s]
        .iter()
        .enumerate()
        .filter_map(|(i, c)| c.map(|c| (i, c)))
        .min_by_key(|(_, c)| *c)
        .unwrap();

    let mut ret = vec![];
    let mut stack = vec![(s, u)];

    while let Some((s, u)) = stack.pop() {
        match from[s][u] {
            Restore::Merge(t) => {
                stack.push((t, u));
                stack.push((s ^ t, u));
            }
            Restore::Edge(_u, v) => {
                assert_eq!(u, _u);
                stack.push((s, v));
                ret.push(g[u][v].unwrap());
            }
            Restore::None => {}
        }
    }

    ret
}
