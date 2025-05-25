//! 一次関数の合成を演算とする代数的構造
pub use crate::algebra::traits::*;
pub use crate::num::one_zero::*;
use std::ops::{Add, Mul};

/// 一次関数の合成を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Affine<T>(pub T, pub T);

impl<T> Set for Affine<T> {}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> BinaryOp for Affine<T> {
    fn op(self, b: Self) -> Self {
        Self(self.0 * b.0, self.0 * b.1 + self.1)
    }
}

impl<T: One + Zero + Copy> Identity for Affine<T> {
    fn id() -> Self {
        Self(T::one(), T::zero())
    }
}

impl<T> Associative for Affine<T> {}
