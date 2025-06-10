//! ベルヌーイ数$B_0, \dots, B_n$を列挙する。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/bernoulli_number>
use crate::math::factorial::FactorialTable;
use crate::math::fps::FPS;
use crate::math::polynomial::Polynomial;
use crate::num::const_modint::*;

/// ベルヌーイ数$B_0, \dots, B_n$を列挙する。
pub fn bernoulli_number<Fps, const P: u32>(
    n: usize,
    ft: &FactorialTable<ConstModIntBuilder<P>>,
    fps: &Fps,
) -> Vec<ConstModInt<P>>
where
    Fps: FPS<Poly = Polynomial<P>>,
{
    let ff = ConstModIntBuilder;
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
    use crate::math::{factorial::*, ntt::*, polynomial::*};

    #[test]
    fn test() {
        let n = 100;

        let ff = ConstModIntBuilder::<998244353>;
        let ntt = NTT::<998244353, 3>::new();
        let fps = PolynomialOperator::new(&ntt);
        let ft = FactorialTable::new(n + 1, ff);

        assert_eq!(ft.bernoulli_number(n), bernoulli_number(n, &ft, &fps));
    }
}
