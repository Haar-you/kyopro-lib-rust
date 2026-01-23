//! Range Affine Range Sum
use crate::algebra::affine::Composition;
use crate::algebra::dual::Dual;
use crate::algebra::semiring::Semiring;
pub use crate::algebra::{act::Act, traits::*};
use crate::math::linear::Linear;

/// Range Affine Range Sum 用のモノイド作用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AffineSum<S: Semiring>(pub Dual<Composition<S>>);

impl<T, S, M> Act<M> for AffineSum<S>
where
    M: Monoid<Element = T> + Additive,
    S: Semiring<Element = T>,
    Dual<Composition<S>>: Monoid<Element = Linear<T>>,
{
    type Monoid = Dual<Composition<S>>;
    type Element = Linear<T>;

    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, _m: &M, val: M::Element, a: Self::Element) -> M::Element {
        let Self(Dual(Composition(ref s))) = &self;
        let Linear { a, b } = a;
        s.add(s.mul(a, val), b)
    }
    fn act_n(&self, m: &M, val: M::Element, a: Self::Element, len: usize) -> M::Element {
        let Self(Dual(Composition(ref s))) = &self;
        let Linear { a, b } = a;
        s.add(s.mul(a, val), Additive::times(m, b, len as u64))
    }
}
