//! $\mathbb{Z} / 2 \mathbb{Z}$上の行列
use crate::ds::bitset::Bitset;
use crate::impl_ops;
pub use crate::linalg::traits::*;
use std::ops::Index;

/// $\mathbb{Z} / 2 \mathbb{Z}$上の行列
#[derive(Clone)]
pub struct MatrixMod2 {
    h: usize,
    w: usize,
    data: Vec<Bitset>,
}

impl Matrix for MatrixMod2 {
    fn width(&self) -> usize {
        self.w
    }
    fn height(&self) -> usize {
        self.h
    }
}

impl MatrixTranspose for MatrixMod2 {
    type Output = Self;
    fn transpose(self) -> Self::Output {
        let mut ret = Self::zero(self.w, self.h);
        for i in 0..self.h {
            for j in 0..self.w {
                if self.data[i].test(j) {
                    ret.data[j].flip(i);
                }
            }
        }
        ret
    }
}

impl MatrixMod2 {
    /// `h`行`w`列の零行列を作る。
    pub fn zero(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            data: vec![Bitset::new(w); h],
        }
    }

    /// `n`行`n`列の単位行列を作る。
    pub fn unit(n: usize) -> Self {
        let mut ret = Self::zero(n, n);
        for i in 0..n {
            ret.data[i].flip(i);
        }
        ret
    }

    /// [`Bitset`]の`Vec`から`MatrixMod2`を生成する
    pub fn from_vec_bitset(other: Vec<Bitset>) -> Self {
        let h = other.len();
        assert!(h > 0);
        let w = other[0].len();
        assert!(other.iter().all(|r| r.len() == w));

        Self { h, w, data: other }
    }

    /// 行列の`p`乗を求める。
    pub fn pow(self, mut p: u64) -> Option<Self> {
        self.is_square().then(|| {
            let size = self.w;
            let mut ret = Self::unit(size);
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

    /// `i`行`j`列の成分を返す
    pub fn get(&self, i: usize, j: usize) -> Option<u32> {
        let a = self.data.get(i)?;
        (j < a.len()).then(|| a.test(j) as u32)
    }
}

impl TryAdd for MatrixMod2 {
    type Output = Self;
    fn try_add(mut self, rhs: Self) -> Option<Self::Output> {
        (self.size() == rhs.size()).then(|| {
            for (x, y) in self.data.iter_mut().zip(rhs.data) {
                *x ^= y;
            }
            self
        })
    }
}

impl TrySub for MatrixMod2 {
    type Output = Self;
    fn try_sub(self, rhs: Self) -> Option<Self::Output> {
        self.try_add(rhs)
    }
}

impl TryMul for MatrixMod2 {
    type Output = Self;
    fn try_mul(self, rhs: Self) -> Option<Self::Output> {
        (self.w == rhs.h).then(|| {
            let n = self.h;
            let l = rhs.w;
            let rhs = rhs.transpose();

            let mut ret = Self::zero(n, l);

            for (r, r2) in ret.data.iter_mut().zip(self.data.iter()) {
                for (i, c) in rhs.data.chunks(Bitset::B_SIZE).enumerate() {
                    let mut a = 0;

                    for (j, x) in c.iter().enumerate() {
                        let t = r2.and_count_ones(x) & 1;

                        if t != 0 {
                            a |= 1 << j;
                        }
                    }

                    r.data[i] = a;
                }
            }

            ret
        })
    }
}

impl_ops!(AddAssign for MatrixMod2, |x: &mut Self, y: Self| *x = x.clone().try_add(y).unwrap());
impl_ops!(SubAssign for MatrixMod2, |x: &mut Self, y: Self| *x = x.clone().try_sub(y).unwrap());
impl_ops!(MulAssign for MatrixMod2, |x: &mut Self, y: Self| *x = x.clone().try_mul(y).unwrap());

impl_ops!(Add for MatrixMod2, |x: Self, y| x.try_add(y).unwrap());
impl_ops!(Sub for MatrixMod2, |x: Self, y| x.try_sub(y).unwrap());
impl_ops!(Mul for MatrixMod2, |x: Self, y| x.try_mul(y).unwrap());

impl_ops!(Neg for MatrixMod2, |x: Self| x);

impl Index<usize> for MatrixMod2 {
    type Output = Bitset;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
