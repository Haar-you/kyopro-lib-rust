//! 疎な形式的冪級数の指数関数
use crate::math::polynomial::Polynomial;
use crate::math::sparse_polynomial::SparsePolynomial;
use crate::num::const_modint::*;

/// 疎な形式的冪級数の指数関数
pub trait FpsExpSparse {
    /// 戻り値の型
    type Output;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\exp (f(x))$の先頭$n$項を求める。
    fn fps_exp_sparse(self, n: usize) -> Self::Output;
}

impl<const P: u32> FpsExpSparse for SparsePolynomial<P> {
    type Output = Polynomial<P>;

    /// **Time complexity** $O(nk)$
    fn fps_exp_sparse(self, n: usize) -> Self::Output {
        let mut f = self;
        f.differential();

        let mut g = vec![ConstModInt::new(0); n];
        g[0] = 1.into();

        let mut invs = vec![ConstModInt::new(1); n + 1];
        for i in 2..=n {
            invs[i] = -invs[P as usize % i] * ConstModInt::new(P / i as u32);
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

        Polynomial::from(g)
    }
}
