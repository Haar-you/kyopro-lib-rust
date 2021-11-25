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

    pub fn straight_mul(self, b: Self) -> Self {
        assert_eq!(self.size, b.size);

        let b = b.transpose();
        let mut ret = Self::new(self.size);

        for (r, r2) in ret.data.iter_mut().zip(self.data.iter()) {
            for (x, c) in r.iter_mut().zip(b.data.iter()) {
                for (y, z) in r2.iter().zip(c.iter()) {
                    *x = *x + *y * *z;
                }
            }
        }

        ret
    }

    pub fn strassen_mul(self, b: Self) -> Self {
        let mut a = self;
        let n = a.size();

        if n <= 256 {
            return Self::straight_mul(a, b);
        }

        let m = (n + 1) / 2;

        let mut a11 = Self::new(m);
        let mut a12 = Self::new(m);
        let mut a21 = Self::new(m);
        let mut a22 = Self::new(m);

        let mut b11 = Self::new(m);
        let mut b12 = Self::new(m);
        let mut b21 = Self::new(m);
        let mut b22 = Self::new(m);

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
        self.strassen_mul(other)
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    use crate::{math::ff::modint::*, modulo};

    modulo!(M, 1000000007);
    type Mint = ModInt<M>;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let size = 300;

        let mut a = vec![vec![Mint::from(0); size]; size];
        let mut b = vec![vec![Mint::from(0); size]; size];

        for i in 0..size {
            for j in 0..size {
                a[i][j] = Mint::from(rng.gen::<u64>());
                b[i][j] = Mint::from(rng.gen::<u64>());
            }
        }

        let a = SquareMatrix::from_vec(a);
        let b = SquareMatrix::from_vec(b);

        assert_eq!(a.clone().straight_mul(b.clone()), a.strassen_mul(b));
    }

    #[test]
    #[ignore]
    fn benchmark() {
        use crate::timer;

        let mut rng = rand::thread_rng();

        let mut straight = vec![];
        let mut strassen = vec![];

        for &size in &[1, 10, 100, 1000] {
            let mut a = vec![vec![Mint::from(0); size]; size];
            let mut b = vec![vec![Mint::from(0); size]; size];

            for i in 0..size {
                for j in 0..size {
                    a[i][j] = Mint::from(rng.gen::<u64>());
                    b[i][j] = Mint::from(rng.gen::<u64>());
                }
            }

            let a = SquareMatrix::from_vec(a);
            let b = SquareMatrix::from_vec(b);

            straight.push(timer!({
                a.clone().straight_mul(b.clone());
            }));

            strassen.push(timer!({
                a.clone().strassen_mul(b.clone());
            }))
        }

        dbg!(straight, strassen);
    }
}
