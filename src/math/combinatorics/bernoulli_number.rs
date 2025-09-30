//! ベルヌーイ数$B_0, \dots, B_n$を列挙する。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/bernoulli_number>
use crate::math::factorial::FactorialTable;
use crate::math::fps::inv::*;
use crate::math::polynomial::*;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// ベルヌーイ数$B_0, \dots, B_n$を列挙する。
pub fn bernoulli_number<P: PrimeMod>(n: usize) -> Vec<ConstModInt<P>> {
    let ft = FactorialTable::new(n + 1, ConstModIntBuilder::new());
    let mut x = vec![ConstModInt::new(0); n + 1];

    for (i, xi) in x.iter_mut().enumerate().take(n + 1) {
        *xi = ft.inv_facto(i + 1);
    }

    x = Polynomial::from(x).fps_inv().unwrap().into();
    for (i, xi) in x.iter_mut().enumerate().take(n + 1) {
        *xi *= ft.facto(i);
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::{factorial::bernoulli::BernoulliNumber, prime_mod::Prime};

    type P = Prime<998244353>;

    #[test]
    fn test() {
        let n = 100;

        let ff = ConstModIntBuilder::<P>::new();
        let ft = FactorialTable::new(n + 1, ff);

        assert_eq!(ft.bernoulli_number(n), bernoulli_number::<P>(n));
    }
}
