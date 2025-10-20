//! 形式的冪級数の指数関数
use crate::math::fps::log::*;
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;

/// 形式的冪級数の指数関数
pub trait FpsExp {
    /// 戻り値の型
    type Output;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\exp (f(x))$の先頭$n$項を求める。
    fn fps_exp(self) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsExp for Polynomial<P> {
    type Output = Self;

    fn fps_exp(self) -> Result<Self::Output, &'static str> {
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
