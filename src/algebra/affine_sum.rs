//! Range Affine Range Sum
pub use crate::algebra::{action::Action, traits::*};
use crate::algebra::{affine::Composition, dual::Dual};
use crate::math::linear::Linear;

use std::fmt::Debug;
use std::ops::{Add, Mul};

/// Range Affine Range Sum用の代数的構造
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct AffineSum<S, U>(S, Dual<Composition<U>>);
impl<S, U> AffineSum<S, U> {
    /// 加算のモノイドと[`Composition`]から[`AffineSum`]を生成する。
    pub fn new(sum: S, affine: Composition<U>) -> Self {
        Self(sum, Dual(affine))
    }
}

impl<S, U> Action for AffineSum<S, U>
where
    S: Monoid,
    Composition<U>: Monoid<Element = Linear<U>>,
    S::Element: Add<Output = S::Element> + Mul<Output = S::Element> + TryFrom<U, Error: Debug>,
    U: Mul<Output = U> + TryFrom<usize, Error: Debug>,
{
    type Output = S::Element;
    type Lazy = Linear<U>;
    type MonoidOutput = S;
    type MonoidLazy = Dual<Composition<U>>;

    fn monoid_output(&self) -> &Self::MonoidOutput {
        &self.0
    }
    fn monoid_lazy(&self) -> &Self::MonoidLazy {
        &self.1
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        let len = U::try_from(len).unwrap();
        S::Element::try_from(lazy.a).unwrap() * value + S::Element::try_from(lazy.b * len).unwrap()
    }
}
