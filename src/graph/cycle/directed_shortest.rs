//! 有向グラフで単一始点の最短サイクルを求める。
//! # Problems
//! - [ABC 142 F - Pure](https://atcoder.jp/contests/abc142/tasks/abc142_f)

use crate::graph::*;
use std::collections::VecDeque;

/// 有向グラフで単一始点の最短サイクルを求める。
///
/// **Time Complexity O(V + E)**
pub fn directed_shortest_cycle<E: EdgeTrait>(
    g: &Graph<Directed, E>,
    src: usize,
) -> Option<Vec<&E>> {
    let n = g.len();
    let mut q = VecDeque::new();
    let mut pre = vec![None; n];
    let mut visit = vec![false; n];

    q.push_back(src);

    while let Some(cur) = q.pop_front() {
        if visit[cur] {
            continue;
        }
        visit[cur] = true;

        for e in &g.edges[cur] {
            if e.to() == src {
                let mut ret = vec![e];
                let mut cur = cur;
                loop {
                    if let Some(e) = pre[cur] {
                        ret.push(e);
                        cur = e.from();
                    } else {
                        ret.reverse();
                        return Some(ret);
                    }
                }
            }

            if !visit[e.to()] {
                pre[e.to()] = Some(e);
                q.push_back(e.to());
            }
        }
    }

    None
}
