//! 疎な形式的冪級数の対数
use crate::math::fps::inv_sparse::*;
use crate::math::polynomial::sparse::SparsePolynomial;
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 疎な形式的冪級数の対数
pub trait FpsLogSparse {
    /// 戻り値の型
    type Output;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\log f(x)$の先頭$n$項を求める。
    fn fps_log_sparse(self, n: usize) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsLogSparse for SparsePolynomial<P> {
    type Output = Polynomial<P>;

    /// **Time complexity** $O(nk)$
    fn fps_log_sparse(self, n: usize) -> Result<Self::Output, &'static str> {
        let mut f = self.clone();
        f.differential();

        let g = self.fps_inv_sparse(n)?;

        let mut h = vec![ConstModInt::new(0); n];
        for (&i, &x) in f.iter() {
            for (&y, h) in g.data.iter().zip(h.iter_mut().skip(i)) {
                *h += x * y;
            }
        }

        let mut invs = vec![ConstModInt::new(1); n + 1];
        for i in 2..=n {
            invs[i] = -invs[P::PRIME_NUM as usize % i] * ConstModInt::new(P::PRIME_NUM / i as u32);
        }

        for i in (0..n - 1).rev() {
            h[i + 1] = h[i] * invs[i + 1];
        }
        h[0] = 0.into();

        Ok(Polynomial::from(h))
    }
}
