//! Range Update Range Sum用の代数的構造
use crate::algebra::action::Action;
use crate::algebra::{first_last::*, sum::*};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Mul;

/// Range Update Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct UpdateSum<T>(PhantomData<T>);

impl<T> Action for UpdateSum<T>
where
    Sum<T>: Monoid,
    Last<T>: Monoid,
    T: Mul<Output = T> + TryFrom<usize, Error: Debug>,
{
    type Output = Sum<T>;
    type Lazy = Last<T>;

    fn convert(value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        match lazy.0 {
            Some(lazy) => Sum(lazy * T::try_from(len).unwrap()),
            _ => value,
        }
    }
}
