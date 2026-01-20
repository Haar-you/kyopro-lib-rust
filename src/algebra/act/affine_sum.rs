//! Range Affine Range Sum
use crate::algebra::affine::Composition;
use crate::algebra::dual::Dual;
pub use crate::algebra::{act::Act, traits::*};
use crate::math::linear::Linear;

use std::fmt::Debug;
use std::ops::Mul;

/// Range Affine Range Sum 用のモノイド作用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AffineSum<T>(pub Dual<Composition<T>>);

impl<T, M> Act<M> for AffineSum<T>
where
    M: Monoid<Element = T> + Additive,
    Dual<Composition<T>>: Monoid<Element = Linear<T>>,
    T: Mul<Output = T> + TryFrom<usize, Error: Debug>,
{
    type Monoid = Dual<Composition<T>>;
    type Element = Linear<T>;

    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, m: &M, val: M::Element, a: Self::Element) -> M::Element {
        let Linear { a, b } = a;
        m.op(a * val, b)
    }
    fn act_n(&self, m: &M, val: M::Element, a: Self::Element, len: usize) -> M::Element {
        let Linear { a, b } = a;
        m.op(a * val, b * T::try_from(len).unwrap())
    }
}
