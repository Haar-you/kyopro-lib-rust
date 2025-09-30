//! ベル数$B_0, \dots, B_n$を列挙する。
use crate::math::prime_mod::PrimeMod;
use crate::{
    math::{factorial::FactorialTable, fps::exp::*, polynomial::*},
    num::const_modint::*,
};

/// ベル数$B_0, \dots, B_n$を列挙する。
pub fn bell_number<P: PrimeMod>(n: usize) -> Vec<ConstModInt<P>> {
    let ft = FactorialTable::new(n, ConstModIntBuilder::new());
    let mut f = vec![ConstModInt::new(0); n + 1];

    for (i, fi) in f.iter_mut().enumerate().take(n + 1).skip(1) {
        *fi = ft.inv_facto(i);
    }

    let mut ret: Vec<_> = Polynomial::from(f).fps_exp().unwrap().into();

    for (i, reti) in ret.iter_mut().enumerate().take(n + 1) {
        *reti *= ft.facto(i);
    }

    ret
}
