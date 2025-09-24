//! 階乗 $n! \pmod P$ ($0 \le n \lt P$)
//!
//! # References
//! - <https://suisen-kyopro.hatenablog.com/entry/2023/11/22/201600>

use crate::math::prime_mod::PrimeMod;
use crate::{math::polynomial::shift_sampling_points::*, num::const_modint::*};

/// 階乗を計算する。
pub struct Factorial<P: PrimeMod> {
    r: u32,
    prod: Vec<ConstModInt<P>>,
}

impl<P: PrimeMod> Factorial<P> {
    /// 前計算を行う。
    pub fn new() -> Self {
        let k = 9;
        let r = 1 << k;

        let mut f = vec![ConstModInt::new(1)];

        for i in 0..k {
            let n = f.len();
            let mut g = shift_sampling_points::<P>(f.clone(), n as u32, n * 3);
            f.append(&mut g);

            f = f
                .chunks_exact(2)
                .enumerate()
                .map(|(j, f)| f[0] * f[1] * ((2 * j + 1) << i).into())
                .collect();
        }

        let block_num = (P::PRIME_NUM / r) as usize;
        if f.len() < block_num {
            let mut g = shift_sampling_points::<P>(f.clone(), f.len() as u32, block_num - f.len());
            f.append(&mut g);
        }

        let mut prod = vec![1.into(); f.len() + 1];
        for (i, fi) in f.into_iter().enumerate() {
            prod[i + 1] = prod[i] * fi * r.into() * (i + 1).into();
        }

        Self { r, prod }
    }

    /// $n! \pmod P$を計算する。
    pub fn factorial(&self, n: u32) -> ConstModInt<P> {
        if n >= P::PRIME_NUM {
            return 0.into();
        }

        let k = n / self.r;
        let p = k * self.r;
        let mut ret = self.prod[k as usize];

        for i in p + 1..=n {
            ret *= i.into()
        }

        ret
    }
}
