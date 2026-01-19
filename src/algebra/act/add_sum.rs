//! Range Add Range Sum
pub use crate::algebra::{act::Act, traits::*};

use std::fmt::Debug;
use std::ops::Mul;

/// Range Add Range Sum 用のモノイド作用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AddSum<S: Monoid + Additive>(pub S);

impl<M, A> Act<M> for AddSum<A>
where
    M: Monoid + Additive,
    A: Monoid + Additive,
    M::Element: TryFrom<A::Element, Error: Debug>,
    A::Element: Mul<Output = A::Element> + TryFrom<usize, Error: Debug>,
{
    type Monoid = A;
    type Element = A::Element;

    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, m: &M, val: M::Element, a: Self::Element) -> M::Element {
        m.op(val, M::Element::try_from(a).unwrap())
    }
    fn act_n(&self, m: &M, val: M::Element, a: Self::Element, len: usize) -> M::Element {
        m.op(
            val,
            M::Element::try_from(a * A::Element::try_from(len).unwrap()).unwrap(),
        )
    }
}
