//! 最大値を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 最大値を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Max<T>(PhantomData<T>);
impl<T> Max<T> {
    /// `Max<T>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Max<T> {
    type Output = T;
}

impl_algebra!(Max<i8>, op: |_, a: i8, b| a.max(b), id: |_| i8::MIN, commu: {}, assoc: {}, idem: {});
impl_algebra!(Max<i16>, op: |_, a: i16, b| a.max(b), id: |_| i16::MIN, commu: {}, assoc: {}, idem: {});
impl_algebra!(Max<i32>, op: |_, a: i32, b| a.max(b), id: |_| i32::MIN, commu: {}, assoc: {}, idem: {});
impl_algebra!(Max<i64>, op: |_, a: i64, b| a.max(b), id: |_| i64::MIN, commu: {}, assoc: {}, idem: {});
impl_algebra!(Max<i128>, op: |_, a: i128, b| a.max(b), id: |_| i128::MIN, commu: {}, assoc: {}, idem: {});
impl_algebra!(Max<isize>, op: |_, a: isize, b| a.max(b), id: |_| isize::MIN, commu: {}, assoc: {}, idem: {});

impl_algebra!(Max<u8>, op: |_, a: u8, b| a.max(b), id: |_| u8::MIN, commu: {}, assoc: {}, idem: {});
impl_algebra!(Max<u16>, op: |_, a: u16, b| a.max(b), id: |_| u16::MIN, commu: {}, assoc: {}, idem: {});
impl_algebra!(Max<u32>, op: |_, a: u32, b| a.max(b), id: |_| u32::MIN, commu: {}, assoc: {}, idem: {});
impl_algebra!(Max<u64>, op: |_, a: u64, b| a.max(b), id: |_| u64::MIN, commu: {}, assoc: {}, idem: {});
impl_algebra!(Max<u128>, op: |_, a: u128, b| a.max(b), id: |_| u128::MIN, commu: {}, assoc: {}, idem: {});
impl_algebra!(Max<usize>, op: |_, a: usize, b| a.max(b), id: |_| usize::MIN, commu: {}, assoc: {}, idem: {});
