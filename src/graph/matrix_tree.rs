//! 行列木定理を用いた数え上げ
//! # Problems
//! - <https://judge.yosupo.jp/problem/counting_spanning_tree_undirected>
//! - <https://judge.yosupo.jp/problem/counting_spanning_tree_directed>

use crate::graph::*;
use crate::linalg::mod_p::determinant::*;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 無向グラフにおいて、無向全域木の個数を数える。
pub fn count_undirected_spanning_tree<P: PrimeMod>(
    g: &Graph<Undirected, impl EdgeTrait>,
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
pub fn count_directed_spanning_tree<P: PrimeMod>(
    g: &Graph<Directed, impl EdgeTrait>,
    root: usize,
) -> ConstModInt<P> {
    let modulo = ConstModIntBuilder::<P>::new();

    let n = g.len();
    let mut lap = vec![vec![modulo.from_u64(0); n - 1]; n - 1];

    for e in g.nodes_iter().flatten() {
        let mut from = e.from();
        let mut to = e.to();

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
