//! 疎な形式的冪級数の累乗
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::math::sparse_polynomial::SparsePolynomial;
use crate::num::const_modint::*;

/// 疎な形式的冪級数の累乗
pub trait FpsPowSparse {
    /// 戻り値の型
    type Output;

    /// $f(x) = \sum_0^{n-1} a_ix^i$について、$(f(x))^m$の先頭$n$項を求める。
    fn fps_pow_sparse(self, m: u64, n: usize) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsPowSparse for SparsePolynomial<P> {
    type Output = Polynomial<P>;

    /// **Time complexity** $O(nk)$
    fn fps_pow_sparse(self, m: u64, n: usize) -> Result<Self::Output, &'static str> {
        if m == 0 {
            let mut f: Vec<_> = vec![ConstModInt::new(0); n];
            f[0] = ConstModInt::new(1);
            return Ok(f.into());
        }

        let k = (0..n).find(|&i| self.coeff_of(i).value() != 0).unwrap_or(n);

        if k >= n {
            return Ok(vec![ConstModInt::new(0); n].into());
        }

        if k.checked_mul(m as usize).is_none_or(|x| x >= n) {
            return Ok(vec![ConstModInt::new(0); n].into());
        }

        let a = self.coeff_of(k);

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

        for i in 0..n - 1 {
            let mut s = ConstModInt::new(0);

            for &(j, fj) in f.data.iter() {
                if j != 0 {
                    if i >= j {
                        s -= fj * g[i - j];
                    }
                    if i + 1 >= j {
                        s += ret[i - j + 1] * j.into() * fj * m.into();
                    }
                }
            }

            g[i] = s;
            ret[i + 1] = s * invs[i + 1];
        }

        let mut ret = Polynomial::from(ret);
        ret.scale(a.pow(m));
        ret.shift_higher(m as usize * k);
        Ok(ret)
    }
}
