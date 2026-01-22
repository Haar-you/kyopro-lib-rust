//! 行列の代数的構造

pub use crate::algebra::traits::*;
use crate::{algebra::semiring::Ring, impl_algebra, linalg::matrix::MatrixOnRing};

/// 行列の加法
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SumMatrix<R>(R, usize, usize);
impl<R> SumMatrix<R> {
    /// [`SumMatrix`]を返す。
    pub fn new(ring: R, h: usize, w: usize) -> Self {
        Self(ring, h, w)
    }
}

impl_algebra!(
    {R: Ring + Clone + PartialEq} SumMatrix<R> where {R::Element: Copy + PartialEq};
    set: MatrixOnRing<R>;
    op: |_, a: Self::Element, b: Self::Element| a + b;
    id: |s: &Self| MatrixOnRing::zero(s.0.clone(), s.1, s.2);
    inv: |_, a: Self::Element| -a;
    assoc;
    commu;
);

/// 行列の乗法
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProdMatrix<R>(R, usize);
impl<R> ProdMatrix<R> {
    /// [`ProdMatrix`]を返す。
    pub fn new(ring: R, n: usize) -> Self {
        Self(ring, n)
    }
}

impl_algebra!(
    {R: Ring + Clone + PartialEq} ProdMatrix<R> where {R::Element: Copy + PartialEq};
    set: MatrixOnRing<R>;
    op: |_, a: Self::Element, b: Self::Element| a * b;
    id: |s: &Self| MatrixOnRing::unit(s.0.clone(), s.1);
    assoc;
);
