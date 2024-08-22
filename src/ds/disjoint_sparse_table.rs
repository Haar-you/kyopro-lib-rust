//! 半群の列の区間演算を行う(O(1))

pub use crate::algebra::traits::Semigroup;
use std::{iter::repeat, ops::Range};

pub struct DisjointSparseTable<S: Semigroup> {
    data: Vec<Vec<Option<S::Output>>>,
    seq: Vec<Option<S::Output>>,
    semigroup: S,
}

impl<T: Clone, S: Semigroup<Output = T>> DisjointSparseTable<S> {
    pub fn new(seq: Vec<T>, semigroup: S) -> Self {
        assert!(!seq.is_empty());

        let size = seq.len();
        let log_size = usize::BITS as usize - (size - 1).leading_zeros() as usize;
        let mut data = vec![vec![None; 1 << log_size]; log_size];

        let seq = seq
            .into_iter()
            .map(Some)
            .chain(repeat(None).take((1 << log_size) - size))
            .collect::<Vec<_>>();

        for (i, x) in seq.iter().enumerate() {
            data[0][i] = x.clone();
        }

        let mut ret = Self {
            data,
            seq,
            semigroup,
        };
        ret.build(0, 1 << log_size, log_size - 1);

        ret
    }

    fn build(&mut self, l: usize, r: usize, d: usize) {
        let m = (l + r) / 2;

        self.data[d][m] = self.seq[m].clone();
        for i in m + 1..r {
            self.data[d][i] = match (self.data[d][i - 1].clone(), self.seq[i].clone()) {
                (Some(x), Some(y)) => Some(self.semigroup.op(x, y)),
                (a @ Some(_), None) => a,
                (None, a @ Some(_)) => a,
                (None, None) => None,
            }
        }

        self.data[d][m - 1] = self.seq[m - 1].clone();
        for i in (l..m - 1).rev() {
            self.data[d][i] = match (self.data[d][i + 1].clone(), self.seq[i].clone()) {
                (Some(x), Some(y)) => Some(self.semigroup.op(x, y)),
                (a @ Some(_), None) => a,
                (None, a @ Some(_)) => a,
                (None, None) => None,
            }
        }

        if d > 0 {
            self.build(l, m, d - 1);
            self.build(m, r, d - 1);
        }
    }

    pub fn fold(&self, Range { start: l, end: r }: Range<usize>) -> Option<T> {
        if l == r {
            None
        } else {
            let r = r - 1;

            if l == r {
                self.seq[l].clone()
            } else {
                let k = usize::BITS as usize - 1 - (l ^ r).leading_zeros() as usize;
                match (self.data[k][l].clone(), self.data[k][r].clone()) {
                    (Some(x), Some(y)) => Some(self.semigroup.op(x, y)),
                    (a @ Some(_), None) => a,
                    (None, a @ Some(_)) => a,
                    (None, None) => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::sum::*;
    use crate::testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 100;
        let a = (0..n).map(|_| rng.gen::<u32>() % 10000).collect::<Vec<_>>();

        let s = DisjointSparseTable::new(a.clone(), Sum::<u32>::new());

        for _ in 0..100 {
            let lr = rand_range(&mut rng, 0..n);

            assert_eq!(
                s.fold(lr.clone()),
                if lr.is_empty() {
                    None
                } else {
                    Some(a[lr].iter().fold(0, |acc, &x| acc + x))
                }
            );
        }
    }
}
