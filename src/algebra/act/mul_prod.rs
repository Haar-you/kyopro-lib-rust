//! Range Multiply Range Product
pub use crate::algebra::{act::Act, traits::*};

/// Range Multiply Range Product 用のモノイド作用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MulProd<S: Monoid + Multiplicative>(pub S);

impl<S> Act<S> for MulProd<S>
where
    S: Monoid + Multiplicative,
    S::Element: Clone,
{
    type Monoid = S;
    type Element = S::Element;

    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, _m: &S, val: S::Element, a: Self::Element, n: usize) -> S::Element {
        self.0.op(val, self.0.times(a, n as u64))
    }
}
