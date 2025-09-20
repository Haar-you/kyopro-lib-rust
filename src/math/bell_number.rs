//! ベル数$B_0, \dots, B_n$を列挙する。
use crate::{
    math::{factorial::FactorialTable, fps::exp::*, ntt::*, polynomial::*},
    num::const_modint::*,
};

/// ベル数$B_0, \dots, B_n$を列挙する。
pub fn bell_number<const P: u32, const PR: u32>(n: usize) -> Vec<ConstModInt<P>> {
    let ft = FactorialTable::new(n, ConstModIntBuilder);
    let ntt = NTT::<P, PR>::new();
    let fps = PolynomialOperator::new(&ntt);
    let mut f = vec![ConstModInt::new(0); n + 1];

    for (i, fi) in f.iter_mut().enumerate().take(n + 1).skip(1) {
        *fi = ft.inv_facto(i);
    }

    let mut ret: Vec<_> = fps.fps_exp(f.into()).unwrap().into();

    for (i, reti) in ret.iter_mut().enumerate().take(n + 1) {
        *reti *= ft.facto(i);
    }

    ret
}
