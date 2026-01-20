//! 添字GCD・LCM畳み込み
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/gcd_convolution>
use crate::math::convolution::div_mul_transform::*;
use std::ops::{Add, Mul, Sub};

/// $h_{\gcd (i, j)} = \sum f_i g_j$を満たす$h$を求める。
///
/// # Requirements
/// `f.len()` = `g.len()`
pub fn convolution_gcd<T>(mut f: Vec<T>, mut g: Vec<T>) -> Vec<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert_eq!(f.len(), g.len());

    mul_zeta(&mut f);
    mul_zeta(&mut g);

    for (x, y) in f.iter_mut().zip(g.into_iter()) {
        *x = *x * y;
    }

    mul_mobius(&mut f);
    f
}

/// $h_{\mathrm{lcm} (i, j)} = \sum f_i g_j$を満たす$h$を求める。
///
/// # Requirements
/// `f.len()` = `g.len()`
pub fn convolution_lcm<T>(mut f: Vec<T>, mut g: Vec<T>) -> Vec<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert_eq!(f.len(), g.len());

    div_zeta(&mut f);
    div_zeta(&mut g);

    for (x, y) in f.iter_mut().zip(g.into_iter()) {
        *x = *x * y;
    }

    div_mobius(&mut f);
    f
}
