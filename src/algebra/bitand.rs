pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct BitAnd<T>(PhantomData<T>);
impl<T> BitAnd<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for BitAnd<T> {
    type Output = T;
}

impl_algebra!(BitAnd<u8>, binaryop: |_, a, b| a & b, identity: |_| !0, commutative: {}, associative: {}, idempotence: {});
impl_algebra!(BitAnd<u16>, binaryop: |_, a, b| a & b, identity: |_| !0, commutative: {}, associative: {}, idempotence: {});
impl_algebra!(BitAnd<u32>, binaryop: |_, a, b| a & b, identity: |_| !0, commutative: {}, associative: {}, idempotence: {});
impl_algebra!(BitAnd<u64>, binaryop: |_, a, b| a & b, identity: |_| !0, commutative: {}, associative: {}, idempotence: {});
impl_algebra!(BitAnd<u128>, binaryop: |_, a, b| a & b, identity: |_| !0, commutative: {}, associative: {}, idempotence: {});
impl_algebra!(BitAnd<usize>, binaryop: |_, a, b| a & b, identity: |_| !0, commutative: {}, associative: {}, idempotence: {});
