//! Range Affine Range Sum用の代数的構造
use crate::algebra::action::Action;
use crate::num::one_zero::*;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

/// Range Affine Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct AffineSum<T>(PhantomData<T>);

impl<T> AffineSum<T> {
    /// `AffineSum<T>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Action for AffineSum<T>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + One + Copy + From<usize>,
{
    type Output = T;
    type Lazy = (T, T);
    fn fold_id(&self) -> Self::Output {
        T::zero()
    }
    fn fold(&self, left: Self::Output, right: Self::Output) -> Self::Output {
        left + right
    }
    fn update_id(&self) -> Self::Lazy {
        (T::one(), T::zero())
    }
    fn update(&self, next: Self::Lazy, cur: Self::Lazy) -> Self::Lazy {
        (next.0 * cur.0, next.0 * cur.1 + next.1)
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        lazy.0 * value + lazy.1 * T::from(len)
    }
}
