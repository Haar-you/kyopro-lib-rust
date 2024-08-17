pub use crate::algebra::traits::*;
use crate::impl_algebra;
pub use crate::traits::one_zero::*;
use std::marker::PhantomData;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Prod<T>(PhantomData<T>);
impl<T> Prod<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Prod<T> {
    type Output = T;
}

impl_algebra!(Prod<i8>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});
impl_algebra!(Prod<i16>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});
impl_algebra!(Prod<i32>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});
impl_algebra!(Prod<i64>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});
impl_algebra!(Prod<i128>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});
impl_algebra!(Prod<isize>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});

impl_algebra!(Prod<u8>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});
impl_algebra!(Prod<u16>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});
impl_algebra!(Prod<u32>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});
impl_algebra!(Prod<u64>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});
impl_algebra!(Prod<u128>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});
impl_algebra!(Prod<usize>, binaryop: |_, a, b| a * b, identity: |_| 1, commutative: {}, associative: {});

impl_algebra!(Prod<f32>, binaryop: |_, a, b| a * b, identity: |_| 1.0, commutative: {}, associative: {});
impl_algebra!(Prod<f64>, binaryop: |_, a, b| a * b, identity: |_| 1.0, commutative: {}, associative: {});
