//! `mod m`の代数的構造

pub use crate::algebra::traits::*;
use crate::{impl_algebra, num::ff::*};

/// `mod m`上の加法
#[derive(Clone, Copy, Default, Debug)]
pub struct SumMod<T>(Option<T>);
impl<T: ZZElem> SumMod<T> {
    /// 値を`SumMod`に入れる。
    pub fn new(a: T) -> Self {
        Self(Some(a))
    }
    /// 値を`u32`として返す。
    pub fn value(self) -> u32 {
        self.0.map_or(0, |a| a.value())
    }
}
impl<T: ZZElem> PartialEq for SumMod<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl<T: ZZElem> Eq for SumMod<T> {}
impl_algebra!(
    [T: ZZElem];
    SumMod<T>;
    set;
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
pub struct ProdMod<T>(Option<T>);
impl<T: ZZElem> ProdMod<T> {
    /// 値を`ProdMod`に入れる。
    pub fn new(a: T) -> Self {
        Self(Some(a))
    }
    /// 値を`u32`として返す。
    pub fn value(self) -> u32 {
        self.0.map_or(1, |a| a.value())
    }
}
impl<T: ZZElem> PartialEq for ProdMod<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl<T: ZZElem> Eq for ProdMod<T> {}
impl_algebra!(
    [T: ZZElem];
    ProdMod<T>;
    set;
    op: |a: Self, b: Self| match (a.0, b.0) {
        (Some(a), Some(b)) => Self(Some(a * b)),
        (a, None) | (None, a) => Self(a),
    };
    id: Self(None);
    assoc;
    commu;
);
impl_algebra!([T: FFElem]; ProdMod<T>; inv: |a: Self| Self(a.0.map(|x| x.inv())););
