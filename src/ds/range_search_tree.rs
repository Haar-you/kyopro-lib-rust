//! 領域内の点を列挙する
//!
//! # Problems
//! - [AOJ DSL 2_C: Range Search(kD Tree)](https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_C)

use crate::algo::{bsearch::lower_bound, merge::merge};

/// Range search tree
pub struct RangeSearchTree<Index> {
    size: usize,
    cxs: Vec<Index>,
    data: Vec<Vec<(Index, usize)>>,
}

impl<Index> RangeSearchTree<Index>
where
    Index: Copy + Ord,
{
    /// $[sx, tx), [sy, ty)$の矩形内部にある点を列挙する。
    pub fn search(
        &self,
        (sx, sy): (Index, Index),
        (tx, ty): (Index, Index),
    ) -> Vec<(Index, Index)> {
        assert!(sx < tx);
        assert!(sy < ty);

        let mut ret = vec![];
        let mut l = lower_bound(&self.cxs, &sx) + self.size / 2;
        let mut r = lower_bound(&self.cxs, &tx) + self.size / 2;

        while l < r {
            if (r & 1) != 0 {
                r -= 1;
                let a = &self.data[r];

                let i = lower_bound(a, &(sy, 0));

                for &(y, x) in a.iter().skip(i).take_while(|(y, _)| *y < ty) {
                    ret.push((self.cxs[x], y));
                }
            }

            if (l & 1) != 0 {
                let a = &self.data[l];
                l += 1;

                let i = lower_bound(a, &(sy, 0));

                for &(y, x) in a.iter().skip(i).take_while(|(y, _)| *y < ty) {
                    ret.push((self.cxs[x], y));
                }
            }

            l >>= 1;
            r >>= 1;
        }

        ret
    }
}

/// [`RangeSearchTree`]を構築するための構造体。
#[derive(Clone, Default)]
pub struct RangeSearchTreeBuilder<Index> {
    size: usize,
    xs: Vec<Index>,
    ys: Vec<Index>,
}

impl<Index> RangeSearchTreeBuilder<Index>
where
    Index: Copy + Ord,
{
    /// 空の[`RangeSearchTreeBuilder`]を用意する。
    pub fn new() -> Self {
        Self {
            size: 0,
            xs: vec![],
            ys: vec![],
        }
    }

    /// 点`(x, y)`を登録する。
    pub fn add(&mut self, x: Index, y: Index) {
        self.size += 1;
        self.xs.push(x);
        self.ys.push(y);
    }

    /// [`RangeSearchTree`]を構築する。
    pub fn build(self) -> RangeSearchTree<Index> {
        let mut cxs = self.xs.clone();
        cxs.sort_unstable();
        cxs.dedup();

        let m = cxs.len();
        let size = m.next_power_of_two() * 2;

        let mut data: Vec<Vec<(Index, usize)>> = vec![vec![]; size];

        for i in 0..self.size {
            let j = lower_bound(&cxs, &self.xs[i]);
            data[size / 2 + j].push((self.ys[i], j));
        }

        for item in data.iter_mut().take(size).skip(size / 2) {
            item.sort_unstable();
        }

        for i in (1..size / 2).rev() {
            data[i] = merge(data[i << 1].clone(), data[(i << 1) | 1].clone());
        }

        RangeSearchTree { size, cxs, data }
    }
}
