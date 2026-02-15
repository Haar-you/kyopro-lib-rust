//! 行列の代数的構造

pub use crate::algebra::traits::*;
use crate::{algebra::semiring::*, impl_algebra, linalg::matrix::MatrixOnSemiring};

/// 行列の加法
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SumMatrix<R> {
    /// 行列の要素の環
    pub semiring: R,
    /// 行数
    pub h: usize,
    /// 列数
    pub w: usize,
}
impl<R> SumMatrix<R> {
    /// [`SumMatrix`]を返す。
    pub fn new(semiring: R, h: usize, w: usize) -> Self {
        Self { semiring, h, w }
    }
}
impl<R: Semiring + Clone + PartialEq> Additive for SumMatrix<R>
where
    R::Element: Copy + PartialEq,
{
    fn times(&self, a: Self::Element, n: u64) -> Self::Element {
        a.times(n)
    }
}
impl_algebra!(
    {R: Semiring + Clone + PartialEq} SumMatrix<R> where {R::Element: Copy + PartialEq};
    set: MatrixOnSemiring<R>;
    op: |_, a: Self::Element, b: Self::Element| a + b;
    id: |s: &Self| MatrixOnSemiring::zero(s.semiring.clone(), s.h, s.w);
    assoc;
    commu;
);
impl_algebra!(
    {R: Ring + Clone + PartialEq} SumMatrix<R> where {R::Element: Copy + PartialEq};
    inv: |_, a: Self::Element| -a;
);

/// 行列の乗法
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProdMatrix<R> {
    /// 行列の要素の環
    pub semiring: R,
    /// 行列の大きさ
    pub n: usize,
}
impl<R> ProdMatrix<R> {
    /// [`ProdMatrix`]を返す。
    pub fn new(semiring: R, n: usize) -> Self {
        Self { semiring, n }
    }
}
impl<R: Semiring + Clone + PartialEq> Multiplicative for ProdMatrix<R> where
    R::Element: Copy + PartialEq
{
}
impl_algebra!(
    {R: Semiring + Clone + PartialEq} ProdMatrix<R> where {R::Element: Copy + PartialEq};
    set: MatrixOnSemiring<R>;
    op: |_, a: Self::Element, b: Self::Element| a * b;
    id: |s: &Self| MatrixOnSemiring::unit(s.semiring.clone(), s.n);
    assoc;
);
