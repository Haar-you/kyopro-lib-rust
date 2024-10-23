//! Range Add Range Sum用の代数的構造
use crate::algebra::action::Action;
use crate::num::one_zero::*;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

/// Range Add Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct AddSum<T, U = T>(PhantomData<(T, U)>);

impl<T, U> AddSum<T, U> {
    /// `AddSum<T, U>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T, U> Action for AddSum<T, U>
where
    T: Add<Output = T> + Zero + From<U>,
    U: Add<Output = U> + Mul<Output = U> + Zero + From<u64>,
{
    type Output = T;
    type Lazy = U;
    fn fold_id(&self) -> Self::Output {
        T::zero()
    }
    fn fold(&self, left: Self::Output, right: Self::Output) -> Self::Output {
        left + right
    }
    fn update_id(&self) -> Self::Lazy {
        U::zero()
    }
    fn update(&self, next: Self::Lazy, cur: Self::Lazy) -> Self::Lazy {
        next + cur
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        value + T::from(lazy * U::from(len as u64))
    }
}
