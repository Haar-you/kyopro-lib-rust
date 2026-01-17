//! `mod m`の代数的構造

use std::marker::PhantomData;

pub use crate::algebra::traits::*;
use crate::num::one_zero::*;
use crate::{impl_algebra, num::ff::*};

/// `mod m`上の加法
#[derive(Clone, Copy, Default, Debug)]
pub struct SumMod<T>(PhantomData<T>);
impl<T> SumMod<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(
    [T: ZZElem + Zero];
    SumMod<T>;
    set: T;
    op: |_, a: T, b: T| a + b;
    id: |_| T::zero(), |_, a: &T| a.value() == 0;
    inv: |_, a: T| -a;
    assoc;
    commu;
);

/// `mod m`上の乗法
#[derive(Clone, Copy, Default, Debug)]
pub struct ProdMod<T>(PhantomData<T>);
impl<T> ProdMod<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(
    [T: ZZElem + One];
    ProdMod<T>;
    set: T;
    op: |_, a: T, b: T|  a * b;
    id: |_| T::one(), |_, a: &T| a.value() == 1;
    assoc;
    commu;
);
impl_algebra!([T: FFElem + One]; ProdMod<T>; inv: |_, a: T| a.inv(););
