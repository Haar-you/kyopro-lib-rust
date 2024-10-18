pub use crate::algebra::traits::*;
pub use crate::num::one_zero::*;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Affine<T>(PhantomData<T>);

impl<T> Affine<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Set for Affine<T> {
    type Element = (T, T);
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> BinaryOp for Affine<T> {
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        (a.0 * b.0, a.0 * b.1 + a.1)
    }
}

impl<T: One + Zero + Copy> Identity for Affine<T> {
    fn id(&self) -> Self::Element {
        (T::one(), T::zero())
    }
}

impl<T> Associative for Affine<T> {}
