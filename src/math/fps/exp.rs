//! 形式的冪級数の指数関数
use crate::math::fps::log::*;
use crate::math::polynomial::{Polynomial, PolynomialOperator};

/// 形式的冪級数の指数関数
pub trait FpsExp {
    /// 多項式の型
    type Poly;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\exp (f(x))$の先頭$n$項を求める。
    fn fps_exp(&self, f: Self::Poly) -> Result<Self::Poly, &'static str>;
}

impl<const P: u32, const PR: u32> FpsExp for PolynomialOperator<P, PR> {
    type Poly = Polynomial<P>;

    fn fps_exp(&self, f: Self::Poly) -> Result<Self::Poly, &'static str> {
        let f: Vec<_> = f.into();
        let n = f.len();

        let mut t = 1;
        let mut b = Polynomial::constant(1.into());

        loop {
            let mut temp: Vec<_> = self.fps_log(b.clone())?.into();

            temp.resize(2 * t, 0.into());
            temp.iter_mut().for_each(|x| *x = -*x);
            temp[0] += 1.into();

            temp.iter_mut()
                .zip(f.iter())
                .for_each(|(temp, f)| *temp += *f);

            b = self.mul(b, temp.into());
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
