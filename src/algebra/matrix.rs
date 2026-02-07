//! 行列の代数的構造

pub use crate::algebra::traits::*;
use crate::{algebra::semiring::Ring, impl_algebra, linalg::matrix::MatrixOnRing};

/// 行列の加法
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SumMatrix<R> {
    /// 行列の要素の環
    pub ring: R,
    /// 行数
    pub h: usize,
    /// 列数
    pub w: usize,
}
impl<R> SumMatrix<R> {
    /// [`SumMatrix`]を返す。
    pub fn new(ring: R, h: usize, w: usize) -> Self {
        Self { ring, h, w }
    }
}
impl<R: Ring + Clone + PartialEq> Additive for SumMatrix<R>
where
    R::Element: Copy + PartialEq,
{
    fn times(&self, a: Self::Element, n: u64) -> Self::Element {
        a.times(n)
    }
}
impl_algebra!(
    {R: Ring + Clone + PartialEq} SumMatrix<R> where {R::Element: Copy + PartialEq};
    set: MatrixOnRing<R>;
    op: |_, a: Self::Element, b: Self::Element| a + b;
    id: |s: &Self| MatrixOnRing::zero(s.ring.clone(), s.h, s.w);
    inv: |_, a: Self::Element| -a;
    assoc;
    commu;
);

/// 行列の乗法
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProdMatrix<R> {
    /// 行列の要素の環
    pub ring: R,
    /// 行列の大きさ
    pub n: usize,
}
impl<R> ProdMatrix<R> {
    /// [`ProdMatrix`]を返す。
    pub fn new(ring: R, n: usize) -> Self {
        Self { ring, n }
    }
}
impl<R: Ring + Clone + PartialEq> Multiplicative for ProdMatrix<R> where R::Element: Copy + PartialEq
{}
impl_algebra!(
    {R: Ring + Clone + PartialEq} ProdMatrix<R> where {R::Element: Copy + PartialEq};
    set: MatrixOnRing<R>;
    op: |_, a: Self::Element, b: Self::Element| a * b;
    id: |s: &Self| MatrixOnRing::unit(s.ring.clone(), s.n);
    assoc;
);
