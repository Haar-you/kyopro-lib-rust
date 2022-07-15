use crate::graph::*;
use std::collections::VecDeque;

pub fn bfs<T, E: EdgeTrait<Weight = T>>(
    g: &Graph<E>,
    src: impl IntoIterator<Item = usize>,
) -> Vec<Option<usize>> {
    let mut ret = vec![None; g.len()];
    let mut check = vec![false; g.len()];
    let mut q = VecDeque::new();

    for s in src {
        ret[s] = Some(0);
        q.push_back(s);
    }

    while let Some(cur) = q.pop_front() {
        if check[cur] {
            continue;
        }
        check[cur] = true;

        for e in &g.edges[cur] {
            if ret[e.to()].is_none() || ret[e.to()].unwrap() > ret[e.from()].unwrap() + 1 {
                ret[e.to()] = Some(ret[e.from()].unwrap() + 1);
                q.push_back(e.to());
            }
        }
    }

    ret
}
