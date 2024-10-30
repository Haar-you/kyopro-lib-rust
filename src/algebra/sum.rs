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

impl_algebra!(Sum<i8>, set: i8, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: i8| -a, commu: {}, assoc: {});
impl_algebra!(Sum<i16>, set: i16, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: i16| -a, commu: {}, assoc: {});
impl_algebra!(Sum<i32>, set: i32, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: i32| -a, commu: {}, assoc: {});
impl_algebra!(Sum<i64>, set: i64, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: i64| -a, commu: {}, assoc: {});
impl_algebra!(Sum<i128>, set: i128, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: i128| -a, commu: {}, assoc: {});
impl_algebra!(Sum<isize>, set: isize, op: |_, a, b| a + b, id: |_| 0, inv: |_, a: isize| -a, commu: {}, assoc: {});

impl_algebra!(Sum<u8>, set: u8, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});
impl_algebra!(Sum<u16>, set: u16, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});
impl_algebra!(Sum<u32>, set: u32, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});
impl_algebra!(Sum<u64>, set: u64, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});
impl_algebra!(Sum<u128>, set: u128, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});
impl_algebra!(Sum<usize>, set: usize, op: |_, a, b| a + b, id: |_| 0, commu: {}, assoc: {});

impl_algebra!(Sum<f32>, set: f32, op: |_, a, b| a + b, id: |_| 0.0, inv: |_, a: f32| -a, commu: {}, assoc: {});
impl_algebra!(Sum<f64>, set: f64, op: |_, a, b| a + b, id: |_| 0.0, inv: |_, a: f64| -a, commu: {}, assoc: {});
