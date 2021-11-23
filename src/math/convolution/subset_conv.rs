use crate::math::convolution::{mobius_sub::*, zeta_sub::*};
use std::ops::{Add, Mul, Sub};

#[allow(clippy::needless_range_loop, clippy::manual_memcpy, clippy::many_single_char_names)]
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

    for i in 0..=k {
        fast_zeta_subset(&mut f2[i]);
        fast_zeta_subset(&mut g2[i]);
    }

    let mut h = vec![vec![T::default(); n]; k + 1];

    for i in 0..=k {
        for s in 0..=i {
            for j in 0..n {
                h[i][j] = h[i][j] + f2[s][j] * g2[i - s][j];
            }
        }
    }

    for i in 0..=k {
        fast_mobius_subset(&mut h[i]);
    }

    (0..n)
        .map(|j| {
            let i = j.count_ones() as usize;
            h[i][j]
        })
        .collect()
}
