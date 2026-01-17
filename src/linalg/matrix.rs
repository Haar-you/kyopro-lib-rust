//! $\mathbb{Z} / m \mathbb{Z}$ 上の行列
pub use crate::linalg::traits::*;
use crate::{algebra::semiring::*, impl_ops};
use std::ops::{Index, Neg};

/// $\mathbb{Z} / m \mathbb{Z}$ 上の行列
#[derive(Clone, PartialEq, Eq)]
pub struct MatrixOnRing<R: Ring> {
    h: usize,
    w: usize,
    ring: R,
    data: Vec<Vec<R::Element>>,
}

impl<R: Ring> Matrix for MatrixOnRing<R> {
    fn width(&self) -> usize {
        self.w
    }
    fn height(&self) -> usize {
        self.h
    }
}

impl<R: Ring> MatrixTranspose for MatrixOnRing<R>
where
    R::Element: Copy,
{
    type Output = Self;
    fn transpose(self) -> Self::Output {
        let mut ret = Self::zero(self.ring, self.w, self.h);
        for i in 0..self.h {
            for j in 0..self.w {
                ret.data[j][i] = self.data[i][j];
            }
        }
        ret
    }
}

impl<R: Ring> MatrixOnRing<R>
where
    R::Element: Copy,
{
    /// `h`×`w`の零行列を作る。
    pub fn zero(ring: R, h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            data: vec![vec![ring.zero(); w]; h],
            ring,
        }
    }

    /// `size`×`size`の単位行列を作る。
    pub fn unit(ring: R, size: usize) -> Self {
        let one = ring.one();
        let mut ret = Self::zero(ring, size, size);
        for i in 0..size {
            ret.data[i][i] = one;
        }
        ret
    }

    /// `i`行`j`列の要素への参照を返す。
    pub fn get(&mut self, i: usize, j: usize) -> Option<&R::Element> {
        let a = self.data.get(i)?;
        a.get(j)
    }

    /// `i`行`j`列の要素への可変参照を返す。
    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut R::Element> {
        let a = self.data.get_mut(i)?;
        a.get_mut(j)
    }
}

impl<R: Ring + Clone> MatrixOnRing<R>
where
    R::Element: Copy,
{
    /// 行列の`p`乗を求める。
    pub fn pow(self, mut p: u64) -> Option<Self> {
        self.is_square().then(|| {
            let size = self.w;
            let mut ret = Self::unit(self.ring.clone(), size);
            let mut a = self;

            while p > 0 {
                if p & 1 != 0 {
                    ret *= a.clone();
                }
                a *= a.clone();

                p >>= 1;
            }

            ret
        })
    }

    /// 愚直に行列積を求める。
    ///
    /// **Time complexity** $O(n^3)$
    fn straight_mul(self, rhs: Self) -> Self {
        assert_eq!(self.w, rhs.h);

        let n = self.h;
        let l = rhs.w;
        let rhs = rhs.transpose();
        let mut ret = Self::zero(self.ring.clone(), n, l);
        let s = &self.ring;

        for (r, r2) in ret.data.iter_mut().zip(self.data.iter()) {
            for (x, c) in r.iter_mut().zip(rhs.data.iter()) {
                for (y, z) in r2.iter().zip(c.iter()) {
                    *x = s.add(*x, s.mul(*y, *z));
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

        let mut a11 = Self::zero(a.ring.clone(), m, m);
        let mut a12 = Self::zero(a.ring.clone(), m, m);
        let mut a21 = Self::zero(a.ring.clone(), m, m);
        let mut a22 = Self::zero(a.ring.clone(), m, m);
        let mut b11 = Self::zero(a.ring.clone(), m, m);
        let mut b12 = Self::zero(a.ring.clone(), m, m);
        let mut b21 = Self::zero(a.ring.clone(), m, m);
        let mut b22 = Self::zero(a.ring.clone(), m, m);

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

impl<R: Ring> TryAdd for MatrixOnRing<R>
where
    R::Element: Copy,
{
    type Output = Self;
    fn try_add(mut self, rhs: Self) -> Option<Self::Output> {
        (self.size() == rhs.size()).then(|| {
            for i in 0..self.h {
                for j in 0..self.w {
                    self.data[i][j] = self.ring.add(self.data[i][j], rhs.data[i][j]);
                }
            }
            self
        })
    }
}

impl<R: Ring> TrySub for MatrixOnRing<R>
where
    R::Element: Copy,
{
    type Output = Self;
    fn try_sub(mut self, rhs: Self) -> Option<Self::Output> {
        (self.size() == rhs.size()).then(|| {
            for i in 0..self.h {
                for j in 0..self.w {
                    self.data[i][j] = self.ring.sub(self.data[i][j], rhs.data[i][j]);
                }
            }
            self
        })
    }
}

impl<R: Ring + Clone> TryMul for MatrixOnRing<R>
where
    R::Element: Copy,
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

impl_ops!([R: Ring<Element: Copy> + Clone]; AddAssign for MatrixOnRing<R>, |x: &mut Self, y: Self| *x = x.clone().try_add(y).unwrap());
impl_ops!([R: Ring<Element: Copy> + Clone]; SubAssign for MatrixOnRing<R>, |x: &mut Self, y: Self| *x = x.clone().try_sub(y).unwrap());
impl_ops!([R: Ring<Element: Copy> + Clone]; MulAssign for MatrixOnRing<R>, |x: &mut Self, y: Self| *x = x.clone().try_mul(y).unwrap());

impl_ops!([R: Ring<Element: Copy> + Clone]; Add for MatrixOnRing<R>, |x: Self, y| x.try_add(y).unwrap());
impl_ops!([R: Ring<Element: Copy> + Clone]; Sub for MatrixOnRing<R>, |x: Self, y| x.try_sub(y).unwrap());
impl_ops!([R: Ring<Element: Copy> + Clone]; Mul for MatrixOnRing<R>, |x: Self, y| x.try_mul(y).unwrap());

impl<R: Ring> Neg for MatrixOnRing<R>
where
    R::Element: Copy,
{
    type Output = Self;
    fn neg(mut self) -> Self {
        self.data.iter_mut().for_each(|r| {
            r.iter_mut().for_each(|x| {
                *x = self.ring.neg(*x);
            })
        });
        self
    }
}

impl<R: Ring> Index<usize> for MatrixOnRing<R> {
    type Output = [R::Element];
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<R: Ring> From<MatrixOnRing<R>> for Vec<Vec<R::Element>> {
    fn from(value: MatrixOnRing<R>) -> Self {
        value.data
    }
}

impl<R: Ring> AsRef<[Vec<R::Element>]> for MatrixOnRing<R> {
    fn as_ref(&self) -> &[Vec<R::Element>] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    use crate::{
        algebra::semiring::{add_mul_mod::AddMulMod, xor_and::XorAnd},
        math::prime_mod::Prime,
        num::const_modint::*,
    };

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();
        let modulo = ConstModIntBuilder::<Prime<1000000007>>::new();
        let ring = AddMulMod::new(modulo);

        let size = 300;

        let mut a = MatrixOnRing::zero(ring, size, size);
        let mut b = MatrixOnRing::zero(ring, size, size);

        for i in 0..size {
            for j in 0..size {
                *a.get_mut(i, j).unwrap() = modulo.from_u64(rng.gen::<u32>() as u64);
                *b.get_mut(i, j).unwrap() = modulo.from_u64(rng.gen::<u32>() as u64);
            }
        }

        assert!(a.clone().straight_mul(b.clone()) == a.strassen_mul(b));
    }

    #[test]
    fn test_xor_and() {
        let mut rng = rand::thread_rng();
        let ring = XorAnd::<u64>::new();

        let size = 300;

        let mut a = MatrixOnRing::zero(ring, size, size);
        let mut b = MatrixOnRing::zero(ring, size, size);

        for i in 0..size {
            for j in 0..size {
                *a.get_mut(i, j).unwrap() = rng.gen::<u64>();
                *b.get_mut(i, j).unwrap() = rng.gen::<u64>();
            }
        }

        assert!(a.clone().straight_mul(b.clone()) == a.strassen_mul(b));
    }

    #[test]
    #[ignore]
    fn benchmark() {
        use crate::get_time;

        let mut rng = rand::thread_rng();
        let modulo = ConstModIntBuilder::<Prime<1000000007>>::new();
        let ring = AddMulMod::new(modulo);

        let mut straight = vec![];
        let mut strassen = vec![];

        for &size in &[1, 10, 100, 300, 500] {
            let mut a = MatrixOnRing::zero(ring, size, size);
            let mut b = MatrixOnRing::zero(ring, size, size);

            for i in 0..size {
                for j in 0..size {
                    *a.get_mut(i, j).unwrap() = modulo.from_u64(rng.gen::<u32>() as u64);
                    *b.get_mut(i, j).unwrap() = modulo.from_u64(rng.gen::<u32>() as u64);
                }
            }

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
