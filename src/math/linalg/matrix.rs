use std::ops::{Add, AddAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    h: usize,
    w: usize,
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Copy + Default,
{
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            data: vec![vec![T::default(); w]; h],
        }
    }

    pub fn from_vec(other: Vec<Vec<T>>) -> Self {
        let h = other.len();
        assert!(h > 0);
        let w = other[0].len();
        assert!(other.iter().all(|r| r.len() == w));

        Self { h, w, data: other }
    }

    pub fn to_vec(&self) -> Vec<Vec<T>> {
        self.data.clone()
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn transpose(self) -> Self {
        let mut ret = Self::new(self.w, self.h);
        for i in 0..self.h {
            for j in 0..self.w {
                ret.data[j][i] = self.data[i][j];
            }
        }
        ret
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        self.data.get_mut(i).map(|a| a.get_mut(j)).flatten()
    }
}

impl<T> AddAssign for Matrix<T>
where
    T: Copy + Add<Output = T>,
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

impl<T> SubAssign for Matrix<T>
where
    T: Copy + Sub<Output = T>,
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

impl<T> MulAssign for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<T> Add for Matrix<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<T> Sub for Matrix<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        assert!(self.w == other.h);

        let n = self.h;
        let l = other.w;
        let other = other.transpose();
        let mut ret = Self::new(n, l);

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

impl<T> Neg for Matrix<T>
where
    T: Copy + Neg<Output = T>,
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

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
