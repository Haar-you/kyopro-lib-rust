//! 符号付き第一種スターリング数$s(0, k), \dots, s(n, k)$を列挙する。
use crate::math::factorial::FactorialTable;
use crate::math::fps::pow::FpsPow;
use crate::math::mod_ops::enum_inv::enumerate_mod_inv;
use crate::math::polynomial::Polynomial;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 符号付き第一種スターリング数$s(0, k), \dots, s(n, k)$を列挙する。
pub fn stirling_first_fixed_k<P: PrimeMod>(n: usize, k: usize) -> Vec<ConstModInt<P>> {
    assert!(k <= n);

    let ft = FactorialTable::new(n, ConstModIntBuilder::new());

    let mut ret: Vec<ConstModInt<P>> = enumerate_mod_inv(n, P::PRIME_NUM as u64)
        .into_iter()
        .map(Into::into)
        .collect();

    for i in (2..=n).step_by(2) {
        ret[i] = -ret[i];
    }

    ret = Polynomial::from(ret).fps_pow(k as u64).unwrap().into();

    for (i, reti) in ret.iter_mut().enumerate().take(n + 1).skip(k) {
        *reti *= ft.inv_facto(k) * ft.facto(i);
    }

    ret
}
