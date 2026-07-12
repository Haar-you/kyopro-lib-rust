//! 行列木定理を用いた数え上げ
//! # Problems
//! - <https://judge.yosupo.jp/problem/counting_spanning_tree_undirected>
//! - <https://judge.yosupo.jp/problem/counting_spanning_tree_directed>

use crate::graph::*;
use crate::linalg::mod_p::determinant::*;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 無向グラフにおいて、無向全域木の個数を数える。
pub fn count_undirected_spanning_tree<P: PrimeMod, W, I>(
    g: &Graph<Undirected, W, I>,
) -> ConstModInt<P> {
    let modulo = ConstModIntBuilder::<P>::new();

    let n = g.len();
    let mut lap = vec![vec![modulo.from_u64(0); n - 1]; n - 1];

    for e in g.nodes_iter().flatten() {
        let from = e.from();
        let to = e.to();

        if from < n - 1 {
            lap[from][from] += modulo.from_u64(1);
            if to < n - 1 {
                lap[from][to] -= modulo.from_u64(1);
            }
        }
    }

    determinant(lap, &modulo)
}

/// 有向グラフにおいて、頂点`root`を終点根とするような、有向全域木の個数を数える。
///
/// `ignore_isolates`が`true`のとき、孤立点を無視する。
pub fn count_directed_spanning_tree<P: PrimeMod, W, I>(
    g: &Graph<Directed, W, I>,
    root: usize,
    ignore_isolates: bool,
) -> ConstModInt<P> {
    let modulo = ConstModIntBuilder::<P>::new();

    let n = g.len();

    let mut k = 0;
    let mut map = vec![0; n];

    if ignore_isolates {
        let mut indeg = vec![0; n];
        let mut outdeg = vec![0; n];

        for e in g.nodes_iter().flatten() {
            indeg[e.to()] += 1;
            outdeg[e.from()] += 1;
        }

        for i in 0..n {
            if indeg[i] != 0 || outdeg[i] != 0 {
                map[i] = k;
                k += 1;
            }
        }
    } else {
        map = (0..n).collect();
        k = n;
    };

    let mut lap = vec![vec![modulo.from_u64(0); k - 1]; k - 1];

    let root = map[root];

    for e in g.nodes_iter().flatten() {
        let mut from = map[e.from()];
        let mut to = map[e.to()];

        if from == root {
            continue;
        }
        if from > root {
            from -= 1;
        }
        lap[from][from] += modulo.from_u64(1);

        if to == root {
            continue;
        }
        if to > root {
            to -= 1;
        }
        lap[from][to] -= modulo.from_u64(1);
    }

    determinant(lap, &modulo)
}
