//! 形式的冪級数の平方根
use crate::num::ff::*;
use crate::{
    math::{
        fps::inv::FpsInv,
        mod_ops::sqrt::mod_sqrt,
        polynomial::{Polynomial, PolynomialOperator},
    },
    num::const_modint::ConstModInt,
};

/// 形式的冪級数の平方根
pub trait FpsSqrt {
    /// 多項式の型
    type Poly;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\sqrt{f(x)}$の先頭$n$項を求める。
    fn fps_sqrt(&self, f: Self::Poly) -> Option<Self::Poly>;
}

impl<const P: u32, const PR: u32> FpsSqrt for PolynomialOperator<'_, P, PR> {
    type Poly = Polynomial<P>;

    fn fps_sqrt(&self, f: Self::Poly) -> Option<Self::Poly> {
        let f: Vec<_> = f.into();

        let n = f.len();
        let k = f
            .iter()
            .enumerate()
            .find(|(_, &x)| x.value() != 0)
            .map_or(n, |(k, _)| k);

        if k == n {
            return Some(f.into());
        }
        if k % 2 == 1 {
            return None;
        }

        let x = mod_sqrt(f[k].value() as u64, P as u64)?;
        let m = n - k;

        let half = ConstModInt::new(2).inv();
        let mut t = 1;
        let mut ret = vec![ConstModInt::new(x as u32)];

        loop {
            let mut f = f[k..k + t.min(m)].to_vec();
            f.resize(t, 0.into());

            ret.resize(t, 0.into());
            let h = self.fps_inv(ret.clone().into());
            let h = self.mul(f.into(), h);
            let h: Vec<_> = h.into();

            for (x, y) in ret.iter_mut().zip(h) {
                *x = (*x + y) * half;
            }

            if t >= m {
                break;
            }

            t <<= 1;
        }

        ret.resize(n, 0.into());
        let mut ret: Polynomial<P> = ret.into();
        ret.shift_higher(k / 2);
        Some(ret)
    }
}
