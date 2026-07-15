//! 形式的冪級数の合成

use std::iter::successors;

use crate::{
    math::{polynomial::Polynomial, prime_mod::PrimeMod},
    num::const_modint::*,
};

/// 形式的冪級数の合成
pub trait FpsComposition {
    /// 戻り値の型
    type Output;

    /// 2つの形式的冪級数の合成を求める。
    fn fps_composition(self, _: Self) -> Result<Self::Output, &'static str>;
}

impl<P: PrimeMod> FpsComposition for Polynomial<P> {
    type Output = Self;

    /// $f(x) = \sum_0^{n-1} a_ix^i$と$g(x) = \sum_0^{n-1} b_ix^i$について、$f(g(x))$の先頭$n$項を求める。
    ///
    /// $g$の定数項が$0$でないとき、`Err`を返す。
    ///
    /// **Time compexity** $O(N^2)$
    fn fps_composition(self, p: Self) -> Result<Self::Output, &'static str> {
        let ff = ConstModIntBuilder::<P>::new();

        let q = self;
        if p.coeff_of(0) != 0.into() {
            return Err("`p`の定数項は`0`でなければならない。");
        }

        assert!(p.len() == q.len());
        let n = p.len();
        let k = (n as f64).sqrt().ceil() as usize;
        let m = n.div_ceil(k);

        let pp = successors(Some(Self::constant(1.into())), |a| {
            Some((a.clone() * p.clone()).get_until(n))
        })
        .take(m + 1)
        .collect::<Vec<_>>();

        let t = pp[m].clone();
        let tp = successors(Some(Self::constant(1.into())), |a| {
            Some((a.clone() * t.clone()).get_until(n))
        })
        .take(k)
        .collect::<Vec<_>>();

        let mut qp = vec![vec![ff.from_u64(0); n]; k];

        for (i, qpi) in qp.iter_mut().enumerate().take(k) {
            for (j, ppj) in pp.iter().enumerate().take(m) {
                for (l, qpil) in qpi.iter_mut().enumerate().take(n) {
                    *qpil += q.coeff_of(i * m + j) * ppj.coeff_of(l);
                }
            }
        }

        let ret = qp
            .into_iter()
            .zip(tp)
            .map(|(qp, t)| (Self::from(qp) * t).get_until(n))
            .fold(Self::zero(), |a, b| a + b);

        Ok(ret)
    }
}
