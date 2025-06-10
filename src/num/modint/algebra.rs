//! [`ModInt`]の代数的構造

pub use crate::algebra::traits::*;
use crate::{impl_algebra, num::modint::*};

/// `mod m`上の加法
#[derive(Clone, Copy, Default, Debug)]
pub struct SumModM(Option<ModInt>);
impl SumModM {
    pub fn new(a: ModInt) -> Self {
        Self(Some(a))
    }
    pub fn value(self) -> u32 {
        self.0.map_or(0, |a| a.value)
    }
}
impl PartialEq for SumModM {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl Eq for SumModM {}
impl_algebra!(
    SumModM;
    op: |a: Self, b: Self| match (a.0, b.0) {
        (Some(a), Some(b)) => Self(Some(a + b)),
        (a, None) | (None, a) => Self(a),
    };
    id: Self(None);
    inv: |a: Self| Self(a.0.map(|x| -x));
    assoc;
    commu;
);

/// `mod m`上の乗法
#[derive(Clone, Copy, Default, Debug)]
pub struct ProdModM(Option<ModInt>);
impl ProdModM {
    pub fn new(a: ModInt) -> Self {
        Self(Some(a))
    }
    pub fn value(self) -> u32 {
        self.0.map_or(1, |a| a.value)
    }
}
impl PartialEq for ProdModM {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl Eq for ProdModM {}
impl_algebra!(
    ProdModM;
    op: |a: Self, b: Self| match (a.0, b.0) {
        (Some(a), Some(b)) => Self(Some(a * b)),
        (a, None) | (None, a) => Self(a),
    };
    id: Self(None);
    assoc;
    commu;
);
