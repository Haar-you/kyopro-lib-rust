//! 半群の列の区間取得($O(1)$)ができる。

pub use crate::algebra::traits::Semigroup;
use crate::misc::range::range_bounds_to_range;
use std::{iter::repeat_n, ops::RangeBounds};

/// 半群の列の区間取得($O(1)$)ができる。
pub struct DisjointSparseTable<S: Semigroup> {
    data: Vec<Vec<Option<S>>>,
    seq: Vec<Option<S>>,
    size: usize,
}

impl<S: Semigroup + Clone> DisjointSparseTable<S> {
    /// 列`seq`から`DisjointSparseTable<S>`を構築する。
    pub fn new(seq: Vec<S>) -> Self {
        assert!(!seq.is_empty());

        let size = seq.len();
        let log_size = usize::BITS as usize - (size - 1).leading_zeros() as usize;
        let mut data = vec![vec![None; 1 << log_size]; log_size];

        let seq = seq
            .into_iter()
            .map(Some)
            .chain(repeat_n(None, (1 << log_size) - size))
            .collect::<Vec<_>>();

        for (i, x) in seq.iter().enumerate() {
            data[0][i] = x.clone();
        }

        let mut this = Self { data, seq, size };
        this.build(0, 1 << log_size, log_size - 1);

        this
    }

    fn build(&mut self, l: usize, r: usize, d: usize) {
        let m = (l + r) / 2;

        self.data[d][m] = self.seq[m].clone();
        for i in m + 1..r {
            self.data[d][i] = match (self.data[d][i - 1].clone(), self.seq[i].clone()) {
                (Some(x), Some(y)) => Some(x.op(y)),
                (a, None) => a,
                (None, a) => a,
            }
        }

        self.data[d][m - 1] = self.seq[m - 1].clone();
        for i in (l..m - 1).rev() {
            self.data[d][i] = match (self.seq[i].clone(), self.data[d][i + 1].clone()) {
                (Some(x), Some(y)) => Some(x.op(y)),
                (a, None) => a,
                (None, a) => a,
            }
        }

        if d > 0 {
            self.build(l, m, d - 1);
            self.build(m, r, d - 1);
        }
    }

    /// **Time complexity** $O(1)$
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Option<S> {
        let (l, r) = range_bounds_to_range(range, 0, self.size);

        if l == r {
            None
        } else {
            let r = r - 1;

            if l == r {
                self.seq[l].clone()
            } else {
                let k = usize::BITS as usize - 1 - (l ^ r).leading_zeros() as usize;
                match (self.data[k][l].clone(), self.data[k][r].clone()) {
                    (Some(x), Some(y)) => Some(x.op(y)),
                    (a, None) => a,
                    (None, a) => a,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::sum::*;
    use my_testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 100;
        let a = (0..n)
            .map(|_| Sum(rng.gen::<u32>() % 10000))
            .collect::<Vec<_>>();
        let s = DisjointSparseTable::<Sum<u32>>::new(a.clone());

        for _ in 0..100 {
            let lr = rand_range(&mut rng, 0..n);

            assert_eq!(
                s.fold(lr.clone()),
                if lr.is_empty() {
                    None
                } else {
                    Some(a[lr].iter().cloned().fold_m())
                }
            );
        }
    }
}
