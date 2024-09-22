//! 乗算を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 乗算を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Prod<T>(PhantomData<T>);
impl<T> Prod<T> {
    /// `Prod<T>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Prod<T> {
    type Output = T;
}

impl_algebra!(Prod<i8>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<i16>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<i32>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<i64>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<i128>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<isize>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});

impl_algebra!(Prod<u8>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<u16>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<u32>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<u64>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<u128>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<usize>, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});

impl_algebra!(Prod<f32>, op: |_, a, b| a * b, id: |_| 1.0, commu: {}, assoc: {});
impl_algebra!(Prod<f64>, op: |_, a, b| a * b, id: |_| 1.0, commu: {}, assoc: {});
