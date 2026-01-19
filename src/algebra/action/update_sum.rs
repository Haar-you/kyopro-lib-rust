//! Range Update Range Sum
pub use crate::algebra::{action::Action, first_last::Last, sum::Sum, traits::*};
use std::fmt::Debug;
use std::ops::Mul;

/// Range Update Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpdateSum<S: Monoid>(S, Last<S::Element>);
impl<S: Monoid> UpdateSum<S> {
    /// [`UpdateSum`]を返す。
    pub fn new(sum: S) -> Self {
        Self(sum, Last::new())
    }
}

impl<S> Action for UpdateSum<S>
where
    S: Monoid,
    Last<S::Element>: Monoid<Element = Option<S::Element>>,
    S::Element: Mul<Output = S::Element> + TryFrom<usize, Error: Debug>,
{
    type Output = S::Element;
    type Lazy = Option<S::Element>;
    type MonoidOutput = S;
    type MonoidLazy = Last<S::Element>;

    fn monoid_output(&self) -> &Self::MonoidOutput {
        &self.0
    }
    fn monoid_lazy(&self) -> &Self::MonoidLazy {
        &self.1
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        match lazy {
            Some(lazy) => lazy * S::Element::try_from(len).unwrap(),
            _ => value,
        }
    }
}
