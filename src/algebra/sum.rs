//! 加法を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 加法を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Sum<T>(PhantomData<T>);
impl<T> Sum<T> {
    /// `Sum<T>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Sum<T> {
    type Output = T;
}

impl_algebra!(Sum<i8>, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: i8| -a, commu: {}, assoc: {});
impl_algebra!(Sum<i16>, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: i16| -a, commu: {}, assoc: {});
impl_algebra!(Sum<i32>, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: i32| -a, commu: {}, assoc: {});
impl_algebra!(Sum<i64>, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: i64| -a, commu: {}, assoc: {});
impl_algebra!(Sum<i128>, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: i128| -a, commu: {}, assoc: {});
impl_algebra!(Sum<isize>, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: isize| -a, commu: {}, assoc: {});

impl_algebra!(Sum<u8>, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});
impl_algebra!(Sum<u16>, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});
impl_algebra!(Sum<u32>, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});
impl_algebra!(Sum<u64>, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});
impl_algebra!(Sum<u128>, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});
impl_algebra!(Sum<usize>, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});

impl_algebra!(Sum<f32>, op: |_, a, b| a + b, id: |_| 0.0, inv: |_, a: f32| -a, commu: {}, assoc: {});
impl_algebra!(Sum<f64>, op: |_, a, b| a + b, id: |_| 0.0, inv: |_, a: f64| -a, commu: {}, assoc: {});
