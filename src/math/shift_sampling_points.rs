//! 多項式の標本点シフト
//!
//! # References
//! - <https://suisen-cp.github.io/cp-library-cpp/library/polynomial/shift_of_sampling_points.hpp.html>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/shift_of_sampling_points_of_polynomial>

use crate::{
    math::factorial::FactorialTable,
    math::ntt::NTT,
    math::prime_mod::PrimeMod,
    num::const_modint::{ConstModInt, ConstModIntBuilder},
};

/// $N$次未満の多項式$f(x)$について、$f(0), f(1), \dots, f(N-1)$から$f(c), f(c + 1), \dots, f(c + M - 1)$を求める。
pub fn shift_sampling_points<P: PrimeMod>(
    f: Vec<impl Into<ConstModInt<P>>>,
    c: u32,
    m: usize,
) -> Vec<ConstModInt<P>> {
    let f = f.into_iter().map(Into::into).collect::<Vec<_>>();

    let n = f.len();
    let ntt = NTT::<P>::new();
    let ft = FactorialTable::new(n.max(m), ConstModIntBuilder::<P>::new());

    let a = {
        let f = f
            .into_iter()
            .enumerate()
            .map(|(i, x)| x * ft.inv_facto(i))
            .collect();
        let g = (0..n)
            .map(|i| {
                if i % 2 == 0 {
                    ft.inv_facto(i)
                } else {
                    -ft.inv_facto(i)
                }
            })
            .collect();
        ntt.convolve(f, g)[..n].to_vec()
    };

    let b = {
        let f = a
            .into_iter()
            .enumerate()
            .rev()
            .map(|(i, x)| x * ft.facto(i))
            .collect();
        let mut p = ConstModInt::new(1);
        let g = (0..n)
            .map(|i| {
                let ret = p * ft.inv_facto(i);
                p *= (c as i64 - i as i64).into();
                ret
            })
            .collect();
        ntt.convolve(f, g)[..n].to_vec()
    };

    let mut ret = {
        let f = b
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, x)| x * ft.inv_facto(i))
            .collect();
        let g = (0..m).map(|i| ft.inv_facto(i)).collect();
        ntt.convolve(f, g)[..m].to_vec()
    };

    ret.iter_mut()
        .enumerate()
        .for_each(|(i, x)| *x *= ft.facto(i));
    ret
}
