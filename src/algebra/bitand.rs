pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct BitAnd<T>(PhantomData<T>);
impl<T> BitAnd<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(BitAnd<u8>, set: u8, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitAnd<u16>, set: u16, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitAnd<u32>, set: u32, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitAnd<u64>, set: u64, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitAnd<u128>, set: u128, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitAnd<usize>, set: usize, op: |_, a, b| a & b, id: |_| !0, commu: {}, assoc: {}, idem: {});
