//! `usize`を用いた集合表現
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc142/tasks/abc142_e>
use std::ops::{BitAnd, BitOr, BitXor, Sub};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UsizeSet(pub usize);

impl UsizeSet {
    #[inline]
    pub fn set(self, i: usize) -> Self {
        Self(self.0 | (1 << i))
    }

    #[inline]
    pub fn reset(self, i: usize) -> Self {
        Self(self.0 & !(1 << i))
    }

    #[inline]
    pub fn flip(self, i: usize) -> Self {
        Self(self.0 ^ (1 << i))
    }

    #[inline]
    pub fn contains(self, i: usize) -> bool {
        (self.0 >> i) & 1 == 1
    }

    #[inline]
    pub fn fill(n: usize) -> Self {
        assert!(n <= usize::BITS as usize);
        if n == usize::BITS as usize {
            Self(!0)
        } else {
            Self(!(!0 << n))
        }
    }

    #[inline]
    pub fn difference(self, rhs: Self) -> Self {
        Self(self.0 & !rhs.0)
    }

    #[inline]
    pub fn union(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    #[inline]
    pub fn intersection(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    #[inline]
    pub fn symmetric_difference(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }
}

impl BitAnd for UsizeSet {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitOr for UsizeSet {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitXor for UsizeSet {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl Sub for UsizeSet {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = UsizeSet(0b0010010101);
        assert_eq!(
            (0..64).filter(|&i| a.contains(i)).collect::<Vec<_>>(),
            vec![0, 2, 4, 7]
        );

        let b = UsizeSet::fill(10);
    }
}
