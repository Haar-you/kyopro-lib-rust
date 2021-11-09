use crate::graph::*;
use std::collections::VecDeque;

/// Time complexity O(N)
///
/// gがDAGのとき、トポロジカルソートした結果をSomeに包んで返す。
/// そうでなければ、Noneを返す。
pub fn tsort<T>(g: &Graph<T>) -> Option<Vec<usize>> {
    let n = g.len();
    let mut indeg = vec![0; n];

    for i in 0..n {
        for &Edge { to, .. } in &g.edges[i] {
            indeg[to] += 1;
        }
    }

    let mut q = VecDeque::new();

    for (i, &x) in indeg.iter().enumerate() {
        if x == 0 {
            q.push_back(i);
        }
    }

    let mut ret = vec![];

    while let Some(cur) = q.pop_front() {
        ret.push(cur);
        for &Edge { to, .. } in &g.edges[cur] {
            indeg[to] -= 1;
            if indeg[to] == 0 {
                q.push_back(to);
            }
        }
    }

    if ret.len() == n {
        Some(ret)
    } else {
        None
    }
}
