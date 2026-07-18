//! 無向グラフの閉路検出
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/cycle_detection_undirected>
use std::collections::HashMap;

use crate::graph::*;

/// 無向グラフから閉路を検出する。
///
/// **Time complexity** $O(V + E)$
pub fn detect_undirected_cycle<W, I>(g: &UndirectedGraph<W, I>) -> Option<Vec<&Edge<W, I>>> {
    let n = g.len();

    for i in 0..n {
        let mut map: HashMap<usize, &Edge<W, I>> = HashMap::new();

        for e in g.node_of(i) {
            if e.from() == e.to() {
                return Some(vec![e]); // 自己ループ
            }

            if let Some(e1) = map.get(&e.to()) {
                let e2 = g
                    .node_of(e.to())
                    .neighbors()
                    .find(|e2| e2.index() == e.index())
                    .unwrap();

                return Some(vec![e1, e2]); // 多重辺
            } else {
                map.insert(e.to(), e);
            }
        }
    }

    let mut status = vec![Status::Unchecked; n];
    for i in 0..n {
        let mut ret = vec![];
        if let Some(l) = rec(g, i, None, 0, &mut ret, &mut status) {
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
    g: &'a UndirectedGraph<W, I>,
    cur: usize,
    prev: Option<usize>,
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
                if prev.is_some_and(|p| p == e.to()) {
                    continue;
                }

                ret.push(e);

                if let Status::Searching(l) = check[e.to()] {
                    return Some(l);
                }

                if let Some(res) = rec(g, e.to(), Some(cur), pos + 1, ret, check) {
                    return Some(res);
                }

                ret.pop();
            }

            check[cur] = Status::Searched;
            None
        }
    }
}
