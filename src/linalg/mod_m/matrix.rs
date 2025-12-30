//! `h`×`w`行列
use crate::impl_ops;
pub use crate::linalg::traits::*;
use crate::num::ff::*;
use std::ops::{Index, Neg};

/// `h`×`w`行列
#[derive(Clone, PartialEq, Eq)]
pub struct MatrixModM<Modulo: FF> {
    h: usize,
    w: usize,
    modulo: Modulo,
    data: Vec<Vec<Modulo::Element>>,
}

impl<Modulo: FF> MatrixModM<Modulo>
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

    /// `size`×`size`の単位行列を作る。
    pub fn unit(size: usize, modulo: Modulo) -> Self {
        let mut ret = Self::new(size, size, modulo.clone());
        for i in 0..size {
            ret.data[i][i] = modulo.from_u64(1);
        }
        ret
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

    /// 行列の行数を返す。
    pub fn height(&self) -> usize {
        self.h
    }

    /// 行列の列数を返す。
    pub fn width(&self) -> usize {
        self.w
    }

    pub fn is_square(&self) -> bool {
        self.h == self.w
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

    /// 行列の`p`乗を求める。
    pub fn pow(self, mut p: u64) -> Option<Self> {
        if !self.is_square() {
            None
        } else {
            let size = self.w;
            let mut ret = Self::unit(size, self.modulo.clone());
            let mut a = self;

            while p > 0 {
                if p & 1 != 0 {
                    ret *= a.clone();
                }
                a *= a.clone();

                p >>= 1;
            }

            Some(ret)
        }
    }

    /// `i`行`j`列の要素への可変参照を返す。
    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Modulo::Element> {
        let a = self.data.get_mut(i)?;
        a.get_mut(j)
    }

    /// 愚直に行列積を求める。
    ///
    /// **Time complexity** $O(n^3)$
    fn straight_mul(self, rhs: Self) -> Self {
        assert_eq!(self.w, rhs.h);

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

        ret
    }

    /// Strassenのアルゴリズムによる行列乗算
    fn strassen_mul(self, b: Self) -> Self {
        let mut a = self;

        assert_eq!(a.width(), b.width());
        assert_eq!(a.height(), b.height());
        assert_eq!(a.width(), a.height());

        let n = a.width();

        if n <= 256 {
            return Self::straight_mul(a, b);
        }

        let m = n.div_ceil(2);

        let mut a11 = Self::new(m, m, a.modulo.clone());
        let mut a12 = Self::new(m, m, a.modulo.clone());
        let mut a21 = Self::new(m, m, a.modulo.clone());
        let mut a22 = Self::new(m, m, a.modulo.clone());

        let mut b11 = Self::new(m, m, a.modulo.clone());
        let mut b12 = Self::new(m, m, a.modulo.clone());
        let mut b21 = Self::new(m, m, a.modulo.clone());
        let mut b22 = Self::new(m, m, a.modulo.clone());

        for i in 0..m {
            for j in 0..m {
                a11.data[i][j] = a[i][j];
                b11.data[i][j] = b[i][j];

                if j + m < n {
                    a12.data[i][j] = a[i][j + m];
                    b12.data[i][j] = b[i][j + m];
                }

                if i + m < n {
                    a21.data[i][j] = a[i + m][j];
                    b21.data[i][j] = b[i + m][j];
                }

                if i + m < n && j + m < n {
                    a22.data[i][j] = a[i + m][j + m];
                    b22.data[i][j] = b[i + m][j + m];
                }
            }
        }

        let p1 = Self::strassen_mul(a11.clone() + a22.clone(), b11.clone() + b22.clone());
        let p2 = Self::strassen_mul(a21.clone() + a22.clone(), b11.clone());
        let p3 = Self::strassen_mul(a11.clone(), b12.clone() - b22.clone());
        let p4 = Self::strassen_mul(a22.clone(), b21.clone() - b11.clone());
        let p5 = Self::strassen_mul(a11.clone() + a12.clone(), b22.clone());
        let p6 = Self::strassen_mul(a21 - a11, b11 + b12);
        let p7 = Self::strassen_mul(a12 - a22, b21 + b22);

        let c11 = p1.clone() + p4.clone() - p5.clone() + p7;
        let c12 = p3.clone() + p5;
        let c21 = p2.clone() + p4;
        let c22 = p1 + p3 - p2 + p6;

        for i in 0..m {
            for j in 0..m {
                a.data[i][j] = c11[i][j];
                if j + m < n {
                    a.data[i][j + m] = c12[i][j];
                }
                if i + m < n {
                    a.data[i + m][j] = c21[i][j];
                }
                if i + m < n && j + m < n {
                    a.data[i + m][j + m] = c22[i][j];
                }
            }
        }

        a
    }
}

impl<Modulo: FF> TryAdd for MatrixModM<Modulo>
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

impl<Modulo: FF> TrySub for MatrixModM<Modulo>
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

impl<Modulo: FF> TryMul for MatrixModM<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Self;
    fn try_mul(self, rhs: Self) -> Option<Self::Output> {
        if self.w != rhs.h {
            None
        } else if self.is_square() && rhs.is_square() {
            Some(self.strassen_mul(rhs))
        } else {
            Some(self.straight_mul(rhs))
        }
    }
}

impl_ops!([Modulo: FF<Element: FFElem + Copy>]; AddAssign for MatrixModM<Modulo>, |x: &mut Self, y: Self| *x = x.clone().try_add(y).unwrap());
impl_ops!([Modulo: FF<Element: FFElem + Copy>]; SubAssign for MatrixModM<Modulo>, |x: &mut Self, y: Self| *x = x.clone().try_sub(y).unwrap());
impl_ops!([Modulo: FF<Element: FFElem + Copy>]; MulAssign for MatrixModM<Modulo>, |x: &mut Self, y: Self| *x = x.clone().try_mul(y).unwrap());

impl_ops!([Modulo: FF<Element: FFElem + Copy>]; Add for MatrixModM<Modulo>, |x: Self, y| x.try_add(y).unwrap());
impl_ops!([Modulo: FF<Element: FFElem + Copy>]; Sub for MatrixModM<Modulo>, |x: Self, y| x.try_sub(y).unwrap());
impl_ops!([Modulo: FF<Element: FFElem + Copy>]; Mul for MatrixModM<Modulo>, |x: Self, y| x.try_mul(y).unwrap());

impl<Modulo: FF> Neg for MatrixModM<Modulo>
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

impl<Modulo: FF> Index<usize> for MatrixModM<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = [Modulo::Element];
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<Modulo: FF> From<MatrixModM<Modulo>> for Vec<Vec<Modulo::Element>> {
    fn from(value: MatrixModM<Modulo>) -> Self {
        value.data
    }
}

impl<Modulo: FF> AsRef<[Vec<Modulo::Element>]> for MatrixModM<Modulo> {
    fn as_ref(&self) -> &[Vec<Modulo::Element>] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    use crate::{math::prime_mod::Prime, num::const_modint::*};

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();
        let modulo = ConstModIntBuilder::<Prime<1000000007>>::new();

        let size = 300;

        let mut a = vec![vec![0; size]; size];
        let mut b = vec![vec![0; size]; size];

        for i in 0..size {
            for j in 0..size {
                a[i][j] = rng.gen::<u32>();
                b[i][j] = rng.gen::<u32>();
            }
        }

        let a = MatrixModM::from_vec_2d(a, modulo);
        let b = MatrixModM::from_vec_2d(b, modulo);

        assert!(a.clone().straight_mul(b.clone()) == a.strassen_mul(b));
    }

    #[test]
    #[ignore]
    fn benchmark() {
        use crate::get_time;

        let mut rng = rand::thread_rng();
        let modulo = ConstModIntBuilder::<Prime<1000000007>>::new();

        let mut straight = vec![];
        let mut strassen = vec![];

        for &size in &[1, 10, 100, 300, 500] {
            let mut a = vec![vec![0; size]; size];
            let mut b = vec![vec![0; size]; size];

            for i in 0..size {
                for j in 0..size {
                    a[i][j] = rng.gen::<u32>();
                    b[i][j] = rng.gen::<u32>();
                }
            }

            let a = MatrixModM::from_vec_2d(a, modulo);
            let b = MatrixModM::from_vec_2d(b, modulo);

            straight.push(get_time!({
                a.clone().straight_mul(b.clone());
            }));

            strassen.push(get_time!({
                a.clone().strassen_mul(b.clone());
            }));
        }

        dbg!(straight, strassen);
    }
}
