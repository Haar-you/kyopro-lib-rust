//! 多項式の根
//!
//! # References
//! - <https://maspypy.com/library-checker-mod-p-equation>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/polynomial_root_finding>

use crate::{
    math::{polynomial::Polynomial, prime_mod::PrimeMod},
    num::const_modint::ConstModInt,
    rand::rand,
};

/// 多項式の根
pub trait RootFinding {
    /// 多項式の係数の型
    type Value;

    /// 多項式の根を求める。
    fn root_finding(self) -> Vec<Self::Value>;
}

impl<P: PrimeMod + std::fmt::Debug> RootFinding for Polynomial<P> {
    type Value = ConstModInt<P>;

    fn root_finding(self) -> Vec<Self::Value> {
        let x = Self::from(vec![0, 1]);

        let g = x.clone().pow_mod(P::PRIME_NUM as u64, self.clone()) - x;
        let g = g.gcd(self);

        let mut fs = vec![g];
        let mut ret = vec![];

        while let Some(f) = fs.pop() {
            match f.deg() {
                Some(1) => ret.push(-f.coeff_of(0) / f.coeff_of(1)),
                Some(0) | None => {}
                Some(n) => {
                    let g = std::iter::repeat_with(|| rand() % P::PRIME_NUM as u64)
                        .take(n)
                        .collect::<Vec<_>>();

                    let g = Self::from(g);
                    let h = g.pow_mod((P::PRIME_NUM as u64 - 1) / 2, f.clone());

                    let f1 = f.clone().gcd(h - Self::constant(1.into()));
                    let f2 = f / f1.clone();

                    fs.push(f1);
                    fs.push(f2);
                }
            }
        }

        ret
    }
}
