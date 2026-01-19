//! Range Add Range Sum
pub use crate::algebra::{action::Action, traits::*};
use std::fmt::Debug;
use std::ops::{Add, Mul};

/// Range Add Range Sum用の代数的構造
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddSum<SumOutput, SumLazy = SumOutput>(pub SumOutput, pub SumLazy);

impl<S1, S2> Action for AddSum<S1, S2>
where
    S1: Monoid,
    S2: Monoid,
    S1::Element: Add<Output = S1::Element> + TryFrom<S2::Element, Error: Debug>,
    S2::Element: Mul<Output = S2::Element> + TryFrom<usize, Error: Debug>,
{
    type Output = S1::Element;
    type Lazy = S2::Element;
    type MonoidOutput = S1;
    type MonoidLazy = S2;

    fn monoid_output(&self) -> &Self::MonoidOutput {
        &self.0
    }
    fn monoid_lazy(&self) -> &Self::MonoidLazy {
        &self.1
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        value + S1::Element::try_from(lazy * S2::Element::try_from(len).unwrap()).unwrap()
    }
}
