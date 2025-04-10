//! Range Affine Range Sum用の代数的構造
use crate::algebra::action::Action;
use crate::algebra::{affine::*, sum::*};
use std::fmt::Debug;
use std::ops::{Add, Mul};

/// Range Affine Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct AffineSum<T, U = T> {
    fold_m: Sum<T>,
    update_m: Affine<U>,
}

impl<T, U> AffineSum<T, U> {
    /// `AffineSum<T, U>`を生成する。
    pub fn new() -> Self {
        Self {
            fold_m: Sum::new(),
            update_m: Affine::new(),
        }
    }
}

impl<T, U> Action for AffineSum<T, U>
where
    Sum<T>: Monoid<Element = T>,
    Affine<U>: Monoid<Element = (U, U)>,
    T: Add<Output = T> + Mul<Output = T> + TryFrom<U, Error: Debug>,
    U: Mul<Output = U> + TryFrom<usize, Error: Debug>,
{
    type FoldMonoid = Sum<T>;
    type UpdateMonoid = Affine<U>;
    type Output = <Self::FoldMonoid as Set>::Element;
    type Lazy = <Self::UpdateMonoid as Set>::Element;

    fn fold_monoid(&self) -> &Self::FoldMonoid {
        &self.fold_m
    }
    fn update_monoid(&self) -> &Self::UpdateMonoid {
        &self.update_m
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        let len = U::try_from(len).unwrap();
        T::try_from(lazy.0).unwrap() * value + T::try_from(lazy.1 * len).unwrap()
    }
}
