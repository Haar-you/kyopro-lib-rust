//! 範囲転倒数取得クエリ

use crate::{algo::bsearch::*, ds::fenwick_add::*};
use std::convert::TryFrom;
use std::ops::Range;

pub struct StaticRangeInversionsQuery {
    data: Vec<usize>,
    qs: Vec<(usize, usize)>,
}

impl StaticRangeInversionsQuery {
    /// **Time complexity** $O(n \log n)$
    pub fn new<T: Clone + Ord>(data: &[T]) -> Self {
        let mut a = data.to_vec();
        a.sort();
        a.dedup();

        let data = data.iter().map(|x| lower_bound(&a, x)).collect();
        Self { data, qs: vec![] }
    }

    pub fn add_query(&mut self, Range { start: l, end: r }: Range<usize>) {
        self.qs.push((l, r));
    }

    pub fn solve(&self) -> Vec<u64> {
        let n = self.data.len();
        let width = (n as f64).sqrt() as usize;

        let mut b = FenwickTreeAdd::<i64>::new(n);
        let mut temp = 0;
        let mut ret = vec![0; self.qs.len()];

        let mut ord = (0..self.qs.len()).collect::<Vec<_>>();

        ord.sort_by(|&i, &j| {
            let a = self.qs[i].0 / width;
            let b = self.qs[j].0 / width;

            if a == b {
                if a % 2 == 1 {
                    self.qs[i].1.cmp(&self.qs[j].1)
                } else {
                    self.qs[j].1.cmp(&self.qs[i].1)
                }
            } else {
                a.cmp(&b)
            }
        });

        let mut l = self.qs[ord[0]].0;
        let mut r = self.qs[ord[0]].0;

        for id in ord {
            while l != self.qs[id].0 || r != self.qs[id].1 {
                if l > self.qs[id].0 {
                    l -= 1;
                    temp += b.fold(0..self.data[l]);
                    b.add(self.data[l], 1);
                }
                if l < self.qs[id].0 {
                    temp -= b.fold(0..self.data[l]);
                    b.add(self.data[l], -1);
                    l += 1;
                }
                if r < self.qs[id].1 {
                    temp += b.fold(self.data[r] + 1..self.data.len());
                    b.add(self.data[r], 1);
                    r += 1;
                }
                if r > self.qs[id].1 {
                    r -= 1;
                    temp -= b.fold(self.data[r] + 1..self.data.len());
                    b.add(self.data[r], -1);
                }
            }

            ret[id] = u64::try_from(temp).unwrap()
        }

        ret
    }
}
