use crate::graph::template::*;
use std::{cmp::min, mem::swap};

/// 橋の列挙
///
/// Time complexity O(V + E)
pub fn bridges<T>(g: &Graph<T>) -> Vec<(usize, usize)> {
    let n = g.len();
    let mut visit = vec![None; n];
    let mut low = vec![0; n];
    let mut ret = vec![];
    let mut v = 0;

    for i in 0..n {
        if visit[i].is_none() {
            bridges_(g, i, None, &mut visit, &mut low, &mut ret, &mut v);
        }
    }

    ret
}

fn bridges_<T>(
    g: &Graph<T>,
    cur: usize,
    par: Option<usize>,
    visit: &mut Vec<Option<usize>>,
    low: &mut Vec<usize>,
    ret: &mut Vec<(usize, usize)>,
    v: &mut usize,
) -> usize {
    if let Some(x) = visit[cur] {
        return x;
    }

    visit[cur] = Some(*v);

    let mut temp = *v;
    *v += 1;

    for &Edge {
        mut from, mut to, ..
    } in &g.edges[cur]
    {
        if Some(to) == par {
            continue;
        }
        let t = bridges_(g, to, Some(cur), visit, low, ret, v);
        temp = min(temp, t);
        if low[to] > visit[cur].unwrap() {
            if from > to {
                swap(&mut from, &mut to);
            }
            ret.push((from, to));
        }
    }
    low[cur] = temp;
    low[cur]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_3_B
        let g =
            Graph::<u32>::from_tuples_undirected(4, &[(0, 1, 1), (0, 2, 1), (1, 2, 1), (2, 3, 1)]);
        let mut ans = bridges(&g);
        ans.sort();
        assert_eq!(ans, [(2, 3)]);

        let g =
            Graph::<u32>::from_tuples_undirected(5, &[(0, 1, 1), (1, 2, 1), (2, 3, 1), (3, 4, 1)]);
        let mut ans = bridges(&g);
        ans.sort();
        assert_eq!(ans, [(0, 1), (1, 2), (2, 3), (3, 4)]);
    }
}
