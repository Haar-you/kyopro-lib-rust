//! 任意サイズのbit列を扱う。
use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

type B = u64;
const B_SIZE: usize = 64;

/// 任意サイズのbit列を扱う。
#[derive(Clone, Debug)]
pub struct Bitset {
    pub(crate) data: Vec<B>,
    size: usize,
}

impl Bitset {
    /// `Bitset`内部で扱う型のBit数
    pub const B_SIZE: usize = B_SIZE;

    /// 長さ`n`の`Bitset`を構築する。
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![0; (n + B_SIZE - 1) / B_SIZE],
            size: n,
        }
    }

    /// `n`番目のbitを`val`で設定する。
    pub fn set(&mut self, n: usize, val: bool) {
        assert!(n < self.size);
        if val {
            unsafe {
                *self.data.get_unchecked_mut(n / B_SIZE) |= 1 << (n % B_SIZE);
            }
        } else {
            unsafe {
                *self.data.get_unchecked_mut(n / B_SIZE) &= !(1 << (n % B_SIZE));
            }
        }
    }

    /// `n`番目のbitが`1`ならば`true`を返す。
    pub fn test(&self, n: usize) -> bool {
        assert!(n < self.size);
        unsafe { (self.data.get_unchecked(n / B_SIZE) >> (n % B_SIZE)) & 1 == 1 }
    }

    /// `n`番目のbitを反転させる。
    pub fn flip(&mut self, n: usize) {
        assert!(n < self.size);
        unsafe {
            *self.data.get_unchecked_mut(n / B_SIZE) ^= 1 << (n % B_SIZE);
        }
    }

    /// `1`が設定されているbitの個数を数える。
    ///
    /// **Time complexity** $O(n)$
    pub fn count_ones(&self) -> u32 {
        self.data.iter().map(|a| a.count_ones()).sum()
    }

    /// `0`が設定されているbitの個数を数える。
    ///
    /// **Time complexity** $O(n)$
    pub fn count_zeros(&self) -> u32 {
        self.size as u32 - self.count_ones()
    }

    pub fn and_count_ones(&self, rhs: &Self) -> u32 {
        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| (a & b).count_ones())
            .sum()
    }

    pub fn same_size_xor_assign(&mut self, rhs: &Self) {
        assert_eq!(self.size, rhs.size);
        for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
            *a ^= b;
        }
    }

    /// bit列の長さを返す。
    ///
    /// **Time complexity** $O(1)$
    pub fn len(&self) -> usize {
        self.size
    }

    /// bit列が空ならば`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl From<Vec<bool>> for Bitset {
    fn from(value: Vec<bool>) -> Self {
        let mut ret = Self::new(value.len());

        for (i, x) in value.chunks(B_SIZE).enumerate() {
            let mut a = 0;

            for (j, y) in x.iter().enumerate() {
                if *y {
                    a |= 1 << j;
                }
            }

            ret.data[i] = a;
        }

        ret
    }
}

impl BitAnd for Bitset {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl BitAndAssign for Bitset {
    fn bitand_assign(&mut self, mut rhs: Self) {
        if self.size > rhs.size {
            std::mem::swap(self, &mut rhs);
        }

        for (a, b) in self.data.iter_mut().zip(rhs.data.into_iter()) {
            *a &= b;
        }
    }
}

impl BitOr for Bitset {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl BitOrAssign for Bitset {
    fn bitor_assign(&mut self, mut rhs: Self) {
        if self.size < rhs.size {
            std::mem::swap(self, &mut rhs);
        }

        for (a, b) in self.data.iter_mut().zip(rhs.data.into_iter()) {
            *a |= b;
        }
    }
}

impl BitXor for Bitset {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}

impl BitXorAssign for Bitset {
    fn bitxor_assign(&mut self, mut rhs: Self) {
        if self.size < rhs.size {
            std::mem::swap(self, &mut rhs);
        }

        for (a, b) in self.data.iter_mut().zip(rhs.data.into_iter()) {
            *a ^= b;
        }
    }
}

impl Display for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(a) = self.data.last() {
            let w = self.len() % B_SIZE;
            let w = if w == 0 { B_SIZE } else { w };
            f.write_fmt(format_args!("{:0width$b}", a, width = w))?
        }
        for a in self.data.iter().rev().skip(1) {
            f.write_fmt(format_args!("{:0width$b}", a, width = B_SIZE))?
        }
        Ok(())
    }
}
