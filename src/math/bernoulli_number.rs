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
pub fn bernoulli_number<const P: u32, const PR: u32>(
    n: usize,
    ft: &FactorialTable<ConstModIntBuilder<P>>,
    ntt: &NTT<P, PR>,
) -> Vec<ConstModInt<P>> {
    let ff = ConstModIntBuilder;
    let fps = PolynomialOperator::new(ntt);
    let mut x: Polynomial<P> = vec![ff.from_u64(0); n + 1].into();

    for i in 0..=n {
        x[i] = ft.inv_facto(i + 1);
    }

    x = fps.fps_inv(x);
    for i in 0..=n {
        x[i] *= ft.facto(i);
    }

    x.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::factorial::bernoulli::BernoulliNumber;

    #[test]
    fn test() {
        let n = 100;

        let ff = ConstModIntBuilder::<998244353>;
        let ntt = NTT::<998244353, 3>::new();
        let ft = FactorialTable::new(n + 1, ff);

        assert_eq!(ft.bernoulli_number(n), bernoulli_number(n, &ft, &ntt));
    }
}
