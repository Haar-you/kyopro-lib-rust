//! 多項式$f(x)$に対して、$f(x + c)$の係数を求める。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/polynomial_taylor_shift>

use crate::math::ntt::*;
use crate::num::const_modint::*;

type Poly<const P: u32> = Vec<ConstModInt<P>>;

/// 多項式 `p` = $f(x) = a_0 + a_1x + \cdots + a_nx^n$に対して、<br>
/// 多項式 $f(x + c) = a_0 + a_1(x + c) + \cdots + a_n(x + c)^n = b_0 + b_0x + \cdots + b_nx^n$
/// を満たす、数列{$b_i$}を求める。
pub fn polynomial_taylor_shift<const P: u32>(
    p: Poly<P>,
    c: ConstModInt<P>,
    ntt: &NTT<P>,
) -> Poly<P> {
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

    let c = ntt.convolve(a, b);
    c.into_iter()
        .skip((n - 1) * 2)
        .zip(g)
        .map(|(c, g)| c * g)
        .collect()
}
