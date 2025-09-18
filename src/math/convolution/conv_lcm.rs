//! LCM畳み込み
use crate::math::convolution::div_mul_transform::*;
use std::ops::{Add, Mul, Sub};

/// $\mathtt{a_{\mathtt{lcm} (i, j)}} = \sum \mathtt{f_{i}} * \mathtt{g_{j}}$を満たす`a`を求める。
///
/// `a`の長さは`|f| = |g|`と等しい。
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
