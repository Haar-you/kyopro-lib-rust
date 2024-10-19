pub use crate::algebra::traits::*;
use crate::{impl_algebra, num::modint::*};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Sum {
    m: u32,
}
impl Sum {
    pub fn new(m: u32) -> Self {
        Self { m }
    }
}

impl_algebra!(Sum,
    set: ModInt,
    op: |_, a, b| a + b,
    id: |s: &Self| ModInt::new(0, s.m),
    inv: |_, a: Self::Element| -a,
    assoc: {},
    commu: {}
);

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Prod {
    m: u32,
}
impl Prod {
    pub fn new(m: u32) -> Self {
        Self { m }
    }
}

impl_algebra!(Prod,
    set: ModInt,
    op: |_, a, b| a * b,
    id: |s: &Self| ModInt::new(1, s.m),
    assoc: {},
    commu: {}
);
