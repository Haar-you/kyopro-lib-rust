//! mod p(素数)でのk乗根
//!
//! # Problems
//! - <https://yukicoder.me/problems/no/981>
//! - <https://judge.yosupo.jp/problem/kth_root_mod>
use crate::math::{
    gcd_lcm::GcdLcm,
    mod_ops::{inv::inv_mod, log::log_mod, pow::pow_mod},
    primitive_root_u64::primitive_root_u64,
};

/// 素数$p$について、$a \equiv x^k \pmod p$となる$x$を求める。
/// 存在しなければ、`None`を返す。
///
/// $p$は$0 < p < 2^{32}$の範囲の素数であること。
pub fn kth_root_mod(a: u64, k: u64, p: u64) -> Option<u64> {
    assert!(0 < p && p <= 0xFFFFFFFF);

    if a == 0 {
        if k != 0 {
            return Some(0);
        } else {
            return None;
        }
    }

    let g = primitive_root_u64(p);
    let y = log_mod(g, a, p)?;

    let yg = y.gcd(p - 1);
    let kg = k.gcd(p - 1);

    if !yg.is_multiple_of(kg) {
        return None;
    }

    let k_ = k / kg;
    let y_ = y / kg;
    let p_ = (p - 1) / kg;

    let z = y_ * inv_mod(k_, p_).unwrap() % p_;
    let x = pow_mod(g, z, p);

    Some(x)
}
