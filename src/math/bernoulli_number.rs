//! ベルヌーイ数$B_0, \dots, B_n$を列挙する。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/bernoulli_number>
use crate::math::factorial::FactorialTable;
use crate::math::fps::inv::*;
use crate::math::ntt::*;
use crate::math::polynomial::*;
use crate::num::const_modint::*;

/// ベルヌーイ数$B_0, \dots, B_n$を列挙する。
pub fn bernoulli_number<const P: u32, const PR: u32>(n: usize) -> Vec<ConstModInt<P>> {
    let ft = FactorialTable::new(n + 1, ConstModIntBuilder);
    let ntt = NTT::<P, PR>::new();
    let fps = PolynomialOperator::new(&ntt);
    let mut x = vec![ConstModInt::new(0); n + 1];

    for (i, xi) in x.iter_mut().enumerate().take(n + 1) {
        *xi = ft.inv_facto(i + 1);
    }

    x = fps.fps_inv(x.into()).unwrap().into();
    for (i, xi) in x.iter_mut().enumerate().take(n + 1) {
        *xi *= ft.facto(i);
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::factorial::bernoulli::BernoulliNumber;

    #[test]
    fn test() {
        let n = 100;

        let ff = ConstModIntBuilder::<998244353>;
        let ft = FactorialTable::new(n + 1, ff);

        assert_eq!(ft.bernoulli_number(n), bernoulli_number::<998244353, 3>(n));
    }
}
