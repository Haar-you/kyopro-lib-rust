//! Range Add Range Min-Count
pub use crate::algebra::{act::Act, min_count::MinCount, sum::Sum, traits::*};
use std::ops::Add;

/// Range Add Range Min-Count用のモノイド作用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AddMinCount<S: Monoid + Additive>(pub S);

impl<T, U, A> Act<MinCount<T, U>> for AddMinCount<A>
where
    A: Monoid<Element = T> + Additive,
    MinCount<T, U>: Monoid<Element = (Option<T>, U)>,
    T: Add<Output = T>,
{
    type Monoid = A;
    type Element = A::Element;

    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, _m: &MinCount<T, U>, val: (Option<T>, U), a: Self::Element) -> (Option<T>, U) {
        (val.0.map(|x| self.0.op(x, a)), val.1)
    }
    fn act_n(
        &self,
        _m: &MinCount<T, U>,
        val: (Option<T>, U),
        a: Self::Element,
        _len: usize,
    ) -> (Option<T>, U) {
        (val.0.map(|x| self.0.op(x, a)), val.1)
    }
}
