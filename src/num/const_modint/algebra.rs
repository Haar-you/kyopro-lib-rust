pub use crate::algebra::traits::*;
use crate::{impl_algebra, num::const_modint::ConstModInt};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Sum<const M: u32>;

impl_algebra!(<const M: u32>; Sum<M>,
    set: ConstModInt<M>,
    op: |_, a, b| a + b,
    id: |_| ConstModInt::new(0),
    inv: |_, a: Self::Element| -a,
    assoc: {},
    commu: {}
);

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Prod<const M: u32>;

impl_algebra!(<const M: u32>; Prod<M>,
    set: ConstModInt<M>,
    op: |_, a, b| a * b,
    id: |_| ConstModInt::new(1),
    assoc: {},
    commu: {}
);
