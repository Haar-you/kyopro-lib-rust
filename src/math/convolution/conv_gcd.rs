//! $\mathtt{a_{\gcd (i, j)}} = \sum \mathtt{f_{i}} * \mathtt{g_{j}}$を満たす`a`を求める。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/gcd_convolution>
use std::ops::{Add, Mul, Sub};

/// $\mathtt{a_{\gcd (i, j)}} = \sum \mathtt{f_{i}} * \mathtt{g_{j}}$を満たす`a`を求める。
pub fn convolution_gcd<T>(mut f: Vec<T>, mut g: Vec<T>) -> Vec<T>
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

fn div_mobius<T>(f: &mut [T])
where
    T: Copy + Sub<Output = T>,
{
    let n = f.len();
    let mut check = vec![true; n];
    for i in 2..n {
        if check[i] {
            for j in (1..).take_while(|j| j * i < n) {
                check[j * i] = false;
                f[j] = f[j] - f[j * i];
            }
        }
    }
}

fn div_zeta<T>(f: &mut [T])
where
    T: Copy + Add<Output = T>,
{
    let n = f.len();
    let mut check = vec![true; n];
    for i in 2..n {
        if check[i] {
            for j in (1..=(n - 1) / i).rev() {
                check[j * i] = false;
                f[j] = f[j] + f[j * i];
            }
        }
    }
}
