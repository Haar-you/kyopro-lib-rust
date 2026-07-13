//! 有向Euler閉路の数え上げ
//!
//! # References
//! - <https://en.wikipedia.org/wiki/BEST_theorem>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/counting_eulerian_circuits>

use crate::{
    graph::{matrix_tree::count_directed_spanning_tree, *},
    math::{factorial::FactorialTable, prime_mod::*},
    num::const_modint::*,
};

/// 有向Euler閉路を数え上げる。
pub fn count_eulerian_circuits<P: PrimeMod, W, I>(g: &Graph<Directed, W, I>) -> ConstModInt<P> {
    let n = g.len();

    let mut indeg = vec![0; n];
    let mut outdeg = vec![0; n];

    let mut m = 0;

    for e in g.nodes_iter().flatten() {
        indeg[e.to()] += 1;
        outdeg[e.from()] += 1;
        m += 1;
    }

    for i in 0..n {
        if indeg[i] != outdeg[i] {
            return 0.into();
        }
    }

    let modulo = ConstModIntBuilder::<P>::new();
    let ft = FactorialTable::new(m, modulo);

    let mut ret = count_directed_spanning_tree(g, 0, true);

    for di in outdeg.into_iter() {
        if di == 0 {
            continue;
        }

        ret *= ft.facto(di - 1);
    }

    ret
}
