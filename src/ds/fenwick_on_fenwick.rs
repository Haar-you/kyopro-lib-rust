use crate::algo::bsearch::lower_bound;
use std::ops::{Add, Range, RangeTo, Sub};

#[derive(Clone, Default)]
pub struct FenwickOnFenwickBuilder {
    xs: Vec<i64>,
    ys: Vec<i64>,
}

#[derive(Clone)]
pub struct FenwickOnFenwick<T> {
    c_xs: Vec<i64>,
    c_ys: Vec<Vec<i64>>,
    segs: Vec<Vec<T>>,
    zero: T,
}

impl FenwickOnFenwickBuilder {
    pub fn new() -> Self {
        Self {
            xs: vec![],
            ys: vec![],
        }
    }

    /// 使用する点を登録する。
    pub fn add(&mut self, x: i64, y: i64) {
        self.xs.push(x);
        self.ys.push(y);
    }

    pub fn build<T: Copy>(self, zero: T) -> FenwickOnFenwick<T> {
        let n = self.xs.len();
        let mut c_xs = self.xs.clone();
        c_xs.sort_unstable();
        c_xs.dedup();

        let x_size = c_xs.len();

        let mut c_ys = vec![vec![]; x_size + 1];

        let mut ord = (0..n).collect::<Vec<usize>>();
        ord.sort_by(|&i, &j| self.ys[i].cmp(&self.ys[j]));

        for i in ord {
            let mut x = c_xs.binary_search(&self.xs[i]).unwrap() + 1;

            while x <= x_size {
                c_ys[x].push(self.ys[i]);
                x += x & (!x + 1);
            }
        }

        let mut segs = vec![vec![]];

        for ys in c_ys.iter_mut().take(x_size + 1).skip(1) {
            ys.dedup();
            segs.push(vec![zero; ys.len() + 1]);
        }

        FenwickOnFenwick {
            c_xs,
            c_ys,
            segs,
            zero,
        }
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> FenwickOnFenwick<T> {
    /// Time Complexity $O(\log ^ 2 n)$
    pub fn update(&mut self, x: i64, y: i64, value: T) {
        let mut x = self.c_xs.binary_search(&x).unwrap() + 1;

        while x <= self.c_xs.len() {
            let mut y = self.c_ys[x].binary_search(&y).unwrap() + 1;

            while y < self.segs[x].len() {
                self.segs[x][y] = self.segs[x][y] + value;
                y += y & (!y + 1);
            }
            x += x & (!x + 1);
        }
    }

    /// Time Complexity $O(\log ^ 2 n)$
    pub fn fold_2d(
        &self,
        Range { start: x1, end: x2 }: Range<i64>,
        Range { start: y1, end: y2 }: Range<i64>,
    ) -> T {
        self.fold_to_2d(..x2, ..y2) - self.fold_to_2d(..x1, ..y2) - self.fold_to_2d(..x2, ..y1)
            + self.fold_to_2d(..x1, ..y1)
    }

    /// Time Complexity $O(\log ^ 2 n)$
    pub fn fold_to_2d(
        &self,
        RangeTo { end: x }: RangeTo<i64>,
        RangeTo { end: y }: RangeTo<i64>,
    ) -> T {
        let mut ret = self.zero;
        let mut x = lower_bound(&self.c_xs, &x);

        while x > 0 {
            let mut y = lower_bound(&self.c_ys[x], &y);
            let seg = &self.segs[x];

            while y > 0 {
                ret = ret + seg[y];
                y -= y & (!y + 1);
            }
            x -= x & (!x + 1);
        }

        ret
    }
}
