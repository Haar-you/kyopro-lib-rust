pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Sum<T>(PhantomData<T>);
impl<T> Sum<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Sum<T> {
    type Output = T;
}

impl_algebra!(Sum<i8>, binaryop: |_, a, b| a + b, identity: |_| 0, inverse: |_, a: i8| -a, commutative: {}, associative: {});
impl_algebra!(Sum<i16>, binaryop: |_, a, b| a + b, identity: |_| 0, inverse: |_, a: i16| -a, commutative: {}, associative: {});
impl_algebra!(Sum<i32>, binaryop: |_, a, b| a + b, identity: |_| 0, inverse: |_, a: i32| -a, commutative: {}, associative: {});
impl_algebra!(Sum<i64>, binaryop: |_, a, b| a + b, identity: |_| 0, inverse: |_, a: i64| -a, commutative: {}, associative: {});
impl_algebra!(Sum<i128>, binaryop: |_, a, b| a + b, identity: |_| 0, inverse: |_, a: i128| -a, commutative: {}, associative: {});
impl_algebra!(Sum<isize>, binaryop: |_, a, b| a + b, identity: |_| 0, inverse: |_, a: isize| -a, commutative: {}, associative: {});

impl_algebra!(Sum<u8>, binaryop: |_, a, b| a + b, identity: |_| 0, commutative: {}, associative: {});
impl_algebra!(Sum<u16>, binaryop: |_, a, b| a + b, identity: |_| 0, commutative: {}, associative: {});
impl_algebra!(Sum<u32>, binaryop: |_, a, b| a + b, identity: |_| 0, commutative: {}, associative: {});
impl_algebra!(Sum<u64>, binaryop: |_, a, b| a + b, identity: |_| 0, commutative: {}, associative: {});
impl_algebra!(Sum<u128>, binaryop: |_, a, b| a + b, identity: |_| 0, commutative: {}, associative: {});
impl_algebra!(Sum<usize>, binaryop: |_, a, b| a + b, identity: |_| 0, commutative: {}, associative: {});

impl_algebra!(Sum<f32>, binaryop: |_, a, b| a + b, identity: |_| 0.0, inverse: |_, a: f32| -a, commutative: {}, associative: {});
impl_algebra!(Sum<f64>, binaryop: |_, a, b| a + b, identity: |_| 0.0, inverse: |_, a: f64| -a, commutative: {}, associative: {});
