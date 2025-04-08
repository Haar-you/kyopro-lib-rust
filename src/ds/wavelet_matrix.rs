//! Wavelet matrix
use crate::{ds::succinct_bitvec::SuccinctBitVec, misc::range::range_bounds_to_range};
use std::{
    marker::PhantomData,
    ops::{BitAnd, BitOrAssign, RangeBounds, Shl, Shr},
};

/// Wavelet matrix
#[derive(Clone)]
pub struct WaveletMatrix<T, const BIT_SIZE: usize> {
    size: usize,
    sdict: Vec<SuccinctBitVec>,
    zero_pos: Vec<usize>,
    _phantom: PhantomData<T>,
}

impl<T, const BIT_SIZE: usize> WaveletMatrix<T, BIT_SIZE>
where
    T: Shr<usize, Output = T>
        + Shl<usize, Output = T>
        + BitAnd<Output = T>
        + BitOrAssign
        + From<u8>
        + Eq
        + Ord
        + Copy,
{
    /// `T`の列から[`WaveletMatrix`]を作る。
    pub fn new(mut data: Vec<T>) -> Self {
        let size = data.len();

        let mut sdict = vec![];
        let mut zero_pos = vec![];

        for k in 0..BIT_SIZE {
            let mut left = vec![];
            let mut right = vec![];
            let mut s = vec![false; size];

            for i in 0..size {
                s[i] = (data[i] >> (BIT_SIZE - 1 - k)) & T::from(1) == T::from(1);
                if s[i] {
                    right.push(data[i]);
                } else {
                    left.push(data[i]);
                }
            }

            sdict.push(SuccinctBitVec::new(s));
            zero_pos.push(left.len());

            data = left;
            data.extend(right);
        }

        Self {
            size,
            sdict,
            zero_pos,
            _phantom: PhantomData,
        }
    }

    /// `index`番目の値を得る。
    pub fn access(&self, index: usize) -> T {
        let mut ret = T::from(0);

        let mut p = index;
        for i in 0..BIT_SIZE {
            let t = self.sdict[i].access(p);

            ret |= T::from(t as u8) << (BIT_SIZE - 1 - i);
            p = self.sdict[i].rank(p, t == 1) + t as usize * self.zero_pos[i];
        }

        ret
    }

    fn rank_(&self, index: usize, value: T) -> (usize, usize) {
        let mut l = 0;
        let mut r = index;

        for i in 0..BIT_SIZE {
            let t = (value >> (BIT_SIZE - 1 - i)) & T::from(1);

            if t == T::from(1) {
                l = self.sdict[i].rank(l, true) + self.zero_pos[i];
                r = self.sdict[i].rank(r, true) + self.zero_pos[i];
            } else {
                l = self.sdict[i].rank(l, false);
                r = self.sdict[i].rank(r, false);
            }
        }

        (l, r)
    }

    /// [0, index)に含まれる`value`の個数。
    pub fn rank(&self, index: usize, value: T) -> usize {
        let (l, r) = self.rank_(index, value);
        r - l
    }

    /// `range`に含まれる`value`の個数。
    pub fn count(&self, range: impl RangeBounds<usize>, value: T) -> usize {
        let (l, r) = range_bounds_to_range(range, 0, self.size);
        self.rank(r, value) - self.rank(l, value)
    }

    /// `nth`(0-indexed)番目の`value`の位置。
    pub fn select(&self, nth: usize, value: T) -> Option<usize> {
        let nth = nth + 1;

        let (l, r) = self.rank_(self.size, value);

        if r - l < nth {
            None
        } else {
            let mut p = l + nth - 1;

            for i in (0..BIT_SIZE).rev() {
                let t = (value >> (BIT_SIZE - i - 1)) & T::from(1);

                if t == T::from(1) {
                    p = self.sdict[i].select(p - self.zero_pos[i], true).unwrap();
                } else {
                    p = self.sdict[i].select(p, false).unwrap();
                }
            }

            Some(p)
        }
    }

    /// `range`で`nth`(0-indexed)番目に小さい値。
    pub fn quantile(&self, range: impl RangeBounds<usize>, nth: usize) -> Option<T> {
        let (mut l, mut r) = range_bounds_to_range(range, 0, self.size);
        if nth >= r - l {
            return None;
        }

        let mut nth = nth + 1;
        let mut ret = T::from(0);

        for (i, sdict) in self.sdict.iter().enumerate() {
            let count_1 = sdict.count(l..r, true);
            let count_0 = r - l - count_1;

            let mut t = 0;

            if nth > count_0 {
                t = 1;
                ret |= T::from(1) << (BIT_SIZE - i - 1);
                nth -= count_0;
            }

            let zeropos = unsafe { self.zero_pos.get_unchecked(i) };
            l = sdict.rank(l, t == 1) + t as usize * zeropos;
            r = sdict.rank(r, t == 1) + t as usize * zeropos;
        }

        Some(ret)
    }

    /// `range`での最大値
    pub fn maximum(&self, range: impl RangeBounds<usize>) -> Option<T> {
        let (l, r) = range_bounds_to_range(range, 0, self.size);
        if r > l {
            self.quantile(l..r, r - l - 1)
        } else {
            None
        }
    }

    /// `range`での最小値
    pub fn minimum(&self, range: impl RangeBounds<usize>) -> Option<T> {
        self.quantile(range, 0)
    }

    fn range_freq_lt(&self, range: impl RangeBounds<usize>, ub: T) -> usize {
        let (mut l, mut r) = range_bounds_to_range(range, 0, self.size);
        let mut ret = 0;
        for i in 0..BIT_SIZE {
            let t = (ub >> (BIT_SIZE - i - 1)) & T::from(1);
            if t == T::from(1) {
                ret += self.sdict[i].count(l..r, false);
                l = self.sdict[i].rank(l, true) + self.zero_pos[i];
                r = self.sdict[i].rank(r, true) + self.zero_pos[i];
            } else {
                l = self.sdict[i].rank(l, false);
                r = self.sdict[i].rank(r, false);
            }
        }
        ret
    }

    /// `range`で`lb`以上の最小値
    pub fn next_value(&self, range: impl RangeBounds<usize> + Clone, lb: T) -> Option<T> {
        let c = self.range_freq_lt(range.clone(), lb);
        self.quantile(range, c)
    }

    /// `range`で`ub`未満の最大値
    pub fn prev_value(&self, range: impl RangeBounds<usize> + Clone, ub: T) -> Option<T> {
        let c = self.range_freq_lt(range.clone(), ub);
        if c == 0 {
            None
        } else {
            self.quantile(range, c - 1)
        }
    }

    /// `range`で`lb`以上`ub`未満の値の個数
    pub fn range_freq(&self, range: impl RangeBounds<usize> + Clone, lb: T, ub: T) -> usize {
        if lb >= ub {
            return 0;
        }
        self.range_freq_lt(range.clone(), ub) - self.range_freq_lt(range, lb)
    }
}

/// [`u64`]の列を管理できる[`WaveletMatrix`]
pub type WM64 = WaveletMatrix<u64, 64>;
/// [`u32`]の列を管理できる[`WaveletMatrix`]
pub type WM32 = WaveletMatrix<u32, 32>;

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_range_loop)]
    use super::*;
    use crate::algo::bsearch::lower_bound;
    use my_testtools::*;
    use rand::Rng;

    #[test]
    fn test_access() {
        let mut rng = rand::thread_rng();
        let n = 10000;
        let b = (0..n).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

        let wm = WM64::new(b.clone());

        for i in 0..n {
            assert_eq!(wm.access(i), b[i]);
        }
    }

    #[test]
    fn test_rank() {
        let mut rng = rand::thread_rng();

        let m = 50;
        let table = (0..m).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

        let n = 300;
        let b = (0..n)
            .map(|_| table[rng.gen::<usize>() % m])
            .collect::<Vec<_>>();

        let wm = WM64::new(b.clone());

        for k in 0..m {
            let mut count = 0;
            for i in 0..=n {
                assert_eq!(wm.rank(i, table[k]), count);
                if b.get(i) == Some(&table[k]) {
                    count += 1;
                }
            }
        }
    }

    #[test]
    fn test_count() {
        let mut rng = rand::thread_rng();

        let m = 50;
        let table = (0..m).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

        let n = 300;
        let b = (0..n)
            .map(|_| table[rng.gen::<usize>() % m])
            .collect::<Vec<_>>();

        let wm = WM64::new(b.clone());

        for _ in 0..1000 {
            let lr = rand_range(&mut rng, 0..n);
            let x = table[rng.gen::<usize>() % m];

            let count = b[lr.clone()].iter().filter(|&&y| x == y).count();

            assert_eq!(wm.count(lr, x), count);
        }
    }

    #[test]
    fn test_select() {
        let mut rng = rand::thread_rng();

        let m = 50;
        let table = (0..m).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

        let n = 300;
        let b = (0..n)
            .map(|_| table[rng.gen::<usize>() % m])
            .collect::<Vec<_>>();

        let wm = WM64::new(b.clone());

        for x in table {
            let count = wm.count(.., x);

            assert_eq!(
                (0..count)
                    .map(|i| wm.select(i, x).unwrap())
                    .collect::<Vec<_>>(),
                (0..n).filter(|&i| b[i] == x).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_quantile() {
        let mut rng = rand::thread_rng();

        let m = 50;
        let table = (0..m).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

        let n = 300;
        let b = (0..n)
            .map(|_| table[rng.gen::<usize>() % m])
            .collect::<Vec<_>>();

        let wm = WM64::new(b.clone());

        for _ in 0..300 {
            let lr = rand_range(&mut rng, 0..n);

            let mut a = b[lr.clone()].to_vec();
            a.sort();

            assert_eq!(
                (0..lr.end - lr.start)
                    .map(|i| wm.quantile(lr.clone(), i).unwrap())
                    .collect::<Vec<_>>(),
                a
            );

            assert_eq!(wm.maximum(lr.clone()), a.last().copied());
            assert_eq!(wm.minimum(lr.clone()), a.first().copied());
        }
    }

    #[test]
    fn test_prev_next_value() {
        let mut rng = rand::thread_rng();

        let m = 50;
        let table = (0..m).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

        let n = 300;
        let b = (0..n)
            .map(|_| table[rng.gen::<usize>() % m])
            .collect::<Vec<_>>();

        let wm = WM64::new(b.clone());

        for _ in 0..1000 {
            let lr = rand_range(&mut rng, 0..n);

            let mut a = b[lr.clone()].to_vec();
            a.sort();

            let x = rng.gen::<u64>();
            let i = lower_bound(&a, &x);

            assert_eq!(wm.next_value(lr.clone(), x), a.get(i).copied());

            let i = lower_bound(&a, &x);

            assert_eq!(
                wm.prev_value(lr, x),
                if i == 0 { None } else { a.get(i - 1).copied() }
            );
        }
    }

    #[test]
    fn test_range_freq() {
        let mut rng = rand::thread_rng();

        let m = 50;
        let table = (0..m).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

        let n = 300;
        let b = (0..n)
            .map(|_| table[rng.gen::<usize>() % m])
            .collect::<Vec<_>>();

        let wm = WM64::new(b.clone());

        for _ in 0..1000 {
            let lr = rand_range(&mut rng, 0..n);
            let lb = rng.gen::<u64>();
            let ub = rng.gen::<u64>();

            assert_eq!(
                wm.range_freq(lr.clone(), lb, ub),
                b[lr].iter().filter(|&&x| lb <= x && x < ub).count()
            );
        }
    }
}
