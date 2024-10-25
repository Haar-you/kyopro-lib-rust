//! [`ConstModInt<M>`]の代数的構造

pub use crate::algebra::traits::*;
pub use crate::algebra::{prod::Prod, sum::Sum};
use crate::{impl_algebra, num::const_modint::ConstModInt};

impl_algebra!(<const M: u32>; Sum<ConstModInt<M>>,
    set: ConstModInt<M>,
    op: |_, a, b| a + b,
    id: |_| ConstModInt::new(0),
    inv: |_, a: Self::Element| -a,
    assoc: {},
    commu: {}
);

impl_algebra!(<const M: u32>; Prod<ConstModInt<M>>,
    set: ConstModInt<M>,
    op: |_, a, b| a * b,
    id: |_| ConstModInt::new(1),
    assoc: {},
    commu: {}
);
