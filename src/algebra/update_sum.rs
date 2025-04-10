//! Range Update Range Sum用の代数的構造
use crate::algebra::action::Action;
use crate::algebra::{sum::*, update::*};
use std::fmt::Debug;
use std::ops::Mul;

/// Range Update Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct UpdateSum<T> {
    fold_m: Sum<T>,
    update_m: Update<T>,
}

impl<T> UpdateSum<T> {
    /// `UpdateSum<T,U>`を生成する。
    pub fn new() -> Self {
        Self {
            fold_m: Sum::new(),
            update_m: Update::new(),
        }
    }
}

impl<T> Action for UpdateSum<T>
where
    Sum<T>: Monoid<Element = T>,
    T: Mul<Output = T> + TryFrom<usize, Error: Debug>,
{
    type FoldMonoid = Sum<T>;
    type UpdateMonoid = Update<T>;
    type Output = <Self::FoldMonoid as Set>::Element;
    type Lazy = <Self::UpdateMonoid as Set>::Element;

    fn fold_monoid(&self) -> &Self::FoldMonoid {
        &self.fold_m
    }
    fn update_monoid(&self) -> &Self::UpdateMonoid {
        &self.update_m
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        match lazy {
            Some(lazy) => lazy * T::try_from(len).unwrap(),
            _ => value,
        }
    }
}
