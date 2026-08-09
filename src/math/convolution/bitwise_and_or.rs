//! 添字AND・OR畳み込み
use std::ops::{Add, Mul, Sub};

use crate::math::convolution::mobius::*;
use crate::math::convolution::zeta::*;

/// $h_{i \land j} = \sum f_i g_j$を満たす$h$を求める。
///
/// # Requirements
/// `f.len()` = `g.len()`
pub fn bitwise_and_convolution<T>(mut f: Vec<T>, mut g: Vec<T>) -> Vec<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert_eq!(f.len(), g.len());
    fast_zeta_superset(&mut f);
    fast_zeta_superset(&mut g);
    for (x, y) in f.iter_mut().zip(g) {
        *x = *x * y;
    }
    fast_mobius_superset(&mut f);
    f
}

/// $h_{i \lor j} = \sum f_i g_j$を満たす$h$を求める。
///
/// # Requirements
/// `f.len()` = `g.len()`
pub fn bitwise_or_convolution<T>(mut f: Vec<T>, mut g: Vec<T>) -> Vec<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert_eq!(f.len(), g.len());
    fast_zeta_subset(&mut f);
    fast_zeta_subset(&mut g);
    for (x, y) in f.iter_mut().zip(g) {
        *x = *x * y;
    }
    fast_mobius_subset(&mut f);
    f
}
