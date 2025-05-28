//! 半環上の行列
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc009/tasks/abc009_4>
use std::ops::Index;

use crate::algebra::semiring::Semiring;

/// 半環上の行列
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemiringMatrix<T> {
    data: Vec<Vec<T>>,
    h: usize,
    w: usize,
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

    /// `w`×`h`の転置行列を作る。
    pub fn transpose(self) -> Self {
        let a = self;
        let mut ret = Self::zero(a.w, a.h);
        for i in 0..a.h {
            for j in 0..a.w {
                ret.data[j][i] = a.data[i][j];
            }
        }
        ret
    }

    /// 行列積$ab$を求める。
    ///
    /// $a$の列数と$b$の行数が一致していないとき、行列積は定義されないので、`None`を返す。
    pub fn try_mul(self, b: Self) -> Option<Self> {
        let a = self;
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

    /// 行列`a`の`n`乗を求める。
    pub fn pow(self, mut n: u64) -> Self {
        let mut a = self;
        assert_eq!(a.h, a.w);

        let mut ret = Self::unit(a.h);

        while n > 0 {
            if n % 2 == 1 {
                ret = ret.try_mul(a.clone()).unwrap();
            }
            a = a.clone().try_mul(a).unwrap();
            n >>= 1;
        }

        ret
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

impl<T> Index<usize> for SemiringMatrix<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
