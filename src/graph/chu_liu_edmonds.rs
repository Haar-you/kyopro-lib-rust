//! 有向グラフ上の最小有向全域木を求める
//! # Problems
//! - [AOJ GRL_2_B](https://onlinejudge.u-aizu.ac.jp/problems/GRL_2_B)

use crate::graph::*;
use std::collections::VecDeque;
use std::ops::Sub;

type Edge_<'a, T, E> = (usize, usize, T, &'a E);

/// 有向グラフ上の最小有向全域木を求める
///
/// **Time complexity** $O(VE)$
pub fn chu_liu_edmonds<E: EdgeTrait>(g: &Graph<Directed, E>, root: usize) -> Vec<&E>
where
    E::Weight: Ord + Copy + Sub<Output = E::Weight>,
{
    let n = g.len();
    let mut rg = vec![vec![]; n];

    for e in g.nodes_iter().flat_map(|v| &v.edges) {
        rg[e.to()].push((e.to(), e.from(), e.weight(), e));
    }

    let res = rec(rg, root);
    res.into_iter().filter_map(|e| e.map(|e| e.3)).collect()
}

fn rec<T, E>(mut g: Vec<Vec<Edge_<T, E>>>, root: usize) -> Vec<Option<Edge_<T, E>>>
where
    T: Ord + Copy + Sub<Output = T>,
    E: EdgeTrait<Weight = T>,
{
    let n = g.len();

    let mut in_edges = vec![None; n];
    let mut out_count = vec![0; n];

    for (i, es) in g.iter().enumerate() {
        if i != root {
            let &res @ (from, to, ..) = es.iter().min_by(|x, y| x.2.cmp(&y.2)).unwrap();

            in_edges[from] = Some(res);
            out_count[to] += 1;
        }
    }

    let mut q: VecDeque<_> = out_count
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| (x == 0).then_some(i))
        .collect();

    while let Some(i) = q.pop_front() {
        if let Some((_, to, ..)) = in_edges[i] {
            out_count[to] -= 1;

            if out_count[to] == 0 {
                q.push_back(to);
            }
        }
    }

    let cycles = {
        let temp = out_count
            .into_iter()
            .enumerate()
            .filter_map(|(i, c)| (c != 0).then_some(i));

        let mut ret = vec![];
        let mut check = vec![false; n];

        for i in temp {
            if check[i] {
                continue;
            }

            let mut cur = i;
            let mut cycle = vec![];

            while !check[cur] {
                check[cur] = true;
                cycle.push(cur);
                cur = in_edges[cur].unwrap().1;
            }

            ret.push(cycle);
        }

        ret
    };

    if !cycles.is_empty() {
        let mut s = vec![0; n];
        let mut in_cycle = vec![false; n];
        let mut groups = vec![];

        for cycle in cycles {
            for &i in &cycle {
                let &c = g[i].iter().min_by(|x, y| x.2.cmp(&y.2)).unwrap();

                for e in g[i].iter_mut() {
                    e.2 = e.2 - c.2;
                }

                in_cycle[i] = true;
            }

            groups.push(cycle);
        }

        for (i, x) in in_cycle.into_iter().enumerate() {
            if !x {
                groups.push(vec![i]);
            }
        }

        let size = groups.len();

        for (i, xs) in groups.iter().enumerate() {
            for &x in xs {
                s[x] = i;
            }
        }

        let mut sg = vec![vec![]; size];
        let root = s[root];

        for &(from, to, weight, e) in g.iter().flatten() {
            if s[from] != s[to] {
                sg[s[from]].push((s[from], s[to], weight, e));
            }
        }

        let res = rec(sg, root);

        for (i, e) in res.into_iter().enumerate() {
            let p = if let Some(e) = e { e.1 } else { continue };

            let c = groups[i]
                .iter()
                .flat_map(|&x| g[x].iter())
                .filter(|e| s[e.1] == p);

            let &res @ (from, ..) = c.min_by(|x, y| x.2.cmp(&y.2)).unwrap();

            in_edges[from] = Some(res);
        }
    }

    in_edges
}
