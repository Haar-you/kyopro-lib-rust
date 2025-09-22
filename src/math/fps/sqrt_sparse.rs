//! 疎な形式的冪級数の平方根
use crate::math::mod_ops::sqrt::mod_sqrt;
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::math::sparse_polynomial::SparsePolynomial;
use crate::num::const_modint::*;

/// 疎な形式的冪級数の平方根
pub trait FpsSqrtSparse {
    /// 戻り値の型
    type Output;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$\sqrt{f(x)}$の先頭$n$項を求める。
    fn fps_sqrt_sparse(self, n: usize) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsSqrtSparse for SparsePolynomial<P> {
    type Output = Polynomial<P>;

    /// **Time complexity** $O(nk)$
    fn fps_sqrt_sparse(self, n: usize) -> Result<Self::Output, &'static str> {
        let Some(k) = (0..n).find(|&i| self.coeff_of(i).value() != 0) else {
            return Ok(vec![ConstModInt::new(0); n].into());
        };

        if k % 2 == 1 {
            return Err("最小次数が偶数ではない。");
        }

        let a = self.coeff_of(k);
        let sr = ConstModInt::new(
            mod_sqrt(a.value() as u64, P::PRIME_NUM as u64)
                .ok_or("最小次数項の係数に平方根が存在しない。")? as u32,
        );

        let mut f = self;
        f.shift_lower(k);
        f.scale(a.inv());

        let mut g = vec![ConstModInt::new(0); n];
        let mut ret = vec![ConstModInt::new(0); n];
        ret[0] = 1.into();

        let mut invs = vec![ConstModInt::new(1); n + 1];
        for i in 2..=n {
            invs[i] = -invs[P::PRIME_NUM as usize % i] * ConstModInt::new(P::PRIME_NUM / i as u32);
        }

        let half = ConstModInt::new(2).inv();

        for i in 0..n - 1 {
            let mut s = ConstModInt::new(0);

            for &(j, fj) in f.data.iter() {
                if j != 0 {
                    if i >= j {
                        s -= fj * g[i - j];
                    }
                    if i + 1 >= j {
                        s += ret[i - j + 1] * j.into() * fj * half;
                    }
                }
            }

            g[i] = s;
            ret[i + 1] = s * invs[i + 1];
        }

        let mut ret = Polynomial::from(ret);
        ret.scale(sr);
        ret.shift_higher(k / 2);
        Ok(ret)
    }
}
