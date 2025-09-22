//! 多項式$f(x)$に対して、$f(x + c)$の係数を求める。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/polynomial_taylor_shift>

use crate::math::polynomial::*;
use crate::num::const_modint::*;

/// Polynomial Taylor shift
pub trait TaylorShift {
    /// 多項式の型
    type Poly;
    /// 多項式の係数の型
    type Value;

    /// 多項式 `p` = $f(x) = a_0 + a_1x + \cdots + a_nx^n$に対して、<br>
    /// 多項式 $f(x + c) = a_0 + a_1(x + c) + \cdots + a_n(x + c)^n = b_0 + b_0x + \cdots + b_nx^n$
    /// を満たす、数列{$b_i$}を求める。
    fn taylor_shift(&self, p: Self::Poly, c: Self::Value) -> Self::Poly;
}

impl<const P: u32, const PR: u32> TaylorShift for PolynomialOperator<P, PR> {
    type Poly = Polynomial<P>;
    type Value = ConstModInt<P>;

    fn taylor_shift(&self, p: Self::Poly, c: Self::Value) -> Self::Poly {
        let p: Vec<_> = p.into();
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

        //    let c = ntt.convolve(a, b);
        let c: Vec<_> = self.mul(a.into(), b.into()).into();
        c.into_iter()
            .skip((n - 1) * 2)
            .zip(g)
            .map(|(c, g)| c * g)
            .collect::<Vec<_>>()
            .into()
    }
}
