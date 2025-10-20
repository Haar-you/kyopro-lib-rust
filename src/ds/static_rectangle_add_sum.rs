//! 矩形加算矩形総和
//!
//! # References
//! - <https://ei1333.hateblo.jp/entry/2022/06/10/022355>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/static_rectangle_add_rectangle_sum>

use std::ops::{Add, AddAssign, Mul, Neg, Sub};

use crate::{algo::compressor::CompressorBuilder, ds::fenwick_add::*, num::one_zero::*};

struct QAdd<T>(i64, usize, T);
struct QSum<T>(i64, usize, T, usize);

/// 矩形加算矩形総和
///
/// すべての矩形加算クエリを行ってから、矩形総和クエリに答える。
#[derive(Clone, Default)]
pub struct StaticRectangleAddSum<T> {
    add: Vec<(i64, i64, i64, i64, T)>,
    sum: Vec<(i64, i64, i64, i64)>,
}

impl<T> StaticRectangleAddSum<T>
where
    T: Copy
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + AddAssign
        + Zero
        + One
        + From<i64>,
{
    /// 空の`StaticRectangleAddSum`を返す。
    pub fn new() -> Self {
        Self {
            add: vec![],
            sum: vec![],
        }
    }

    /// 矩形加算クエリを追加する。
    pub fn query_add(&mut self, l: i64, d: i64, r: i64, u: i64, w: T) {
        self.add.push((l, d, r, u, w));
    }

    /// 矩形総和クエリを追加する。
    pub fn query_sum(&mut self, l: i64, d: i64, r: i64, u: i64) {
        self.sum.push((l, d, r, u));
    }

    /// 矩形総和クエリの結果を返す。
    pub fn solve(self) -> Vec<T> {
        let mut cp = CompressorBuilder::new();
        cp.extend(self.add.iter().flat_map(|(_, d, _, u, _)| vec![*d, *u]));
        cp.extend(self.sum.iter().flat_map(|(_, d, _, u)| vec![*d, *u]));
        let cp = cp.build();

        let mut add = self
            .add
            .into_iter()
            .flat_map(|(l, d, r, u, w)| {
                let d = cp.index(&d);
                let u = cp.index(&u);
                vec![QAdd(l, d, w), QAdd(r, u, w), QAdd(l, u, -w), QAdd(r, d, -w)]
            })
            .collect::<Vec<_>>();

        add.sort_by_key(|q| q.0);
        add.reverse();

        let mut sum = self
            .sum
            .iter()
            .enumerate()
            .flat_map(|(i, &(l, d, r, u))| {
                let d = cp.index(&d);
                let u = cp.index(&u);
                vec![
                    QSum(l, d, T::one(), i),
                    QSum(r, u, T::one(), i),
                    QSum(l, u, -T::one(), i),
                    QSum(r, d, -T::one(), i),
                ]
            })
            .collect::<Vec<_>>();

        sum.sort_by_key(|q| q.0);
        sum.reverse();

        let mut fxy = FenwickTreeAdd::new(cp.size());
        let mut fx = FenwickTreeAdd::new(cp.size());
        let mut fy = FenwickTreeAdd::new(cp.size());
        let mut f = FenwickTreeAdd::new(cp.size());
        let mut ret = vec![T::zero(); self.sum.len()];

        loop {
            let is_sum = match (add.last(), sum.last()) {
                (None, None) => break,
                (Some(_), None) => false,
                (None, Some(_)) => true,
                (Some(a), Some(s)) => a.0 >= s.0,
            };

            if is_sum {
                let QSum(r, y, c, index) = sum.pop().unwrap();
                let u = *cp.get(y);

                let ans = fxy.fold_to(..y)
                    + f.fold_to(..y) * r.into() * u.into()
                    + fy.fold_to(..y) * r.into()
                    + fx.fold_to(..y) * u.into();

                ret[index] += c * ans;
            } else {
                let QAdd(l, y, w) = add.pop().unwrap();
                let d = *cp.get(y);

                fxy.add(y, w * l.into() * d.into());
                f.add(y, w);
                fy.sub(y, w * d.into());
                fx.sub(y, w * l.into());
            }
        }

        ret
    }
}
