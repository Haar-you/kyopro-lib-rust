//! 第二種スターリング数$S(0, k), \dots, S(n, k)$を列挙する。
use crate::math::factorial::FactorialTable;
use crate::math::fps::pow::FpsPow;
use crate::math::ntt::NTT;
use crate::math::polynomial::PolynomialOperator;
use crate::num::const_modint::*;

/// 第二種スターリング数$S(0, k), \dots, S(n, k)$を列挙する。
pub fn stirling_second_fixed_k<const P: u32, const PR: u32>(
    n: usize,
    k: usize,
) -> Vec<ConstModInt<P>> {
    assert!(k <= n);

    let ntt = NTT::<P, PR>::new();
    let fps = PolynomialOperator::new(&ntt);
    let ft = FactorialTable::new(n, ConstModIntBuilder);

    let mut ret = vec![ConstModInt::new(0); n + 1];

    for i in 1..=n {
        ret[i] = ft.inv_facto(i);
    }

    ret = fps.fps_pow(ret.into(), k as u64).unwrap().into();

    for i in k..=n {
        ret[i] *= ft.inv_facto(k) * ft.facto(i);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::stirling_second_table::stirling_second_table;

    #[test]
    fn test() {
        let n = 100;
        let ans = stirling_second_table(n, ConstModIntBuilder::<998244353>);

        for k in 0..=n {
            assert_eq!(
                stirling_second_fixed_k::<998244353, 3>(n, k),
                ans.iter().map(|a| a[k]).collect::<Vec<_>>()
            );
        }
    }
}
