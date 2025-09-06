//! 最頻値取得クエリ
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/static_range_mode_query>

use crate::{algo::bsearch_slice::BinarySearch, misc::range::range_bounds_to_range};
use std::ops::RangeBounds;

/// 最頻値取得クエリ
pub struct StaticRangeModeQuery<T> {
    d: Vec<T>,
    b: Vec<usize>,
    b_index: Vec<usize>,
    block_size: usize,
    n: usize,
    index: Vec<Vec<usize>>,
    mode: Vec<Vec<usize>>,
    freq: Vec<Vec<usize>>,
}

impl<T: Clone + Ord> StaticRangeModeQuery<T> {
    /// 配列`a`での最頻値クエリを構築する。
    pub fn new(a: Vec<T>) -> Self {
        let mut d = a.clone();
        let n = a.len();
        let block_size = n.isqrt();
        let block_num = n.div_ceil(block_size);
        let mut mode = vec![vec![0; block_num]; block_num];
        let mut freq = vec![vec![0; block_num]; block_num];

        d.sort();
        d.dedup();
        let k = d.len();

        let b = a.iter().map(|x| d.lower_bound(x)).collect::<Vec<_>>();

        let mut index = vec![vec![]; k];
        let mut b_index = vec![0; n];
        for i in 0..n {
            b_index[i] = index[b[i]].len();
            index[b[i]].push(i);
        }

        for i in 0..block_num {
            let mut temp = vec![0; k];
            let mut md = 0;
            let mut fr = 0;

            for j in i..block_num {
                let r = n.min(block_size * (j + 1));

                for x in block_size * j..r {
                    temp[b[x]] += 1;

                    if temp[b[x]] > fr {
                        md = b[x];
                        fr = temp[b[x]];
                    }
                }

                mode[i][j] = md;
                freq[i][j] = fr;
            }
        }

        Self {
            d,
            b,
            n,
            b_index,
            block_size,
            index,
            mode,
            freq,
        }
    }

    /// 範囲`range`での最頻値とその出現回数を返す。
    pub fn query(&self, range: impl RangeBounds<usize>) -> (T, usize) {
        let Self {
            d,
            b,
            b_index,
            block_size,
            n,
            index,
            mode,
            freq,
        } = self;

        let (l, r) = range_bounds_to_range(range, 0, *n);

        let mut ret = (None, 0);

        let span_l = l.div_ceil(*block_size);
        let span_r = r / block_size;

        if span_l < span_r {
            ret = (
                Some(d[mode[span_l][span_r - 1]].clone()),
                freq[span_l][span_r - 1],
            );
        }

        // prefix
        for i in l..r.min(span_l * block_size) {
            if b_index[i] >= 1 && index[b[i]][b_index[i] - 1] >= l {
                continue;
            }

            if b_index[i] + ret.1 < 1
                || index[b[i]]
                    .get(b_index[i] + ret.1 - 1)
                    .is_some_and(|&x| x < r)
            {
                let mut fr = ret.1;
                for j in b_index[i] + ret.1..index[b[i]].len() {
                    if index[b[i]][j] < r {
                        fr += 1;
                    } else {
                        break;
                    }
                }

                if fr > ret.1 {
                    ret = (Some(d[b[i]].clone()), fr);
                }
            }
        }

        // suffix
        for i in (l.max(span_r * block_size)..r).rev() {
            if index[b[i]].get(b_index[i] + 1).is_some_and(|&x| x < r) {
                continue;
            }

            if b_index[i] + 1 >= ret.1
                && index[b[i]]
                    .get(b_index[i] - ret.1 + 1)
                    .is_some_and(|&x| x >= l)
            {
                let mut fr = ret.1;

                if b_index[i] >= ret.1 {
                    for j in (0..=self.b_index[i] - ret.1).rev() {
                        if index[b[i]][j] >= l {
                            fr += 1;
                        } else {
                            break;
                        }
                    }
                }

                if fr > ret.1 {
                    ret = (Some(d[b[i]].clone()), fr);
                }
            }
        }

        let (m, f) = ret;
        (m.unwrap(), f)
    }
}
