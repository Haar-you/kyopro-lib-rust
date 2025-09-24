//! 第二種スターリング数$S(0, k), \dots, S(n, k)$を列挙する。
use crate::math::factorial::FactorialTable;
use crate::math::fps::pow::FpsPow;
use crate::math::polynomial::PolynomialOperator;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// 第二種スターリング数$S(0, k), \dots, S(n, k)$を列挙する。
pub fn stirling_second_fixed_k<P: PrimeMod>(n: usize, k: usize) -> Vec<ConstModInt<P>> {
    assert!(k <= n);

    let fps = PolynomialOperator::<P>::new();
    let ft = FactorialTable::new(n, ConstModIntBuilder::new());

    let mut ret = vec![ConstModInt::new(0); n + 1];

    for (i, reti) in ret.iter_mut().enumerate().take(n + 1).skip(1) {
        *reti = ft.inv_facto(i);
    }

    ret = fps.fps_pow(ret.into(), k as u64).unwrap().into();

    for (i, reti) in ret.iter_mut().enumerate().take(n + 1).skip(k) {
        *reti *= ft.inv_facto(k) * ft.facto(i);
    }

    ret
}
