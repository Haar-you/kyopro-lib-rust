//! 正方行列
use crate::num::ff::*;
use std::ops::{Add, AddAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

/// 正方行列
#[derive(Clone, PartialEq, Eq)]
pub struct SquareMatrix<Modulo: FF>
where
    Modulo::Element: FFElem,
{
    size: usize,
    modulo: Modulo,
    data: Vec<Vec<Modulo::Element>>,
}

impl<Modulo: FF> SquareMatrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    /// `size`×`size`の零行列を作る。
    pub fn new(size: usize, modulo: Modulo) -> Self {
        Self {
            size,
            data: vec![vec![modulo.from_u64(0); size]; size],
            modulo,
        }
    }

    /// `size`×`size`の単位行列を作る。
    pub fn unit(size: usize, modulo: Modulo) -> Self {
        let mut ret = Self::new(size, modulo.clone());
        for i in 0..size {
            ret.data[i][i] = modulo.from_u64(1);
        }
        ret
    }

    /// [`Vec<Vec<u32>>`]から[`SquareMatrix`]を作る。
    pub fn from_vec_vec_u32(other: Vec<Vec<u32>>, modulo: Modulo) -> Self {
        let size = other.len();
        assert!(size > 0);
        assert!(other.iter().all(|r| r.len() == size));

        let other = other
            .into_iter()
            .map(|a| {
                a.into_iter()
                    .map(|x| modulo.from_u64(x as u64))
                    .collect::<Vec<_>>()
            })
            .collect();

        Self {
            size,
            data: other,
            modulo,
        }
    }

    /// 行列の行数(列数)を返す。
    pub fn size(&self) -> usize {
        self.size
    }

    /// 行列の転置を求める。
    pub fn transpose(self) -> Self {
        let mut ret = Self::new(self.size, self.modulo);
        for i in 0..self.size {
            for j in 0..self.size {
                ret.data[j][i] = self.data[i][j];
            }
        }
        ret
    }

    /// 行列の`p`乗を求める。
    pub fn pow(self, mut p: u64) -> Self {
        let mut ret = Self::unit(self.size, self.modulo.clone());
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

    /// `i`行`j`列の要素への可変参照を返す。
    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Modulo::Element> {
        let a = self.data.get_mut(i)?;
        a.get_mut(j)
    }

    /// 愚直に行列積を求める。
    ///
    /// **Time complexity** $O(n^3)$
    pub fn straight_mul(self, b: Self) -> Self {
        assert_eq!(self.size, b.size);

        let b = b.transpose();
        let mut ret = Self::new(self.size, self.modulo);

        for (r, r2) in ret.data.iter_mut().zip(self.data.iter()) {
            for (x, c) in r.iter_mut().zip(b.data.iter()) {
                for (y, z) in r2.iter().zip(c.iter()) {
                    *x += *y * *z;
                }
            }
        }

        ret
    }

    /// Strassenのアルゴリズムによる行列乗算
    pub fn strassen_mul(self, b: Self) -> Self {
        let mut a = self;
        let n = a.size();

        if n <= 256 {
            return Self::straight_mul(a, b);
        }

        let m = (n + 1) / 2;

        let mut a11 = Self::new(m, a.modulo.clone());
        let mut a12 = Self::new(m, a.modulo.clone());
        let mut a21 = Self::new(m, a.modulo.clone());
        let mut a22 = Self::new(m, a.modulo.clone());

        let mut b11 = Self::new(m, a.modulo.clone());
        let mut b12 = Self::new(m, a.modulo.clone());
        let mut b21 = Self::new(m, a.modulo.clone());
        let mut b22 = Self::new(m, a.modulo.clone());

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

impl<Modulo: FF> From<SquareMatrix<Modulo>> for Vec<Vec<Modulo::Element>> {
    fn from(value: SquareMatrix<Modulo>) -> Self {
        value.data
    }
}

impl<Modulo: FF> AsRef<[Vec<Modulo::Element>]> for SquareMatrix<Modulo> {
    fn as_ref(&self) -> &[Vec<Modulo::Element>] {
        &self.data
    }
}

impl<Modulo: FF> Add for SquareMatrix<Modulo>
where
    Modulo::Element: FFElem,
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        assert_eq!(self.size, other.size);
        for (a, b) in self.data.iter_mut().zip(other.data.into_iter()) {
            for (x, y) in a.iter_mut().zip(b.into_iter()) {
                *x += y;
            }
        }
        self
    }
}

impl<Modulo: FF> Sub for SquareMatrix<Modulo>
where
    Modulo::Element: FFElem,
{
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        assert_eq!(self.size, other.size);
        for (a, b) in self.data.iter_mut().zip(other.data.into_iter()) {
            for (x, y) in a.iter_mut().zip(b.into_iter()) {
                *x -= y;
            }
        }
        self
    }
}

impl<Modulo: FF> Mul for SquareMatrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        self.strassen_mul(other)
    }
}

impl<Modulo: FF> AddAssign for SquareMatrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<Modulo: FF> SubAssign for SquareMatrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl<Modulo: FF> MulAssign for SquareMatrix<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<Modulo: FF> Neg for SquareMatrix<Modulo>
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

impl<Modulo: FF> Index<usize> for SquareMatrix<Modulo>
where
    Modulo::Element: FFElem,
{
    type Output = [Modulo::Element];
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    use crate::num::const_modint::*;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();
        let modulo = ConstModIntBuilder::<1000000007>;

        let size = 300;

        let mut a = vec![vec![0; size]; size];
        let mut b = vec![vec![0; size]; size];

        for i in 0..size {
            for j in 0..size {
                a[i][j] = rng.gen::<u32>();
                b[i][j] = rng.gen::<u32>();
            }
        }

        let a = SquareMatrix::from_vec_vec_u32(a, modulo);
        let b = SquareMatrix::from_vec_vec_u32(b, modulo);

        assert!(a.clone().straight_mul(b.clone()) == a.strassen_mul(b));
    }

    #[test]
    #[ignore]
    fn benchmark() {
        use crate::get_time;

        let mut rng = rand::thread_rng();
        let modulo = ConstModIntBuilder::<1000000007>;

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

            let a = SquareMatrix::from_vec_vec_u32(a, modulo);
            let b = SquareMatrix::from_vec_vec_u32(b, modulo);

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
