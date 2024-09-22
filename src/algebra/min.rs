//! 最小値を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 最小値を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Min<T>(PhantomData<T>);
impl<T> Min<T> {
    /// `Min<T>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Min<T> {
    type Output = T;
}

impl_algebra!(Min<i8>, op: |_, a: i8, b| a.min(b), id: |_| i8::MAX, commu: {}, assoc: {}, idem: {});
impl_algebra!(Min<i16>, op: |_, a: i16, b| a.min(b), id: |_| i16::MAX, commu: {}, assoc: {}, idem: {});
impl_algebra!(Min<i32>, op: |_, a: i32, b| a.min(b), id: |_| i32::MAX, commu: {}, assoc: {}, idem: {});
impl_algebra!(Min<i64>, op: |_, a: i64, b| a.min(b), id: |_| i64::MAX, commu: {}, assoc: {}, idem: {});
impl_algebra!(Min<i128>, op: |_, a: i128, b| a.min(b), id: |_| i128::MAX, commu: {}, assoc: {}, idem: {});
impl_algebra!(Min<isize>, op: |_, a: isize, b| a.min(b), id: |_| isize::MAX, commu: {}, assoc: {}, idem: {});

impl_algebra!(Min<u8>, op: |_, a: u8, b| a.min(b), id: |_| u8::MAX, commu: {}, assoc: {}, idem: {});
impl_algebra!(Min<u16>, op: |_, a: u16, b| a.min(b), id: |_| u16::MAX, commu: {}, assoc: {}, idem: {});
impl_algebra!(Min<u32>, op: |_, a: u32, b| a.min(b), id: |_| u32::MAX, commu: {}, assoc: {}, idem: {});
impl_algebra!(Min<u64>, op: |_, a: u64, b| a.min(b), id: |_| u64::MAX, commu: {}, assoc: {}, idem: {});
impl_algebra!(Min<u128>, op: |_, a: u128, b| a.min(b), id: |_| u128::MAX, commu: {}, assoc: {}, idem: {});
impl_algebra!(Min<usize>, op: |_, a: usize, b| a.min(b), id: |_| usize::MAX, commu: {}, assoc: {}, idem: {});
