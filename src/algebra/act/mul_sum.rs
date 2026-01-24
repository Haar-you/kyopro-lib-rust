//! Range Multiply Range Sum
pub use crate::algebra::{act::Act, traits::*};

/// Range Multiply Range Sum 用のモノイド作用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MulSum<S: Monoid + Multiplicative>(pub S);

impl<T, M, S> Act<M> for MulSum<S>
where
    M: Monoid<Element = T> + Additive,
    S: Monoid<Element = T> + Multiplicative,
{
    type Monoid = S;
    type Element = S::Element;

    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, _m: &M, val: M::Element, a: Self::Element, _n: usize) -> M::Element {
        self.0.op(val, a)
    }
}
