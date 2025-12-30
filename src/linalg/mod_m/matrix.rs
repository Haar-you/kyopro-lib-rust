//! `h`×`w`行列
use crate::impl_ops;
pub use crate::linalg::traits::*;
use crate::num::ff::*;
use std::ops::{Index, Neg};

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

impl<Modulo: FF> TryAdd for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Self;
    fn try_add(mut self, rhs: Self) -> Option<Self::Output> {
        if self.h != rhs.h || self.w != rhs.h {
            None
        } else {
            for i in 0..self.h {
                for j in 0..self.w {
                    self.data[i][j] += rhs.data[i][j];
                }
            }
            Some(self)
        }
    }
}

impl<Modulo: FF> TrySub for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Self;
    fn try_sub(mut self, rhs: Self) -> Option<Self::Output> {
        if self.h != rhs.h || self.w != rhs.h {
            None
        } else {
            for i in 0..self.h {
                for j in 0..self.w {
                    self.data[i][j] -= rhs.data[i][j];
                }
            }
            Some(self)
        }
    }
}

impl<Modulo: FF> TryMul for Matrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Self;
    fn try_mul(self, rhs: Self) -> Option<Self::Output> {
        if self.w != rhs.h {
            None
        } else {
            let n = self.h;
            let l = rhs.w;
            let rhs = rhs.transpose();
            let mut ret = Self::new(n, l, self.modulo);

            for (r, r2) in ret.data.iter_mut().zip(self.data.iter()) {
                for (x, c) in r.iter_mut().zip(rhs.data.iter()) {
                    for (y, z) in r2.iter().zip(c.iter()) {
                        *x += *y * *z;
                    }
                }
            }

            Some(ret)
        }
    }
}

impl_ops!([Modulo: FF<Element: FFElem + Copy>]; AddAssign for Matrix<Modulo>, |x: &mut Self, y: Self| *x = x.clone().try_add(y).unwrap());
impl_ops!([Modulo: FF<Element: FFElem + Copy>]; SubAssign for Matrix<Modulo>, |x: &mut Self, y: Self| *x = x.clone().try_sub(y).unwrap());
impl_ops!([Modulo: FF<Element: FFElem + Copy>]; MulAssign for Matrix<Modulo>, |x: &mut Self, y: Self| *x = x.clone().try_mul(y).unwrap());

impl_ops!([Modulo: FF<Element: FFElem + Copy>]; Add for Matrix<Modulo>, |x: Self, y| x.try_add(y).unwrap());
impl_ops!([Modulo: FF<Element: FFElem + Copy>]; Sub for Matrix<Modulo>, |x: Self, y| x.try_sub(y).unwrap());
impl_ops!([Modulo: FF<Element: FFElem + Copy>]; Mul for Matrix<Modulo>, |x: Self, y| x.try_mul(y).unwrap());

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
