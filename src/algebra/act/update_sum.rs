//! Range Update Range Sum
pub use crate::algebra::{act::Act, first_last::Last, traits::*};
use std::fmt::Debug;
use std::ops::Mul;

/// Range Update Range Sum 用のモノイド作用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UpdateSum<T>(pub Last<T>);

impl<T, M> Act<M> for UpdateSum<T>
where
    M: Monoid<Element = T> + Additive,
    T: Mul<Output = T> + TryFrom<usize, Error: Debug>,
{
    type Monoid = Last<T>;
    type Element = Option<T>;

    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, _m: &M, val: <M>::Element, a: Self::Element) -> <M>::Element {
        match a {
            Some(a) => a,
            _ => val,
        }
    }
    fn act_n(&self, _m: &M, val: <M>::Element, a: Self::Element, len: usize) -> <M>::Element {
        match a {
            Some(a) => a * T::try_from(len).unwrap(),
            _ => val,
        }
    }
}
