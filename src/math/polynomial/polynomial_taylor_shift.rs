//! 多項式$f(x)$に対して、$f(x + c)$の係数を求める。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/polynomial_taylor_shift>

use crate::math::polynomial::*;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// Polynomial Taylor shift
pub trait TaylorShift {
    /// 多項式の係数の型
    type Value;

    /// 多項式 `p` = $f(x) = a_0 + a_1x + \cdots + a_nx^n$に対して、<br>
    /// 多項式 $f(x + c) = a_0 + a_1(x + c) + \cdots + a_n(x + c)^n = b_0 + b_0x + \cdots + b_nx^n$
    /// を満たす、数列{$b_i$}を求める。
    fn taylor_shift(self, c: Self::Value) -> Self;
}

impl<P: PrimeMod> TaylorShift for Polynomial<P> {
    type Value = ConstModInt<P>;

    fn taylor_shift(self, c: Self::Value) -> Self {
        let p: Vec<_> = self.into();
        let n = p.len();
        let mut f = ConstModInt::new(1);

        let mut a = vec![ConstModInt::new(0); 2 * n - 1];
        for (i, (a, p)) in a.iter_mut().skip(n - 1).zip(p.into_iter()).enumerate() {
            if i != 0 {
                f *= ConstModInt::new(i as u32);
            }
            *a = p * f;
        }

        let mut g = vec![ConstModInt::new(0); n];
        g[n - 1] = f.inv();
        for i in (0..n - 1).rev() {
            g[i] = g[i + 1] * ConstModInt::new(i as u32 + 1);
        }

        let mut d = ConstModInt::new(1);
        let mut b = vec![ConstModInt::new(0); 2 * n - 1];
        for (b, g) in b.iter_mut().take(n).rev().zip(g.iter()) {
            *b = d * *g;
            d *= c;
        }

        let c = Self::NTT.convolve(a, b);
        c.into_iter()
            .skip((n - 1) * 2)
            .zip(g)
            .map(|(c, g)| c * g)
            .collect::<Vec<_>>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::prime_mod::Prime;

    use super::*;

    type P = Prime<998244353>;

    #[test]
    fn test() {
        let coeffs = vec![1, 2, 3, 4, 5];
        let c = 3;

        let res = Polynomial::<P>::from(coeffs.clone()).taylor_shift(c.into());

        let mut ans = Polynomial::zero();
        for (i, a) in coeffs.into_iter().enumerate() {
            let mut p = Polynomial::<P>::from(vec![c, 1]).pow(i as u64);
            p.scale(a.into());
            ans += p;
        }

        assert_eq!(res, ans);
    }
}
