pub use crate::algebra::one_zero::*;
use crate::trait_alias;
use std::ops::{Add, AddAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

trait_alias!(
    SquareMatrixElem,
    Copy
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + One<Output = Self>
        + Zero<Output = Self>
);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SquareMatrix<T> {
    size: usize,
    data: Vec<Vec<T>>,
}

impl<T: SquareMatrixElem> SquareMatrix<T> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: vec![vec![T::zero(); size]; size],
        }
    }

    pub fn unit(size: usize) -> Self {
        let mut ret = Self::new(size);
        for i in 0..size {
            ret.data[i][i] = T::one();
        }
        ret
    }

    pub fn from_vec(other: Vec<Vec<T>>) -> Self {
        let size = other.len();
        assert!(size > 0);
        assert!(other.iter().all(|r| r.len() == size));

        Self { size, data: other }
    }

    pub fn to_vec(&self) -> Vec<Vec<T>> {
        self.data.clone()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn transpose(self) -> Self {
        let mut ret = Self::new(self.size);
        for i in 0..self.size {
            for j in 0..self.size {
                ret.data[j][i] = self.data[i][j];
            }
        }
        ret
    }

    pub fn pow(self, mut p: u64) -> Self {
        let mut ret = Self::unit(self.size);
        let mut a = self;

        while p > 0 {
            if p & 1 != 0 {
                ret *= a.clone();
            }
            a *= a.clone();

            p >>= 1;
        }

        ret
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        self.data.get_mut(i).map(|a| a.get_mut(j)).flatten()
    }
}

impl<T: SquareMatrixElem> Add for SquareMatrix<T> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        assert_eq!(self.size, other.size);
        for (a, b) in self.data.iter_mut().zip(other.data.into_iter()) {
            for (x, y) in a.iter_mut().zip(b.into_iter()) {
                *x = *x + y;
            }
        }
        self
    }
}

impl<T: SquareMatrixElem> Sub for SquareMatrix<T> {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        assert_eq!(self.size, other.size);
        for (a, b) in self.data.iter_mut().zip(other.data.into_iter()) {
            for (x, y) in a.iter_mut().zip(b.into_iter()) {
                *x = *x - y;
            }
        }
        self
    }
}

impl<T: SquareMatrixElem> Mul for SquareMatrix<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        assert_eq!(self.size, other.size);

        let other = other.transpose();
        let mut ret = Self::new(self.size);

        for (r, r2) in ret.data.iter_mut().zip(self.data.iter()) {
            for (x, c) in r.iter_mut().zip(other.data.iter()) {
                for (y, z) in r2.iter().zip(c.iter()) {
                    *x = *x + *y * *z;
                }
            }
        }

        ret
    }
}

impl<T: SquareMatrixElem> AddAssign for SquareMatrix<T> {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<T: SquareMatrixElem> SubAssign for SquareMatrix<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl<T: SquareMatrixElem> MulAssign for SquareMatrix<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<T: SquareMatrixElem + Neg<Output = T>> Neg for SquareMatrix<T> {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.data.iter_mut().for_each(|r| {
            r.iter_mut().for_each(|x| {
                *x = -*x;
            })
        });
        self
    }
}

impl<T> Index<usize> for SquareMatrix<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
