//! 形式的冪級数の対数
use crate::math::fps::inv::*;
use crate::math::polynomial::{Polynomial, PolynomialOperator};
use crate::num::ff::*;

/// 形式的冪級数の対数
pub trait FpsLog {
    /// 多項式の型
    type Poly;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\log (f(x))$の先頭$n$項を求める。
    fn fps_log(&self, f: Self::Poly) -> Result<Self::Poly, &'static str>;
}

impl<const P: u32, const PR: u32> FpsLog for PolynomialOperator<'_, P, PR> {
    type Poly = Polynomial<P>;

    fn fps_log(&self, f: Self::Poly) -> Result<Self::Poly, &'static str> {
        assert_eq!(f.coeff_of(0).value(), 1);
        let n = f.len();
        let mut a = f.clone();
        a.differentiate();
        let b = self.fps_inv(f)?;
        let mut ret = self.mul(a, b);
        ret.integrate();
        ret.as_mut().resize(n, 0.into());
        Ok(ret)
    }
}
