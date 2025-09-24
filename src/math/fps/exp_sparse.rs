//! 疎な形式的冪級数の指数関数
use crate::math::polynomial::sparse::SparsePolynomial;
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 疎な形式的冪級数の指数関数
pub trait FpsExpSparse {
    /// 戻り値の型
    type Output;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\exp (f(x))$の先頭$n$項を求める。
    fn fps_exp_sparse(self, n: usize) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsExpSparse for SparsePolynomial<P> {
    type Output = Polynomial<P>;

    /// **Time complexity** $O(nk)$
    fn fps_exp_sparse(self, n: usize) -> Result<Self::Output, &'static str> {
        if self.coeff_of(0).value() != 0 {
            return Err("定数項が`0`の形式的べき級数のexpを計算しようとした。");
        }

        let mut f = self;
        f.differential();

        let mut g = vec![ConstModInt::new(0); n];
        g[0] = 1.into();

        let mut invs = vec![ConstModInt::new(1); n + 1];
        for i in 2..=n {
            invs[i] = -invs[P::PRIME_NUM as usize % i] * ConstModInt::new(P::PRIME_NUM / i as u32);
        }

        for i in 0..n - 1 {
            let mut s = ConstModInt::new(0);
            for &(j, fj) in f.data.iter() {
                if i >= j {
                    s += fj * g[i - j];
                }
            }

            g[i + 1] = s * invs[i + 1];
        }

        Ok(Polynomial::from(g))
    }
}
