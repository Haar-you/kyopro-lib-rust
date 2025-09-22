//! $\\#_p$ Subset sum
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/sharp_p_subset_sum>
use crate::math::fps::exp::*;
use crate::math::polynomial::*;
use crate::num::const_modint::*;

/// $\\#_p$ Subset sum
pub fn number_of_subset_sum<const P: u32, const PR: u32>(
    s: Vec<usize>,
    t: usize,
) -> Vec<ConstModInt<P>> {
    let ff = ConstModIntBuilder;
    let fps = PolynomialOperator::<P, PR>::new();

    let mut c = vec![0; t + 1];
    for x in s {
        c[x] += 1;
    }

    let mut ret = vec![ff.from_u64(0); t + 1];

    for (i, c) in c.into_iter().enumerate().skip(1) {
        if c != 0 {
            for j in (1..).take_while(|&j| i * j <= t) {
                let k = j * i;
                let x = ff.from_i64(if j % 2 == 1 { 1 } else { -1 })
                    * ff.from_u64(i as u64)
                    * ff.from_u64(k as u64).inv();
                ret[k] += x * ff.from_u64(c as u64);
            }
        }
    }

    fps.fps_exp(ret.into()).unwrap().into()
}
