//! $\mathtt{a_{i \oplus j}} = \sum \mathtt{f_{i}} * \mathtt{g_{j}}$を満たす`a`を求める。
use std::ops::{Add, Sub};

use crate::num::ff::*;

/// $\mathtt{a_{i \oplus j}} = \sum \mathtt{f_{i}} * \mathtt{g_{j}}$を満たす`a`を求める。
pub fn convolution_xor<Modulo: FF>(
    mut f: Vec<Modulo::Element>,
    mut g: Vec<Modulo::Element>,
    modulo: Modulo,
) -> Vec<Modulo::Element>
where
    Modulo::Element: Copy + FFElem,
{
    assert_eq!(f.len(), g.len());

    fwt(&mut f);
    fwt(&mut g);

    for (x, y) in f.iter_mut().zip(g.into_iter()) {
        *x *= y;
    }

    fwt(&mut f);

    let t = modulo.frac(1, f.len() as i64);

    for x in f.iter_mut() {
        *x *= t;
    }

    f
}

fn fwt<T>(f: &mut [T])
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    let n = f.len();
    assert!(n.is_power_of_two());
    for i in (0..).map(|i| 1 << i).take_while(|&x| x < n) {
        for j in 0..n {
            if j & i == 0 {
                let x = f[j];
                let y = f[j | i];
                f[j] = x + y;
                f[j | i] = x - y;
            }
        }
    }
}
