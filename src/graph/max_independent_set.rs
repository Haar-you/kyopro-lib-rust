//! 最大独立集合
//!
//! # Verifications
//! - [Maximum Independent Set](https://judge.yosupo.jp/problem/maximum_independent_set) [#142761](https://judge.yosupo.jp/submission/142761) (n <= 40)
use crate::graph::*;
use std::collections::HashSet;

/// 最大独立集合を求める
///
/// nは64以下に制限している。
/// 最大独立集合の補集合は最小頂点被覆集合になる。
pub fn max_independent_set<E: EdgeTrait>(g: &Graph<Undirected, E>) -> Vec<usize> {
    let n = g.len();
    assert!(n <= 64);

    let mut set = HashSet::new();
    rec(g, 0, 0, &mut set);

    let (a, _) = set
        .into_iter()
        .filter(|(indep, cover)| (indep | cover).count_ones() as usize == n)
        .max_by_key(|&(indep, _)| indep.count_ones())
        .unwrap();

    (0..n).filter(|i| a & (1 << i) != 0).collect()
}

fn rec<E: EdgeTrait>(
    g: &Graph<Undirected, E>,
    indep: u64,
    cover: u64,
    set: &mut HashSet<(u64, u64)>,
) {
    if set.contains(&(indep, cover)) {
        return;
    }

    set.insert((indep, cover));

    let removed = indep | cover;

    let next = (0..g.len())
        .filter(|i| removed & (1 << i) == 0)
        .max_by_key(|&i| {
            g.nodes[i]
                .edges
                .iter()
                .filter(|e| removed & (1 << e.to()) == 0)
                .count()
        });

    let next = if let Some(next) = next {
        next
    } else {
        return;
    };

    let mut deg = 0;
    let mut neighbour: u64 = 0;

    for e in g.nodes[next].edges.iter() {
        if removed & (1 << e.to()) == 0 {
            deg += 1;
            neighbour |= 1 << e.to();
        }
    }

    rec(g, indep | (1 << next), cover | neighbour, set);
    if deg > 1 {
        rec(g, indep, cover | (1 << next), set);
    }
}
