use crate::graph::*;
use std::collections::HashSet;

pub fn enumerate_triangles<E: EdgeTrait>(g: &Graph<Undirected, E>) -> Vec<(usize, usize, usize)> {
    let n = g.len();
    let mut ret = vec![];
    let mut adjacent = vec![HashSet::new(); n];

    for i in 0..n {
        for e in &g.edges[i] {
            if g.edges[e.from()].len() < g.edges[e.to()].len()
                || (g.edges[e.from()].len() == g.edges[e.to()].len() && e.from() < e.to())
            {
                adjacent[e.from()].insert(e.to());
            }
        }
    }

    for i in 0..n {
        for &j in &adjacent[i] {
            for &k in &adjacent[j] {
                if adjacent[i].contains(&k) {
                    ret.push((i, j, k));
                }
            }
        }
    }

    ret
}
