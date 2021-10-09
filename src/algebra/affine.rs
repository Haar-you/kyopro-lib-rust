pub use crate::algebra::traits::*;
pub use crate::algebra::one_zero::*;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

#[derive(Clone, Default)]
pub struct Affine<T>(PhantomData<T>);

impl<T> Affine<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Affine<T> {
    type Output = (T, T);
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> BinaryOp for Affine<T> {
    fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        (a.0 * b.0, a.0 * b.1 + a.1)
    }
}

impl<T: One<Output = T> + Zero<Output = T> + Copy> Identity for Affine<T> {
    fn id(&self) -> Self::Output {
        (T::one(), T::zero())
    }
}
