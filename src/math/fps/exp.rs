//! 形式的冪級数の指数関数
use crate::math::fps::log::*;
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::num::ff::ZZElem;

/// 形式的冪級数の指数関数
pub trait FpsExp {
    /// 戻り値の型
    type Output;

    /// 形式的冪級数の指数関数を求める。
    fn fps_exp(self) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsExp for Polynomial<P> {
    type Output = Self;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\exp (f(x))$の先頭$n$項を求める。
    ///
    /// 定数項が$0$でないとき、`Err`を返す。
    ///
    /// **Time complexity** $O(N \log N)$
    fn fps_exp(self) -> Result<Self::Output, &'static str> {
        if self.coeff_of(0).value() != 0 {
            return Err("定数項は`0`でなければならない。");
        }

        let f: Vec<_> = self.into();
        let n = f.len();

        let mut t = 1;
        let mut b = Self::constant(1.into());

        loop {
            let mut temp: Vec<_> = b.clone().fps_log()?.into();

            temp.resize(2 * t, 0.into());
            temp.iter_mut().for_each(|x| *x = -*x);
            temp[0] += 1.into();

            temp.iter_mut()
                .zip(f.iter())
                .for_each(|(temp, f)| *temp += *f);

            b *= temp.into();
            b.as_mut().resize(2 * t, 0.into());

            if t >= n {
                break;
            }

            t <<= 1;
        }

        b.as_mut().truncate(n);
        Ok(b)
    }
}
