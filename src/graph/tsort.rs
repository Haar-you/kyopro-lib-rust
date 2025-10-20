//! トポロジカルソート

use crate::graph::*;
use std::collections::VecDeque;

/// トポロジカルソート
///
/// **Time complexity** $O(V + E)$
///
/// gがDAGのとき、トポロジカルソートした結果をSomeに包んで返す。
/// そうでなければ、Noneを返す。
pub fn tsort<E: EdgeTrait>(g: &Graph<Directed, E>) -> Option<Vec<usize>> {
    let n = g.len();
    let mut indeg = vec![0; n];

    for e in g.nodes_iter().flat_map(|v| &v.edges) {
        indeg[e.to()] += 1;
    }

    let mut q: VecDeque<_> = indeg
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| (x == 0).then_some(i))
        .collect();

    let mut ret = vec![];

    while let Some(cur) = q.pop_front() {
        ret.push(cur);
        for e in g.nodes[cur].edges.iter() {
            let to = e.to();
            indeg[to] -= 1;
            if indeg[to] == 0 {
                q.push_back(to);
            }
        }
    }

    (ret.len() == n).then_some(ret)
}
