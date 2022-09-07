use crate::ds::succinct_dict::SuccinctDict;
use std::ops::{Bound, RangeBounds};

const BIT_SIZE: usize = 64;

pub struct WaveletMatrix {
    size: usize,
    sdict: Vec<SuccinctDict>,
    zero_pos: Vec<usize>,
}

impl WaveletMatrix {
    pub fn new(mut data: Vec<u64>) -> Self {
        let size = data.len();

        let mut sdict = vec![];
        let mut zero_pos = vec![];

        for k in 0..BIT_SIZE {
            let mut left = vec![];
            let mut right = vec![];
            let mut s = vec![false; size];

            for i in 0..size {
                s[i] = (data[i] >> (BIT_SIZE - 1 - k)) & 1 == 1;
                if s[i] {
                    right.push(data[i]);
                } else {
                    left.push(data[i]);
                }
            }

            sdict.push(SuccinctDict::new(s));
            zero_pos.push(left.len());

            data = left;
            data.extend(right);
        }

        Self {
            size,
            sdict,
            zero_pos,
        }
    }

    fn get_half_open_range(&self, r: impl RangeBounds<usize>) -> (usize, usize) {
        let l = match r.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };
        let r = match r.end_bound() {
            Bound::Unbounded => self.size,
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
        };
        assert!(l <= r && r <= self.size);

        (l, r)
    }

    /// `index`番目の値を得る。
    pub fn access(&self, index: usize) -> u64 {
        let mut ret = 0;

        let mut p = index;
        for i in 0..BIT_SIZE {
            let t = self.sdict[i].access(p);

            ret |= t << (BIT_SIZE - 1 - i);
            p = self.sdict[i].rank(p, t == 1) + t as usize * self.zero_pos[i];
        }

        ret
    }

    fn rank_(&self, index: usize, value: u64) -> (usize, usize) {
        let mut l = 0;
        let mut r = index;

        for i in 0..BIT_SIZE {
            let t = (value >> (BIT_SIZE - 1 - i)) & 1;
            l = self.sdict[i].rank(l, t == 1) + t as usize * self.zero_pos[i];
            r = self.sdict[i].rank(r, t == 1) + t as usize * self.zero_pos[i];
        }

        (l, r)
    }

    /// [0, index)に含まれる`value`の個数。
    pub fn rank(&self, index: usize, value: u64) -> usize {
        let (l, r) = self.rank_(index, value);
        r - l
    }

    /// `range`に含まれる`value`の個数。
    pub fn count(&self, range: impl RangeBounds<usize>, value: u64) -> usize {
        let (l, r) = self.get_half_open_range(range);
        self.rank(r, value) - self.rank(l, value)
    }

    /// `nth`(0-indexed)番目の`value`の位置。
    pub fn select(&self, nth: usize, value: u64) -> Option<usize> {
        let nth = nth + 1;

        let (l, r) = self.rank_(self.size, value);

        if r - l < nth {
            None
        } else {
            let mut p = l + nth - 1;

            for i in (0..BIT_SIZE).rev() {
                let t = (value >> (BIT_SIZE - i - 1)) & 1;
                p = self.sdict[i]
                    .select(p - t as usize * self.zero_pos[i], t == 1)
                    .unwrap();
            }

            Some(p)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testtools::*;
    use rand::Rng;

    #[test]
    fn test_access() {
        let mut rng = rand::thread_rng();
        let n = 10000;
        let b = (0..n).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

        let wm = WaveletMatrix::new(b.clone());

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

        let wm = WaveletMatrix::new(b.clone());

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

        let wm = WaveletMatrix::new(b.clone());

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

        let wm = WaveletMatrix::new(b.clone());

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
}
