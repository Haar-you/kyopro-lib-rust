//! 形式的冪級数の累乗
use crate::math::fps::exp::*;
use crate::math::fps::log::*;
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::ConstModInt;
use crate::num::ff::*;

/// 形式的冪級数の累乗
pub trait FpsPow {
    /// 戻り値の型
    type Output;

    /// 形式的冪級数の累乗を求める。
    fn fps_pow(self, m: u64) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsPow for Polynomial<P> {
    type Output = Self;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$(f(x))^m$の先頭$n$項を求める。
    ///
    /// **Time complexity** $O(N \log N)$
    fn fps_pow(self, m: u64) -> Result<Self::Output, &'static str> {
        if m == 0 {
            let mut f: Vec<_> = self.into();
            f.fill(ConstModInt::new(0));
            f[0] = ConstModInt::new(1);
            return Ok(f.into());
        }
        if m == 1 {
            return Ok(self);
        }

        let n = self.len();
        let mut k = 0;
        while k < n {
            if self.coeff_of(k).value() != 0 {
                break;
            }
            k += 1;
        }

        if k >= n {
            return Ok(self);
        }

        if k.checked_mul(m as usize).is_none_or(|x| x >= n) {
            return Ok(vec![ConstModInt::new(0); n].into());
        }

        let a = self.coeff_of(k);

        let mut ret = self;
        ret.shift_lower(k);
        ret.scale(a.inv());
        let mut ret = ret.fps_log()?;
        ret.scale(m.into());
        let mut ret = ret.fps_exp()?;
        ret.scale(a.pow(m));
        ret.shift_higher(m as usize * k);
        Ok(ret)
    }
}
