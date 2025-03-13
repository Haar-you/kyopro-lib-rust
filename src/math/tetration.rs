//! $a \uparrow \uparrow b \pmod m$
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/tetration_mod>
use crate::math::{mod_ops::pow::mod_pow, totient::totient};

/// $a \uparrow \uparrow b \pmod m$を求める。
pub fn tetration(a: u64, b: u64, m: u64) -> u64 {
    rec(a, b, m) % m
}

fn rec(a: u64, b: u64, m: u64) -> u64 {
    match b {
        0 => 1 % m,
        1 => a % m,
        2 => mod_pow(a, a, m),
        _ if a == 0 => 1 - b % 2,
        _ if m == 1 => 1,
        _ => {
            let phi = totient(m);
            let mut p = rec(a, b - 1, phi);

            if p == 0 {
                p = phi;
            }

            mod_pow(a, p, m)
        }
    }
}
