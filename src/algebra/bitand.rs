//! 論理積を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 論理積を演算とする代数的構造
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct BitAnd<T>(PhantomData<T>);
impl<T> BitAnd<T> {
    /// `BitAnd<T>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for BitAnd<T> {
    type Output = T;
}

impl_algebra!(BitAnd<u8>, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitAnd<u16>, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitAnd<u32>, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitAnd<u64>, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitAnd<u128>, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitAnd<usize>, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
