//! 半環上の行列
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc009/tasks/abc009_4>
use crate::algebra::semiring::Semiring;
use crate::impl_ops;
pub use crate::linalg::traits::*;
use std::ops::Index;

/// 半環上の行列
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemiringMatrix<T> {
    data: Vec<Vec<T>>,
    h: usize,
    w: usize,
}

impl<T: Semiring + Copy> Matrix for SemiringMatrix<T> {
    fn width(&self) -> usize {
        self.w
    }
    fn height(&self) -> usize {
        self.h
    }
}

impl<T: Semiring + Copy> MatrixTranspose for SemiringMatrix<T> {
    type Output = Self;
    fn transpose(self) -> Self::Output {
        let a = self;
        let mut ret = Self::zero(a.w, a.h);
        for i in 0..a.h {
            for j in 0..a.w {
                ret.data[j][i] = a.data[i][j];
            }
        }
        ret
    }
}

impl<T: Semiring + Copy> SemiringMatrix<T> {
    /// `h`×`w`の零行列を返す。
    pub fn zero(h: usize, w: usize) -> Self {
        let data = vec![vec![T::zero(); w]; h];
        Self { data, h, w }
    }

    /// `n`×`n`の単位行列を返す。
    pub fn unit(n: usize) -> Self {
        let mut this = Self::zero(n, n);
        for i in 0..n {
            this.data[i][i] = T::one();
        }
        this
    }

    /// 行列`a`の`n`乗を求める。
    pub fn pow(self, mut n: u64) -> Option<Self> {
        if !self.is_square() {
            None
        } else {
            let mut a = self;

            let mut ret = Self::unit(a.h);

            while n > 0 {
                if n % 2 == 1 {
                    ret = ret.try_mul(a.clone()).unwrap();
                }
                a = a.clone().try_mul(a).unwrap();
                n >>= 1;
            }

            Some(ret)
        }
    }

    /// `i`行`j`列の要素への可変参照を返す。
    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        let a = self.data.get_mut(i)?;
        a.get_mut(j)
    }

    /// `i`行`j`列の要素への参照を返す。
    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        let a = self.data.get(i)?;
        a.get(j)
    }
}

impl<T: Semiring + Copy> TryAdd for SemiringMatrix<T> {
    type Output = Self;
    fn try_add(mut self, rhs: Self) -> Option<Self::Output> {
        if self.h != rhs.h || self.w != rhs.h {
            None
        } else {
            for i in 0..self.h {
                for j in 0..self.w {
                    self.data[i][j] = T::add(self.data[i][j], rhs.data[i][j]);
                }
            }
            Some(self)
        }
    }
}

impl<T: Semiring + Copy> TryMul for SemiringMatrix<T> {
    type Output = Self;
    fn try_mul(self, rhs: Self) -> Option<Self::Output> {
        let a = self;
        let b = rhs;
        if a.w != b.h {
            return None;
        }

        let n = a.h;
        let l = b.w;
        let b = b.transpose();
        let mut ret = Self::zero(n, l);

        for (r, r2) in ret.data.iter_mut().zip(a.data.iter()) {
            for (x, c) in r.iter_mut().zip(b.data.iter()) {
                for (y, z) in r2.iter().zip(c.iter()) {
                    *x = T::add(*x, T::mul(*y, *z));
                }
            }
        }

        Some(ret)
    }
}

impl_ops!([T: Semiring + Copy]; AddAssign for SemiringMatrix<T>, |x: &mut Self, y: Self| *x = x.clone().try_add(y).unwrap());
impl_ops!([T: Semiring + Copy]; MulAssign for SemiringMatrix<T>, |x: &mut Self, y: Self| *x = x.clone().try_mul(y).unwrap());

impl_ops!([T: Semiring + Copy]; Add for SemiringMatrix<T>, |x: Self, y| x.try_add(y).unwrap());
impl_ops!([T: Semiring + Copy]; Mul for SemiringMatrix<T>, |x: Self, y| x.try_mul(y).unwrap());

impl<T> Index<usize> for SemiringMatrix<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
