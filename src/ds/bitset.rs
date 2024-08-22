use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

type B = u64;
const B_SIZE: usize = 64;

#[derive(Clone, Debug)]
pub struct Bitset {
    data: Vec<B>,
    size: usize,
}

impl Bitset {
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![0; (n + B_SIZE - 1) / B_SIZE],
            size: n,
        }
    }

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

    pub fn test(&self, n: usize) -> bool {
        assert!(n < self.size);
        unsafe { (self.data.get_unchecked(n / B_SIZE) >> (n % B_SIZE)) & 1 == 1 }
    }

    pub fn flip(&mut self, n: usize) {
        assert!(n < self.size);
        unsafe {
            *self.data.get_unchecked_mut(n / B_SIZE) ^= 1 << (n % B_SIZE);
        }
    }

    pub fn count_ones(&self) -> u32 {
        self.data.iter().map(|a| a.count_ones()).sum()
    }

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

    pub fn len(&self) -> usize {
        self.size
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
