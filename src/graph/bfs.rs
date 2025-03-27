//! 幅優先探索

use crate::graph::*;
use crate::misc::is_none_or::*;
use std::collections::VecDeque;
use std::iter::zip;

/// 幅優先探索で辺数が最小の経路を得る。
pub fn bfs<D: Direction, E: EdgeTrait>(
    g: &Graph<D, E>,
    src: impl IntoIterator<Item = usize>,
) -> Vec<Option<(usize, Option<&E>)>> {
    let mut dist = vec![None; g.len()];
    let mut prev = vec![None; g.len()];
    let mut check = vec![false; g.len()];
    let mut q = VecDeque::new();

    for s in src {
        dist[s] = Some(0);
        q.push_back(s);
    }

    while let Some(cur) = q.pop_front() {
        if check[cur] {
            continue;
        }
        check[cur] = true;

        let d_cur = dist[cur].unwrap();
        for e in g.nodes[cur].edges.iter() {
            let to = e.to();
            if dist[to].is_none_or(|d| d > d_cur + 1) {
                dist[to] = Some(d_cur + 1);
                prev[to] = Some(e);
                q.push_back(to);
            }
        }
    }

    zip(dist, prev).map(|(a, b)| a.map(|a| (a, b))).collect()
}
