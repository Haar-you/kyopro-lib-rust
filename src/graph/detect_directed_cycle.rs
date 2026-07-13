//! 有向グラフの閉路検出
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/cycle_detection>

use crate::graph::*;

/// 有向グラフから閉路を検出する。
///
/// **Time complexity** $O(V + E)$
pub fn detect_directed_cycle<W, I>(g: &DirectedGraph<W, I>) -> Option<Vec<&Edge<W, I>>> {
    let n = g.len();

    let mut status = vec![Status::Unchecked; n];
    for i in 0..n {
        let mut ret = vec![];
        if let Some(l) = rec(g, i, 0, &mut ret, &mut status) {
            return Some(ret[l..].to_vec());
        }
    }

    None
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Unchecked,
    Searched,
    Searching(usize),
}

fn rec<'a, W, I>(
    g: &'a DirectedGraph<W, I>,
    cur: usize,
    pos: usize,
    ret: &mut Vec<&'a Edge<W, I>>,
    check: &mut [Status],
) -> Option<usize> {
    match check[cur] {
        Status::Searched => None,
        Status::Searching(_) => unreachable!(),
        Status::Unchecked => {
            check[cur] = Status::Searching(pos);

            for e in g.node_of(cur) {
                ret.push(e);

                if let Status::Searching(l) = check[e.to()] {
                    return Some(l);
                }

                if let Some(res) = rec(g, e.to(), pos + 1, ret, check) {
                    return Some(res);
                }

                ret.pop();
            }

            check[cur] = Status::Searched;
            None
        }
    }
}
