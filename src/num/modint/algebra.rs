//! [`ModInt`]の代数的構造

pub use crate::algebra::traits::*;
use crate::{impl_algebra, num::modint::*};

/// `mod m`上の加法
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct SumModM {
    m: u32,
}
impl SumModM {
    /// `m`を法とした`SumModM`を生成する。
    pub fn new(m: u32) -> Self {
        Self { m }
    }
}
impl_algebra!(SumModM,
    set: ModInt,
    op: |_, a, b| a + b,
    id: |s: &Self| ModInt::new(0, s.m),
    inv: |_, a: Self::Element| -a,
    assoc: {},
    commu: {}
);

/// `mod m`上の乗法
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct ProdModM {
    m: u32,
}
impl ProdModM {
    /// `m`を法とした`ProdModM`を生成する。
    pub fn new(m: u32) -> Self {
        Self { m }
    }
}
impl_algebra!(ProdModM,
    set: ModInt,
    op: |_, a, b| a * b,
    id: |s: &Self| ModInt::new(1, s.m),
    assoc: {},
    commu: {}
);
