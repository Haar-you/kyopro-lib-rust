//! $\mathbb{F}_2$ベクトル空間$u$と$v$の共通部分
use crate::ds::bitset::*;
use crate::linalg::mod_2::lineq::*;

/// $\mathbb{F}_2$ベクトル空間$u$と$v$の共通部分の基底を求める。
pub fn intersection_vector_space(u: Vec<Bitset>, v: Vec<Bitset>) -> Vec<Bitset> {
    if u.is_empty() || v.is_empty() {
        return vec![];
    }

    let n = u.len();
    let m = v.len();

    let dim = u[0].len();

    let mut w = vec![Bitset::new(n + m); dim];
    for (i, x) in u.iter().chain(v.iter()).enumerate() {
        for (j, wj) in w.iter_mut().enumerate() {
            if x.test(j) {
                wj.flip(i);
            }
        }
    }

    let (_, s) = lineq(w, vec![false; dim]).unwrap();

    let mut basis = vec![];
    for x in s {
        let mut b = Bitset::new(dim);
        for (i, ui) in u.iter().enumerate() {
            if x.test(i) {
                b.same_size_xor_assign(ui);
            }
        }
        basis.push(b);
    }

    basis
}
