use crate::graph::*;
use std::collections::VecDeque;

pub struct PseudoTree {
    pub group: Vec<usize>,
    pub in_loop: Vec<bool>,
}

pub fn pseudo_tree<T, E: EdgeTrait<Weight = T>>(g: &Graph<E>) -> PseudoTree {
    let n = g.len();
    let mut indeg = vec![0; n];
    let mut queue = VecDeque::new();
    let mut visit = vec![false; n];
    let mut in_loop = vec![true; n];

    for es in &g.edges {
        for e in es {
            indeg[e.to()] += 1;
        }
    }

    for (i, &deg) in indeg.iter().enumerate() {
        if deg == 1 {
            queue.push_back(i);
        }
    }

    while let Some(cur) = queue.pop_front() {
        in_loop[cur] = false;

        if visit[cur] {
            continue;
        }
        visit[cur] = true;

        for e in &g.edges[cur] {
            if !visit[e.to()] {
                indeg[e.to()] -= 1;

                if indeg[e.to()] == 1 {
                    queue.push_back(e.to());
                }
            }
        }
    }

    let mut group = vec![0; n];

    for i in 0..n {
        if in_loop[i] {
            group[i] = i;
            for e in &g.edges[i] {
                if !in_loop[e.to()] {
                    dfs(&g, e.to(), i, &mut group);
                }
            }
        }
    }

    PseudoTree { group, in_loop }
}

fn dfs<T, E: EdgeTrait<Weight = T>>(g: &Graph<E>, cur: usize, par: usize, group: &mut [usize]) {
    group[cur] = group[par];

    for e in &g.edges[cur] {
        if e.to() != par {
            dfs(g, e.to(), cur, group);
        }
    }
}
