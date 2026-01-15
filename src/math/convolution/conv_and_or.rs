//! 添字AND・OR畳み込み
use crate::math::convolution::{mobius::*, zeta::*};
use std::ops::{Add, Mul, Sub};

/// $h_{i \land j} = \sum f_i g_j$を満たす$h$を求める。
///
/// # Requirements
/// `f.len()` = `g.len()`
pub fn convolution_and<T>(mut f: Vec<T>, mut g: Vec<T>) -> Vec<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert!(f.len() == g.len());
    fast_zeta_superset(&mut f);
    fast_zeta_superset(&mut g);
    for (x, y) in f.iter_mut().zip(g.into_iter()) {
        *x = *x * y;
    }
    fast_mobius_superset(&mut f);
    f
}

/// $h_{i \lor j} = \sum f_i g_j$を満たす$h$を求める。
///
/// # Requirements
/// `f.len()` = `g.len()`
pub fn convolution_or<T>(mut f: Vec<T>, mut g: Vec<T>) -> Vec<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert!(f.len() == g.len());
    fast_zeta_subset(&mut f);
    fast_zeta_subset(&mut g);
    for (x, y) in f.iter_mut().zip(g.into_iter()) {
        *x = *x * y;
    }
    fast_mobius_subset(&mut f);
    f
}
