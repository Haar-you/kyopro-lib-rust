//! 大きさがコンパイル時固定の行列
use std::ops::{Add, Mul, Neg, Sub};

use crate::algebra::{prod::*, sum::*};
use crate::impl_algebra;
use crate::num::one_zero::{One, Zero};

/// `R`×`C`の行列
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T, const R: usize, const C: usize> {
    data: [[T; C]; R],
}

impl<T: Copy + Zero, const R: usize, const C: usize> Matrix<T, R, C> {
    /// ゼロ行列を返す。
    pub fn new() -> Self {
        let data = [[T::zero(); C]; R];
        Self { data }
    }
}

impl<T: Copy + Zero, const R: usize, const C: usize> Default for Matrix<T, R, C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Zero + One, const N: usize> Matrix<T, N, N> {
    /// 単位行列を返す。
    pub fn unit() -> Self {
        let mut data = [[T::zero(); N]; N];
        for (i, r) in data.iter_mut().enumerate() {
            r[i] = T::one();
        }
        Self { data }
    }
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    /// `i`行`j`列の要素への参照を返す。
    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        let a = self.data.get(i)?;
        a.get(j)
    }

    /// `i`行`j`列の要素への可変参照を返す。
    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        let a = self.data.get_mut(i)?;
        a.get_mut(j)
    }
}

impl<T: Copy + Zero, const R: usize, const C: usize> Matrix<T, R, C> {
    /// 転置行列を返す。
    pub fn transpose(self) -> Matrix<T, C, R> {
        let mut ret = Matrix::<T, C, R>::new();
        for i in 0..R {
            for j in 0..C {
                ret.data[j][i] = self.data[i][j];
            }
        }
        ret
    }
}

impl<T: Copy + Add<Output = T>, const R: usize, const C: usize> Add for Matrix<T, R, C> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        for (a, b) in self.data.iter_mut().zip(other.data) {
            for (x, y) in a.iter_mut().zip(b) {
                *x = *x + y;
            }
        }
        self
    }
}

impl<T: Copy + Sub<Output = T>, const R: usize, const C: usize> Sub for Matrix<T, R, C> {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        for (a, b) in self.data.iter_mut().zip(other.data) {
            for (x, y) in a.iter_mut().zip(b) {
                *x = *x - y;
            }
        }
        self
    }
}

impl<T: Copy + Neg<Output = T>, const R: usize, const C: usize> Neg for Matrix<T, R, C> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for a in self.data.iter_mut() {
            for x in a.iter_mut() {
                *x = -*x;
            }
        }
        self
    }
}

impl<T, const R: usize, const C: usize, const C2: usize> Mul<Matrix<T, C, C2>> for Matrix<T, R, C>
where
    T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T, R, C2>;
    fn mul(self, other: Matrix<T, C, C2>) -> Self::Output {
        let b = other.transpose();
        let mut ret = Matrix::new();

        for (r, r2) in ret.data.iter_mut().zip(self.data) {
            for (x, c) in r.iter_mut().zip(b.data.iter()) {
                for (y, z) in r2.iter().zip(c.iter()) {
                    *x = *x + *y * *z;
                }
            }
        }

        ret
    }
}

impl_algebra!(
    [T: Copy + One + Zero + Add<Output = T> + Mul<Output = T>, const N: usize];
    Prod<Matrix<T, N, N>>;
    op: |a: Self, b: Self| Self(a.0 * b.0);
    id: Self(Matrix::unit());
    assoc;
);

impl_algebra!(
    [T: Copy + Zero + Add<Output = T> + Neg<Output = T>, const R: usize, const C: usize];
    Sum<Matrix<T, R, C>>;
    op: |a: Self, b: Self| Self(a.0 + b.0);
    id: Self(Matrix::new());
    inv: |a: Self| Self(-a.0);
    assoc;
    commu;
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = Matrix::<u32, 5, 5>::unit();
        let b = Matrix::<u32, 5, 3>::new();

        dbg!(a * b);
    }
}
