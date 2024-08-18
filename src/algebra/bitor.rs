pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct BitOr<T>(PhantomData<T>);
impl<T> BitOr<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for BitOr<T> {
    type Output = T;
}

impl_algebra!(BitOr<u8>, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitOr<u16>, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitOr<u32>, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitOr<u64>, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitOr<u128>, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitOr<usize>, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
