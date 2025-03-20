use crate::math::convolution::{mobius::*, zeta::*};
use std::ops::{Add, Mul, Sub};

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
