pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Prod<T>(PhantomData<T>);
impl<T> Prod<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(Prod<i8>, set: i8, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<i16>, set: i16, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<i32>, set: i32, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<i64>, set: i64, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<i128>, set: i128, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<isize>, set: isize, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});

impl_algebra!(Prod<u8>, set: u8, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<u16>, set: u16, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<u32>, set: u32, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<u64>, set: u64, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<u128>, set: u128, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});
impl_algebra!(Prod<usize>, set: usize, op: |_, a, b| a * b, id: |_| 1, commu: {}, assoc: {});

impl_algebra!(Prod<f32>, set: f32, op: |_, a, b| a * b, id: |_| 1.0, commu: {}, assoc: {});
impl_algebra!(Prod<f64>, set: f64, op: |_, a, b| a * b, id: |_| 1.0, commu: {}, assoc: {});
