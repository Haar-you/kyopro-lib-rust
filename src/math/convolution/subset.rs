//! 添字部分集合畳み込み
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/subset_convolution>
use std::ops::{Add, Mul, Sub};

use crate::math::convolution::{mobius::*, zeta::*};

/// $h_k = \sum_{i \lor j = k, i \land j = 0} f_i g_j$を満たす$h$を求める。
///
/// # Requirements
/// `f.len()` = `g.len()`は2の累乗
#[allow(clippy::manual_memcpy)]
pub fn subset_convolution<T>(f: Vec<T>, g: Vec<T>) -> Vec<T>
where
    T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert_eq!(f.len(), g.len());

    let n = f.len();
    assert!(n.is_power_of_two());

    let k = n.trailing_zeros() as usize;

    let mut f2 = vec![vec![T::default(); n]; k + 1];
    let mut g2 = vec![vec![T::default(); n]; k + 1];

    for j in 0..n {
        let i = j.count_ones() as usize;
        f2[i][j] = f[j];
        g2[i][j] = g[j];
    }

    for (f2i, g2i) in f2.iter_mut().zip(g2.iter_mut()) {
        fast_zeta_subset(f2i);
        fast_zeta_subset(g2i);
    }

    let mut h = vec![vec![T::default(); n]; k + 1];

    for (i, hi) in h.iter_mut().enumerate() {
        for (f2s, g2i_s) in f2.iter().take(i + 1).zip(g2.iter().take(i + 1).rev()) {
            for ((hij, f2sj), g2i_sj) in hi.iter_mut().zip(f2s).zip(g2i_s) {
                *hij = *hij + *f2sj * *g2i_sj;
            }
        }
    }

    for hi in h.iter_mut() {
        fast_mobius_subset(hi);
    }

    (0..n)
        .map(|j| {
            let i = j.count_ones() as usize;
            h[i][j]
        })
        .collect()
}
