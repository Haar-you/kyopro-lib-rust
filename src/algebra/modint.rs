//! `mod m`の代数的構造

pub use crate::algebra::traits::*;
use crate::{impl_algebra, num::ff::*};

/// `mod m`上の加法
#[derive(Clone, Copy, Default, Debug)]
pub struct SumMod<T: ZZ>(T);
impl<T: ZZ> SumMod<T> {
    /// [`SumMod<T>`]を作る。
    pub fn new(m: T) -> Self {
        Self(m)
    }
}

impl<T: ZZ> Additive for SumMod<T> {
    fn times(&self, a: Self::Element, n: u64) -> Self::Element {
        a * self.0.from_u64(n)
    }
}

impl_algebra!(
    {T: ZZ} SumMod<T>;
    set: T::Element;
    op: |_, a: T::Element, b: T::Element| a + b;
    id: |s: &Self| s.0.zero(), |_, a: &T::Element| a.value() == 0;
    inv: |_, a: T::Element| -a;
    assoc;
    commu;
);

/// `mod m`上の乗法
#[derive(Clone, Copy, Default, Debug)]
pub struct ProdMod<T: ZZ>(T);
impl<T: ZZ> ProdMod<T> {
    /// [`ProdMod<T>`]を作る。
    pub fn new(m: T) -> Self {
        Self(m)
    }
}

impl<T: ZZ> Multiplicative for ProdMod<T> {}

impl_algebra!(
    {T: ZZ} ProdMod<T>;
    set: T::Element;
    op: |_, a: T::Element, b: T::Element|  a * b;
    id: |s: &Self| s.0.one(), |_, a: &T::Element| a.value() == 1;
    assoc;
    commu;
);
impl_algebra!({T: FF} ProdMod<T> where {T::Element: FFElem}; inv: |_, a: T::Element| a.inv(););
