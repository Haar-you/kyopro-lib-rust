pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct BitXor<T>(PhantomData<T>);
impl<T> BitXor<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for BitXor<T> {
    type Output = T;
}

impl_algebra!(BitXor<i8>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
impl_algebra!(BitXor<i16>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
impl_algebra!(BitXor<i32>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
impl_algebra!(BitXor<i64>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
impl_algebra!(BitXor<i128>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
impl_algebra!(BitXor<isize>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});

impl_algebra!(BitXor<u8>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
impl_algebra!(BitXor<u16>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
impl_algebra!(BitXor<u32>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
impl_algebra!(BitXor<u64>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
impl_algebra!(BitXor<u128>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
impl_algebra!(BitXor<usize>, binaryop: |_, a, b| a ^ b, identity: |_| 0, inverse: |_, a| a, commutative: {}, associative: {});
