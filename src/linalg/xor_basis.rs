//! $\mathbb{F}_2^{64}$上の要素{$a_1, \dots, a_n$}の張る部分空間の基底を求める。
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc283/tasks/abc283_g>

/// $\mathbb{F}_2^{64}$上の要素{$a_1, \dots, a_n$}の張る部分空間の基底を求める。
pub fn xor_basis(a: Vec<u64>) -> Vec<u64> {
    let mut basis = vec![];
    for mut e in a {
        for &b in &basis {
            if e ^ b < e {
                e ^= b;
            }
        }
        for b in basis.iter_mut() {
            if e ^ *b < *b {
                *b ^= e;
            }
        }
        if e != 0 {
            basis.push(e);
        }
    }
    basis
}
