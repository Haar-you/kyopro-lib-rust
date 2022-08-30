//! 有向グラフ上の最小有向全域木を求める

use crate::graph::*;
use std::collections::VecDeque;
use std::ops::Sub;

type Edge_<'a, T, E> = (usize, usize, T, &'a E);

/// 有向グラフ上の最小有向全域木を求める
///
/// # Complexity
/// Time Complexity $O(VE)$
///
/// # Problems
/// - [AOJ GRL_2_B](https://onlinejudge.u-aizu.ac.jp/problems/GRL_2_B)
pub fn chu_liu_edmonds<T, E>(g: &Graph<E>, root: usize) -> Vec<&E>
where
    T: Ord + Copy + Sub<Output = T>,
    E: EdgeTrait<Weight = T>,
{
    let n = g.len();
    let mut rg = vec![vec![]; n];

    for es in &g.edges {
        for e in es {
            rg[e.to()].push((e.to(), e.from(), e.weight(), e));
        }
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
            let (from, to, weight, e) = es.iter().min_by(|x, y| x.2.cmp(&y.2)).unwrap().clone();

            in_edges[from] = Some((from, to, weight, e));
            out_count[to] += 1;
        }
    }

    let mut q = VecDeque::new();
    for (i, &x) in out_count.iter().enumerate() {
        if x == 0 {
            q.push_back(i);
        }
    }

    while let Some(i) = q.pop_front() {
        for &(_, to, ..) in &in_edges[i] {
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
            .filter(|&(_, c)| c != 0)
            .map(|(i, _)| i);

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
                let c = g[i].iter().min_by(|x, y| x.2.cmp(&y.2)).unwrap().clone();

                for e in g[i].iter_mut() {
                    (*e).2 = (*e).2 - c.2;
                }
            }

            for &i in &cycle {
                in_cycle[i] = true;
            }

            groups.push(cycle);
        }

        for i in 0..n {
            if !in_cycle[i] {
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

        for es in &g {
            for &(from, to, weight, e) in es {
                if s[from] != s[to] {
                    sg[s[from]].push((s[from], s[to], weight, e));
                }
            }
        }

        let res = rec(sg, root);

        for (i, e) in res.into_iter().enumerate() {
            let p = if let Some(e) = e { e.1 } else { continue };

            let mut c = vec![];

            for &x in &groups[i] {
                for e in &g[x] {
                    let to = e.1;
                    if s[to] == p {
                        c.push(e);
                    }
                }
            }

            let &(from, to, weight, e) = c.iter().min_by(|x, y| x.2.cmp(&y.2)).unwrap().clone();

            in_edges[from] = Some((from, to, weight, e));
        }
    }

    in_edges
}
