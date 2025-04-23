//! Range Add Range Sum用の代数的構造
use crate::algebra::action::Action;
use crate::algebra::sum::*;
use std::fmt::Debug;
use std::ops::{Add, Mul};

/// Range Add Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct AddSum<T, U = T> {
    fold_m: Sum<T>,
    update_m: Sum<U>,
}

impl<T, U> AddSum<T, U> {
    /// `AddSum<T, U>`を生成する。
    pub fn new() -> Self {
        Self {
            fold_m: Sum::new(),
            update_m: Sum::new(),
        }
    }
}

impl<T, U> Action for AddSum<T, U>
where
    Sum<T>: Monoid<Element = T>,
    Sum<U>: Monoid<Element = U>,
    T: Add<Output = T> + TryFrom<U, Error: Debug>,
    U: Mul<Output = U> + TryFrom<usize, Error: Debug>,
{
    type FoldMonoid = Sum<T>;
    type UpdateMonoid = Sum<U>;
    type Output = <Self::FoldMonoid as Set>::Element;
    type Lazy = <Self::UpdateMonoid as Set>::Element;

    fn fold_monoid(&self) -> &Self::FoldMonoid {
        &self.fold_m
    }
    fn update_monoid(&self) -> &Self::UpdateMonoid {
        &self.update_m
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        value + T::try_from(lazy * U::try_from(len).unwrap()).unwrap()
    }
}
