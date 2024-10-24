//! Range Affine Range Sum用の代数的構造
use crate::algebra::action::Action;
use crate::num::one_zero::*;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

/// Range Affine Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct AffineSum<T, U = T>(PhantomData<(T, U)>);

impl<T, U> AffineSum<T, U> {
    /// `AffineSum<T, U>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T, U> Action for AffineSum<T, U>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Copy + From<U>,
    U: Add<Output = U> + Mul<Output = U> + Zero + One + Copy + From<u64>,
{
    type Output = T;
    type Lazy = (U, U);
    fn fold_id(&self) -> Self::Output {
        T::zero()
    }
    fn fold(&self, left: Self::Output, right: Self::Output) -> Self::Output {
        left + right
    }
    fn update_id(&self) -> Self::Lazy {
        (U::one(), U::zero())
    }
    fn update(&self, next: Self::Lazy, cur: Self::Lazy) -> Self::Lazy {
        (next.0 * cur.0, next.0 * cur.1 + next.1)
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        T::from(lazy.0) * value + T::from(lazy.1 * U::from(len as u64))
    }
}
