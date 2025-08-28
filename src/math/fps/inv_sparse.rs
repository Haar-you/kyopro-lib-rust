//! 疎な形式的冪級数の逆数
use crate::math::polynomial::Polynomial;
use crate::math::sparse_polynomial::SparsePolynomial;
use crate::num::const_modint::*;

/// 疎な形式的冪級数の逆数
pub trait FpsInvSparse {
    /// 戻り値の型
    type Output;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\frac{1}{f(x)}$の先頭$n$項を求める。
    fn fps_inv_sparse(self, n: usize) -> Self::Output;
}

impl<const P: u32> FpsInvSparse for SparsePolynomial<P> {
    type Output = Polynomial<P>;

    /// **Time complexity** $O(nk)$
    fn fps_inv_sparse(self, n: usize) -> Self::Output {
        let f = self;

        let f0 = f.coeff_of(0);
        if f0.value() == 0 {
            panic!("f[0] == 0");
        }

        let mut g = vec![ConstModInt::new(0); n];

        g[0] = f0.inv();

        for i in 1..n {
            for (&j, &fj) in f.data.iter() {
                if j != 0 && i >= j {
                    let t = fj * g[i - j] * g[0];
                    g[i] -= t;
                }
            }
        }

        Polynomial::from(g)
    }
}
