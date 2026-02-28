//! 大きさがコンパイル時固定の行列
use std::ops::{Add, Mul, Neg, Sub};

use crate::algebra::semiring::*;
pub use crate::linalg::traits::*;

/// $R \times C$の行列
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstMatrix<S: Semiring, const R: usize, const C: usize> {
    sr: S,
    data: [[S::Element; C]; R],
}

impl<S: Semiring, const R: usize, const C: usize> ConstMatrix<S, R, C>
where
    S::Element: Copy,
{
    /// ゼロ行列を返す。
    pub fn zero(semiring: S) -> Self {
        let data = [[semiring.zero(); C]; R];
        Self { sr: semiring, data }
    }
}

impl<S: Semiring, const N: usize> ConstMatrix<S, N, N>
where
    S::Element: Copy,
{
    /// 単位行列を返す。
    pub fn unit(semiring: S) -> Self {
        let mut data = [[semiring.zero(); N]; N];
        for (i, r) in data.iter_mut().enumerate() {
            r[i] = semiring.one();
        }
        Self { sr: semiring, data }
    }
}

impl<S: Semiring, const R: usize, const C: usize> Matrix for ConstMatrix<S, R, C> {
    fn width(&self) -> usize {
        C
    }
    fn height(&self) -> usize {
        R
    }
}

impl<S: Semiring, const R: usize, const C: usize> ConstMatrix<S, R, C> {
    /// `i`行`j`列の要素への参照を返す。
    pub fn get(&self, i: usize, j: usize) -> Option<&S::Element> {
        let a = self.data.get(i)?;
        a.get(j)
    }

    /// `i`行`j`列の要素への可変参照を返す。
    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut S::Element> {
        let a = self.data.get_mut(i)?;
        a.get_mut(j)
    }
}

impl<S: Semiring, const R: usize, const C: usize> MatrixTranspose for ConstMatrix<S, R, C>
where
    S::Element: Copy,
{
    type Output = ConstMatrix<S, C, R>;
    fn transpose(self) -> Self::Output {
        let mut ret = ConstMatrix::<S, C, R>::zero(self.sr);
        for i in 0..R {
            for j in 0..C {
                ret.data[j][i] = self.data[i][j];
            }
        }
        ret
    }
}

impl<S: Semiring, const R: usize, const C: usize> Add for ConstMatrix<S, R, C>
where
    S::Element: Copy,
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        for (a, b) in self.data.iter_mut().zip(other.data) {
            for (x, y) in a.iter_mut().zip(b) {
                *x = self.sr.add(*x, y);
            }
        }
        self
    }
}

impl<S: Ring, const R: usize, const C: usize> Sub for ConstMatrix<S, R, C>
where
    S::Element: Copy,
{
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        for (a, b) in self.data.iter_mut().zip(other.data) {
            for (x, y) in a.iter_mut().zip(b) {
                *x = self.sr.sub(*x, y);
            }
        }
        self
    }
}

impl<S: Ring, const R: usize, const C: usize> Neg for ConstMatrix<S, R, C>
where
    S::Element: Copy,
{
    type Output = Self;
    fn neg(mut self) -> Self {
        for a in self.data.iter_mut() {
            for x in a.iter_mut() {
                *x = self.sr.neg(*x);
            }
        }
        self
    }
}

impl<S: Semiring + Clone, const R: usize, const C: usize, const C2: usize>
    Mul<ConstMatrix<S, C, C2>> for ConstMatrix<S, R, C>
where
    S::Element: Copy,
{
    type Output = ConstMatrix<S, R, C2>;
    fn mul(self, other: ConstMatrix<S, C, C2>) -> Self::Output {
        let b = other.transpose();
        let mut ret = ConstMatrix::zero(self.sr.clone());

        for (r, r2) in ret.data.iter_mut().zip(self.data) {
            for (x, c) in r.iter_mut().zip(b.data.iter()) {
                for (y, z) in r2.iter().zip(c.iter()) {
                    *x = self.sr.add(*x, self.sr.mul(*y, *z));
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::semiring::add_mul::AddMul;

    use super::*;

    #[test]
    fn test() {
        let r = AddMul::<u32>::new();
        let a = ConstMatrix::<_, 5, 5>::unit(r);
        let b = ConstMatrix::<_, 5, 3>::zero(r);

        dbg!(a * b);
    }
}
