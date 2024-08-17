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

impl_algebra!(BitOr<u8>, binaryop: |_, a, b| a | b, identity: |_| 0, commutative: {}, associative: {}, idempotence: {});
impl_algebra!(BitOr<u16>, binaryop: |_, a, b| a | b, identity: |_| 0, commutative: {}, associative: {}, idempotence: {});
impl_algebra!(BitOr<u32>, binaryop: |_, a, b| a | b, identity: |_| 0, commutative: {}, associative: {}, idempotence: {});
impl_algebra!(BitOr<u64>, binaryop: |_, a, b| a | b, identity: |_| 0, commutative: {}, associative: {}, idempotence: {});
impl_algebra!(BitOr<u128>, binaryop: |_, a, b| a | b, identity: |_| 0, commutative: {}, associative: {}, idempotence: {});
impl_algebra!(BitOr<usize>, binaryop: |_, a, b| a | b, identity: |_| 0, commutative: {}, associative: {}, idempotence: {});
