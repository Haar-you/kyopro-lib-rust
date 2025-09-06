//! 種類数クエリ
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/static_range_count_distinct>
//! - <https://atcoder.jp/contests/abc174/tasks/abc174_f>

use crate::algo::bsearch_slice::BinarySearch;
use crate::ds::fenwick_add::FenwickTreeAdd;

/// 種類数クエリ
pub struct StaticRangeCountQuery {
    data: Vec<usize>,
    qs: Vec<(usize, usize, usize)>,
}

impl StaticRangeCountQuery {
    /// 配列`a`から`StaticRangeCountQuery`を作る。
    pub fn new<T: Clone + Ord>(a: Vec<T>) -> Self {
        let mut temp = a.clone();
        temp.sort();
        temp.dedup();

        let data = a.into_iter().map(|x| temp.lower_bound(&x)).collect();
        Self { data, qs: vec![] }
    }

    /// 範囲`l..r`でのクエリを追加する。
    pub fn add(&mut self, l: usize, r: usize) {
        let i = self.qs.len();
        self.qs.push((r, l, i));
    }

    /// 種類数クエリを解く。
    pub fn solve(mut self) -> Vec<usize> {
        self.qs.sort();
        let n = self.data.len();
        let q = self.qs.len();

        let mut b = FenwickTreeAdd::<usize>::new(n);
        let mut last_index = vec![!0; n];
        let mut ret = vec![0; q];
        let mut cur = 0;

        for (r, l, i) in self.qs {
            while cur < r {
                if last_index[self.data[cur]] != !0 {
                    b.sub(last_index[self.data[cur]], 1);
                }

                last_index[self.data[cur]] = cur;
                b.add(last_index[self.data[cur]], 1);

                cur += 1;
            }

            ret[i] = b.fold(l..r);
        }

        ret
    }
}
