use crate::math::convolution::{mobius_super::*, zeta_super::*};
use std::ops::{Add, Mul, Sub};

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
