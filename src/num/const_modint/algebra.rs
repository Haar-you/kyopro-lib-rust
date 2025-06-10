//! [`ConstModInt<M>`]の代数的構造

pub use crate::algebra::traits::*;
pub use crate::algebra::{prod::Prod, sum::Sum};
use crate::{impl_algebra, num::const_modint::ConstModInt};

impl_algebra!(
    [const M: u32]; Sum<ConstModInt<M>>;
    op: |a: Self, b: Self| Self(a.0 + b.0);
    id: Self(ConstModInt::new(0));
    inv: |a: Self| Self(-a.0);
    assoc;
    commu;
);

impl_algebra!(
    [const M: u32]; Prod<ConstModInt<M>>;
    op: |a: Self, b: Self| Self(a.0 * b.0);
    id: Self(ConstModInt::new(1));
    assoc;
    commu;
);
