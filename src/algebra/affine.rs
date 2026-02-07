//! 一次関数の合成
use crate::algebra::semiring::Semiring;
pub use crate::algebra::traits::*;
use crate::{impl_algebra, math::linear::Linear};

/// [`Linear`]の合成
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Composition<S: Semiring>(pub S);

impl_algebra!(
    {T: Copy, S: Semiring<Element = T>} Composition<S>;
    set: Linear<T>;
    op: |Self(ref s): &Self, f: Linear<T>, g: Linear<T>| Linear::new(s.mul(f.a, g.a), s.add(s.mul(f.a, g.b), f.b));
    id: |Self(ref s): &Self| Linear::new(s.one(), s.zero());
    assoc;
);
