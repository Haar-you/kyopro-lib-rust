//! 疎な形式的冪級数の逆数
use crate::math::polynomial::sparse::SparsePolynomial;
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 疎な形式的冪級数の逆数
pub trait FpsInvSparse {
    /// 戻り値の型
    type Output;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\frac{1}{f(x)}$の先頭$n$項を求める。
    fn fps_inv_sparse(self, n: usize) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsInvSparse for SparsePolynomial<P> {
    type Output = Polynomial<P>;

    /// **Time complexity** $O(nk)$
    fn fps_inv_sparse(self, n: usize) -> Result<Self::Output, &'static str> {
        let f = self;

        let f0 = f.coeff_of(0);
        if f0.value() == 0 {
            return Err("定数項が`0`の形式的べき級数の逆数を計算しようとした。");
        }

        let mut g = vec![ConstModInt::new(0); n];

        g[0] = f0.inv();

        for i in 1..n {
            let mut s = ConstModInt::new(0);
            for &(j, fj) in f.data.iter() {
                if j != 0 && i >= j {
                    s += fj * g[i - j];
                }
            }
            g[i] = -s * g[0];
        }

        Ok(Polynomial::from(g))
    }
}
