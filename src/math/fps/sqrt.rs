//! 形式的冪級数の平方根
use crate::num::ff::*;
use crate::{
    math::prime_mod::PrimeMod,
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
    fn fps_sqrt(&self, f: Self::Poly) -> Result<Self::Poly, &'static str>;
}

impl<P: PrimeMod> FpsSqrt for PolynomialOperator<P> {
    type Poly = Polynomial<P>;

    fn fps_sqrt(&self, f: Self::Poly) -> Result<Self::Poly, &'static str> {
        let f: Vec<_> = f.into();

        let n = f.len();
        let k = f
            .iter()
            .enumerate()
            .find(|(_, &x)| x.value() != 0)
            .map_or(n, |(k, _)| k);

        if k == n {
            return Ok(f.into());
        }
        if k % 2 == 1 {
            return Err("最小次数が偶数ではない。");
        }

        let x = mod_sqrt(f[k].value() as u64, P::PRIME_NUM as u64)
            .ok_or("最小次数項の係数に平方根が存在しない。")?;
        let m = n - k;

        let half = ConstModInt::new(2).inv();
        let mut t = 1;
        let mut ret = vec![ConstModInt::new(x as u32)];

        loop {
            let mut f = f[k..k + t.min(m)].to_vec();
            f.resize(t, 0.into());

            ret.resize(t, 0.into());
            let h = self.fps_inv(ret.clone().into())?;
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
        Ok(ret)
    }
}
