//! `h`×`w`行列
use crate::num::ff::*;
use std::ops::{Add, AddAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

/// `h`×`w`行列
#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<Modulo: FF> {
    h: usize,
    w: usize,
    modulo: Modulo,
    data: Vec<Vec<Modulo::Element>>,
}

impl<Modulo: FF> Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    /// `h`×`w`の零行列を作る。
    pub fn new(h: usize, w: usize, modulo: Modulo) -> Self {
        Self {
            h,
            w,
            data: vec![vec![modulo.from_u64(0); w]; h],
            modulo,
        }
    }

    /// [`Vec<Vec<u32>>`]から[`Matrix<Modulo>`]を作る。
    pub fn from_vec_2d(other: Vec<Vec<u32>>, modulo: Modulo) -> Self {
        let h = other.len();
        assert!(h > 0);
        let w = other[0].len();
        assert!(other.iter().all(|r| r.len() == w));

        let other = other
            .into_iter()
            .map(|a| {
                a.into_iter()
                    .map(|x| modulo.from_u64(x as u64))
                    .collect::<Vec<_>>()
            })
            .collect();

        Self {
            h,
            w,
            data: other,
            modulo,
        }
    }

    // pub fn to_vec(&self) -> Vec<Vec<T>> {
    //     self.data.clone()
    // }

    /// 行列の行数を返す。
    pub fn height(&self) -> usize {
        self.h
    }

    /// 行列の列数を返す。
    pub fn width(&self) -> usize {
        self.w
    }

    /// `w`×`h`の転置行列を作る。
    pub fn transpose(self) -> Self {
        let mut ret = Self::new(self.w, self.h, self.modulo);
        for i in 0..self.h {
            for j in 0..self.w {
                ret.data[j][i] = self.data[i][j];
            }
        }
        ret
    }

    /// `i`行`j`列の要素への可変参照を返す。
    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Modulo::Element> {
        let a = self.data.get_mut(i)?;
        a.get_mut(j)
    }
}

impl<Modulo: FF> AddAssign for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    fn add_assign(&mut self, other: Self) {
        assert!(self.h == other.h && self.w == other.h);
        for i in 0..self.h {
            for j in 0..self.w {
                self.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
    }
}

impl<Modulo: FF> SubAssign for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        assert!(self.h == other.h && self.w == other.h);
        for i in 0..self.h {
            for j in 0..self.w {
                self.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
    }
}

impl<Modulo: FF> MulAssign for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<Modulo: FF> Add for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<Modulo: FF> Sub for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl<Modulo: FF> Mul for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        assert!(self.w == other.h);

        let n = self.h;
        let l = other.w;
        let other = other.transpose();
        let mut ret = Self::new(n, l, self.modulo);

        for (r, r2) in ret.data.iter_mut().zip(self.data.iter()) {
            for (x, c) in r.iter_mut().zip(other.data.iter()) {
                for (y, z) in r2.iter().zip(c.iter()) {
                    *x += *y * *z;
                }
            }
        }

        ret
    }
}

impl<Modulo: FF> Neg for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
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

impl<Modulo: FF> Index<usize> for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = [Modulo::Element];
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
