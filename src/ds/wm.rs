use crate::ds::succinct_dict::*;
use std::ops::Range;

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
            let mut s = vec![false; size];

            let mut left = vec![];
            let mut right = vec![];

            for i in 0..size {
                s[i] = (data[i] >> (BIT_SIZE - 1 - k)) & 1 != 0;
                if s[i] {
                    right.push(data[i]);
                } else {
                    left.push(data[i]);
                }
            }

            sdict.push(SuccinctDict::new(s));
            zero_pos.push(left.len());

            left.extend(right);
            std::mem::swap(&mut data, &mut left);
        }

        Self {
            size,
            sdict,
            zero_pos,
        }
    }

    pub fn access(&self, index: usize) -> u64 {
        let mut ret = 0;
        let mut p = index;
        for i in 0..BIT_SIZE {
            let t = self.sdict[i].access(p) as u64;
            ret |= t << (BIT_SIZE - 1 - i);
            p = self.sdict[i].rank(p, t != 0) as usize + t as usize * self.zero_pos[i];
        }

        ret
    }

    fn __rank(&self, index: usize, value: u64) -> (usize, usize) {
        let mut l = 0;
        let mut r = index;
        for i in 0..BIT_SIZE {
            let t = ((value >> (BIT_SIZE - i - 1)) & 1) as usize;
            l = self.sdict[i].rank(l, t == 1) + t * self.zero_pos[i];
            r = self.sdict[i].rank(r, t == 1) + t * self.zero_pos[i];
        }

        (l, r)
    }

    pub fn rank(&self, index: usize, value: u64) -> usize {
        let (l, r) = self.__rank(index, value);
        r - l
    }

    pub fn count(&self, Range { start: l, end: r }: Range<usize>, value: u64) -> usize {
        self.rank(r, value) - self.rank(l, value)
    }

    pub fn select(&self, nth: usize, value: u64) -> Option<usize> {
        let (l, r) = self.__rank(self.size, value);
        if r - l <= nth {
            return None;
        }

        let mut p = l + nth;
        for i in (0..BIT_SIZE).rev() {
            let t = ((value >> (BIT_SIZE - i - 1)) & 1) as usize;
            p = self.sdict[i]
                .select(p - t * self.zero_pos[i], t == 1)
                .unwrap();
        }

        Some(p)
    }

    pub fn quantile(
        &self,
        Range {
            start: mut l,
            end: mut r,
        }: Range<usize>,
        mut nth: usize,
    ) -> Option<u64> {
        let mut ret = 0;

        for i in 0..BIT_SIZE {
            let count_1 = self.sdict[i].rank(r, true) - self.sdict[i].rank(l, true);
            let count_0 = r - l - count_1;

            let mut t = 0;
            if nth + 1 > count_0 {
                t = 1;
                ret |= 1 << (BIT_SIZE - i - 1);
                nth -= count_0;
            }
            l = self.sdict[i].rank(l, t == 1) + t * self.zero_pos[i];
            r = self.sdict[i].rank(r, t == 1) + t * self.zero_pos[i];
        }

        Some(ret)
    }

    pub fn max(&self, Range { start: l, end: r }: Range<usize>) -> Option<u64> {
        self.quantile(l..r, r - l - 1)
    }

    pub fn min(&self, Range { start: l, end: r }: Range<usize>) -> Option<u64> {
        self.quantile(l..r, 0)
    }

    pub fn range_freq_lt(
        &self,
        Range {
            start: mut l,
            end: mut r,
        }: Range<usize>,
        ub: u64,
    ) -> usize {
        let mut ret = 0;

        for i in 0..BIT_SIZE {
            let t = ((ub >> (BIT_SIZE - i - 1)) & 1) as usize;

            if t == 1 {
                ret += self.sdict[i].count(l..r, false);
            }

            l = self.sdict[i].rank(l, t == 1) + t * self.zero_pos[i];
            r = self.sdict[i].rank(r, t == 1) + t * self.zero_pos[i];
        }

        ret
    }

    pub fn next_value(&self, Range { start: l, end: r }: Range<usize>, lb: u64) -> Option<u64> {
        let c = self.range_freq_lt(l..r, lb);
        self.quantile(l..r, c + 1)
    }

    pub fn prev_value(&self, Range { start: l, end: r }: Range<usize>, ub: u64) -> Option<u64> {
        let c = self.range_freq_lt(l..r, ub);
        self.quantile(l..r, c)
    }

    pub fn range_freq(&self, Range { start: l, end: r }: Range<usize>, lb: u64, ub: u64) -> usize {
        self.range_freq_lt(l..r, ub) - self.range_freq_lt(l..r, lb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_access() {
        let mut rng = rand::thread_rng();

        let n = 1000;
        let data = (0..n).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

        let wm = WaveletMatrix::new(data.clone());

        assert_eq!(data, (0..n).map(|i| wm.access(i)).collect::<Vec<_>>());
    }

    #[test]
    fn test_rank() {
        let mut rng = rand::thread_rng();

        let n = 100;
        let data = (0..n).map(|_| rng.gen::<u64>() % 100).collect::<Vec<_>>();

        let wm = WaveletMatrix::new(data.clone());

        for x in 0..100 {
            for i in 0..=n {
                let t = (0..i).filter(|&i| data[i] == x).count();
                assert_eq!(wm.rank(i, x), t);
            }
        }
    }

    #[test]
    fn test_count() {
        let mut rng = rand::thread_rng();

        let n = 30;
        let data = (0..n).map(|_| rng.gen::<u64>() % 100).collect::<Vec<_>>();

        let wm = WaveletMatrix::new(data.clone());

        for x in 0..100 {
            for l in 0..n {
                for r in l + 1..=n {
                    assert_eq!(wm.count(l..r, x), (l..r).filter(|&y| data[y] == x).count());
                }
            }
        }
    }

    #[test]
    fn test_select() {
        let mut rng = rand::thread_rng();

        let n = 30;
        let data = (0..n).map(|_| rng.gen::<u64>() % 100).collect::<Vec<_>>();

        let wm = WaveletMatrix::new(data.clone());

        for x in 0..100 {
            for i in 0..n {
                assert_eq!(wm.select(i, x), (0..n).filter(|&y| data[y] == x).nth(i));
            }
        }
    }

    #[test]
    fn test_quantile() {
        let mut rng = rand::thread_rng();

        let n = 200;
        let data = (0..n).map(|_| rng.gen::<u64>() % 100).collect::<Vec<_>>();

        let wm = WaveletMatrix::new(data.clone());

        for _ in 0..100 {
            let l = rng.gen::<usize>() % n;
            let r = l + rng.gen::<usize>() % (n - l) + 1;

            let mut b = data[l..r].to_vec();
            b.sort();

            for i in 0..r - l {
                assert_eq!(wm.quantile(l..r, i).unwrap(), b[i]);
            }
        }
    }

    #[test]
    fn test_range_freq() {
        let mut rng = rand::thread_rng();

        let n = 200;
        let data = (0..n).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

        let wm = WaveletMatrix::new(data.clone());

        for _ in 0..100 {
            let l = rng.gen::<usize>() % n;
            let r = l + rng.gen::<usize>() % (n - l) + 1;

            let mut lb = rng.gen::<u64>();
            let mut ub = rng.gen::<u64>();

            if ub < lb {
                std::mem::swap(&mut lb, &mut ub);
            }

            assert_eq!(
                wm.range_freq(l..r, lb, ub),
                (&data[l..r]).iter().filter(|&&x| lb <= x && x < ub).count()
            );
        }
    }
}
