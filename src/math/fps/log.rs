//! 形式的冪級数の対数
use crate::math::fps::inv::*;
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::num::ff::*;

/// 形式的冪級数の対数
pub trait FpsLog {
    /// 戻り値の型
    type Output;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\log (f(x))$の先頭$n$項を求める。
    fn fps_log(self) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsLog for Polynomial<P> {
    type Output = Self;

    fn fps_log(self) -> Result<Self::Output, &'static str> {
        assert_eq!(self.coeff_of(0).value(), 1);
        let n = self.len();
        let mut a = self.clone();
        a.differentiate();
        let b = self.fps_inv()?;
        let mut ret = a * b;
        ret.integrate();
        ret.as_mut().resize(n, 0.into());
        Ok(ret)
    }
}
