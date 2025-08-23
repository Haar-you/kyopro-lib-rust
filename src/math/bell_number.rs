//! ベル数$B_0, \dots, B_n$を列挙する。
use crate::{
    math::{factorial::FactorialTable, fps::exp::*, ntt::*, polynomial::*},
    num::const_modint::*,
};

/// ベル数$B_0, \dots, B_n$を列挙する。
pub fn bell_number<const P: u32, const PR: u32>(
    n: usize,
    ft: &FactorialTable<ConstModIntBuilder<P>>,
    ntt: &NTT<P, PR>,
) -> Vec<ConstModInt<P>> {
    let fps = PolynomialOperator::new(ntt);
    let mut f = vec![ConstModInt::new(0); n + 1];

    for i in 1..=n {
        f[i] = ft.inv_facto(i);
    }

    let mut ret: Vec<_> = fps.fps_exp(f.into()).into();

    for i in 0..=n {
        ret[i] *= ft.facto(i);
    }

    ret
}
