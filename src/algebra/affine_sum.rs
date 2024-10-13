use crate::algebra::action::Action;
use crate::num::one_zero::*;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

/// Range Affine Range Sum用の代数構造
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
    type FType = T;
    type UType = (T, T);
    fn fold_id(&self) -> Self::FType {
        T::zero()
    }
    fn fold(&self, x: Self::FType, y: Self::FType) -> Self::FType {
        x + y
    }
    fn update_id(&self) -> Self::UType {
        (T::one(), T::zero())
    }
    fn update(&self, a: Self::UType, b: Self::UType) -> Self::UType {
        (a.0 * b.0, a.0 * b.1 + a.1)
    }
    fn convert(&self, a: Self::FType, b: Self::UType, l: usize) -> Self::FType {
        b.0 * a + b.1 * T::from(l)
    }
}
