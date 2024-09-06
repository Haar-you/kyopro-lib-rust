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

impl From<Vec<usize>> for UsizeSet {
    fn from(value: Vec<usize>) -> Self {
        let mut ret = Self(0);
        for a in value {
            ret = ret.set(a);
        }
        ret
    }
}

impl From<UsizeSet> for Vec<usize> {
    fn from(value: UsizeSet) -> Self {
        (0..usize::BITS as usize)
            .filter(|i| value.contains(*i))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::iter::FromIterator;

    use super::*;
    use rand::{seq::SliceRandom, Rng};

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        #[allow(non_snake_case)]
        let U: Vec<usize> = (0..usize::BITS as usize).collect();

        for _ in 0..100 {
            let count = rng.gen::<usize>() % 65;
            let a: Vec<_> = U.choose_multiple(&mut rng, count).cloned().collect();
            let b: Vec<_> = U.choose_multiple(&mut rng, count).cloned().collect();

            let a_ans = BTreeSet::from_iter(a.clone());
            let b_ans = BTreeSet::from_iter(b.clone());

            let a_res = UsizeSet::from(a);
            let b_res = UsizeSet::from(b);

            let c_ans = &a_ans & &b_ans;
            let c_res = a_res & b_res;
            assert_eq!(BTreeSet::from_iter(Vec::from(c_res)), c_ans);

            let c_ans = &a_ans | &b_ans;
            let c_res = a_res | b_res;
            assert_eq!(BTreeSet::from_iter(Vec::from(c_res)), c_ans);

            let c_ans = &a_ans ^ &b_ans;
            let c_res = a_res ^ b_res;
            assert_eq!(BTreeSet::from_iter(Vec::from(c_res)), c_ans);

            let c_ans = &a_ans - &b_ans;
            let c_res = a_res - b_res;
            assert_eq!(BTreeSet::from_iter(Vec::from(c_res)), c_ans);
        }
    }
}