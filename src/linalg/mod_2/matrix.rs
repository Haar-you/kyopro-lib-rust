//! 行列 (mod 2)
use std::ops::Index;
use std::ops::Mul;

use crate::ds::bitset::Bitset;

/// 行列 (mod 2)
#[derive(Clone)]
pub struct MatrixMod2 {
    h: usize,
    w: usize,
    data: Vec<Bitset>,
}

impl MatrixMod2 {
    /// `h`行`w`列の`MatrixMod2`を生成
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            data: vec![Bitset::new(w); h],
        }
    }

    /// [`Bitset`]の`Vec`から`MatrixMod2`を生成する
    pub fn from_vec_bitset(other: Vec<Bitset>) -> Self {
        let h = other.len();
        assert!(h > 0);
        let w = other[0].len();
        assert!(other.iter().all(|r| r.len() == w));

        Self { h, w, data: other }
    }

    /// 行列の転置を得る
    pub fn transpose(self) -> Self {
        let mut ret = Self::new(self.w, self.h);
        for i in 0..self.h {
            for j in 0..self.w {
                if self.data[i].test(j) {
                    ret.data[j].flip(i);
                }
            }
        }
        ret
    }

    /// `i`行`j`列の成分を返す
    pub fn get(&self, i: usize, j: usize) -> Option<u32> {
        let a = self.data.get(i)?;
        (j < a.len()).then(|| a.test(j) as u32)
    }
}

impl Mul for MatrixMod2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.w, rhs.h);

        let n = self.h;
        let l = rhs.w;
        let rhs = rhs.transpose();

        let mut ret = Self::new(n, l);

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
    }
}

impl Index<usize> for MatrixMod2 {
    type Output = Bitset;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
