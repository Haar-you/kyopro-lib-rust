//! 符号付き第一種スターリング数$s(0, k), \dots, s(n, k)$を列挙する。
use crate::math::factorial::FactorialTable;
use crate::math::fps::pow::FpsPow;
use crate::math::mod_ops::enum_inv::enumerate_mod_inv;
use crate::math::polynomial::PolynomialOperator;
use crate::num::const_modint::*;

/// 符号付き第一種スターリング数$s(0, k), \dots, s(n, k)$を列挙する。
pub fn stirling_first_fixed_k<const P: u32, const PR: u32>(
    n: usize,
    k: usize,
) -> Vec<ConstModInt<P>> {
    assert!(k <= n);

    let fps = PolynomialOperator::<P, PR>::new();
    let ft = FactorialTable::new(n, ConstModIntBuilder);

    let mut ret: Vec<ConstModInt<P>> = enumerate_mod_inv(n, P as u64)
        .into_iter()
        .map(Into::into)
        .collect();

    for i in (2..=n).step_by(2) {
        ret[i] = -ret[i];
    }

    ret = fps.fps_pow(ret.into(), k as u64).unwrap().into();

    for (i, reti) in ret.iter_mut().enumerate().take(n + 1).skip(k) {
        *reti *= ft.inv_facto(k) * ft.facto(i);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::stirling_first_table::stirling_first_table;

    #[test]
    fn test() {
        let n = 100;
        let ans = stirling_first_table(n, ConstModIntBuilder::<998244353>);

        for k in 0..=n {
            assert_eq!(
                stirling_first_fixed_k::<998244353, 3>(n, k),
                ans.iter().map(|a| a[k]).collect::<Vec<_>>()
            );
        }
    }
}
