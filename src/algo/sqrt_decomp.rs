//! 平方分割
//!
//! # Problems
//! - <https://onlinejudge.u-aizu.ac.jp/problems/3170>

use std::ops::Range;

/// 平方分割
pub struct SqrtDecomposition {
    size: usize,
    block_size: usize,
    block_num: usize,
}

impl SqrtDecomposition {
    /// 大きさ`size`の列の平方分割の準備をする。
    pub fn new(size: usize) -> Self {
        let block_size = size.isqrt();
        let block_num = size.div_ceil(block_size);

        Self {
            size,
            block_size,
            block_num,
        }
    }

    /// 各ブロックで初期化を施す。
    pub fn init(&self, mut init: impl FnMut(usize, Range<usize>)) {
        for i in 0..self.block_num {
            let l = i * self.block_size;
            let r = self.size.min((i + 1) * self.block_size);
            init(i, l..r);
        }
    }

    /// 分割したブロック数を返す。
    pub fn block_num(&self) -> usize {
        self.block_num
    }

    /// `range`の範囲でのクエリを行う。
    ///
    /// `f`には、ブロックの番号、ブロックの範囲、ブロックの部分クエリの範囲(`None`のときブロックの全体クエリ、`Some`のときブロックの部分クエリ)が与えられる。
    pub fn query<F>(&self, range: Range<usize>, mut f: F)
    where
        F: FnMut(usize, Range<usize>, Option<Range<usize>>),
    {
        let Range { start: l, end: r } = range;

        for i in 0..self.block_num {
            let b_l = i * self.block_size;
            let b_r = self.size.min((i + 1) * self.block_size);

            if l <= b_l && b_r <= r {
                f(i, b_l..b_r, None);
            } else if (b_l <= l && l < b_r) || (b_l < r && r <= b_r) {
                f(i, b_l..b_r, Some(l.max(b_l)..r.min(b_r)));
            }
        }
    }
}
