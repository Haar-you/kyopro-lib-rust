//! 形式的冪級数の累乗
use crate::math::fps::{exp::*, log::*};
use crate::math::polynomial::{Polynomial, PolynomialOperator};
use crate::num::{const_modint::ConstModInt, ff::*};

/// 形式的冪級数の累乗
pub trait FpsPow {
    /// 多項式の型
    type Poly;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$(f(x))^m$の先頭$n$項を求める。
    fn fps_pow(&self, f: Self::Poly, m: u64) -> Self::Poly;
}

impl<const P: u32, const PR: u32> FpsPow for PolynomialOperator<'_, P, PR> {
    type Poly = Polynomial<P>;

    fn fps_pow(&self, f: Self::Poly, m: u64) -> Self::Poly {
        if m == 0 {
            let mut f: Vec<_> = f.into();
            f.fill(ConstModInt::new(0));
            f[0] = ConstModInt::new(1);
            return f.into();
        }
        if m == 1 {
            return f;
        }

        let n = f.len();
        let mut k = 0;
        while k < n {
            if f.coeff_of(k).value() != 0 {
                break;
            }
            k += 1;
        }

        if k >= n {
            return f;
        }

        if k.checked_mul(m as usize).is_none_or(|x| x >= n) {
            return vec![ConstModInt::new(0); n].into();
        }

        let a = f.coeff_of(k);

        let ret = self.shift_lower(f, k);
        let ret = self.scale(ret, a.inv());
        let ret = self.scale(self.fps_log(ret), m.into());
        let ret = self.fps_exp(ret);
        let ret = self.scale(ret, a.pow(m));
        self.shift_higher(ret, m as usize * k)
    }
}
