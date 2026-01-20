//! 一次関数の合成
pub use crate::algebra::traits::*;
use crate::math::linear::Linear;
pub use crate::num::one_zero::*;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

/// [`Linear`]の合成
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Composition<T>(PhantomData<T>);
impl<T> Composition<T> {
    /// [`Composition`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Set for Composition<T> {
    type Element = Linear<T>;
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> BinaryOp for Composition<T> {
    fn op(&self, f: Self::Element, g: Self::Element) -> Self::Element {
        Linear::new(f.a * g.a, f.a * g.b + f.b)
    }
}

impl<T: One + Zero + Copy + PartialEq> Identity for Composition<T> {
    fn id(&self) -> Self::Element {
        Linear::new(T::one(), T::zero())
    }
    fn is_id(&self, a: &Self::Element) -> bool {
        a == &self.id()
    }
}

impl<T> Associative for Composition<T> {}
