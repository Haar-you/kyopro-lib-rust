//! 列の中で同じ値が連続する最大長を管理する。
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc415/tasks/abc415_f>

use crate::algebra::traits::*;
use crate::impl_algebra;

/// 同じ値が連続する最大長を管理する。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaxContiguousMany<T> {
    /// 最大連続長と値
    pub max: (usize, T),
    /// 左端からの最大連続長と値
    pub left: (usize, T),
    /// 右端からの最大連続長と値
    pub right: (usize, T),
    /// 列の長さ
    pub length: usize,
}

impl<T: Copy> MaxContiguousMany<T> {
    /// 値`value`をただ一つだけもつ列。
    pub fn unit(value: T) -> Self {
        Self {
            max: (1, value),
            left: (1, value),
            right: (1, value),
            length: 1,
        }
    }
}

fn max<T>(a: (usize, T), b: (usize, T)) -> (usize, T) {
    if a.0 >= b.0 {
        a
    } else {
        b
    }
}

fn join<T: Eq>(a: (usize, T), b: (usize, T)) -> (usize, T) {
    if a.1 == b.1 {
        (a.0 + b.0, a.1)
    } else {
        max(a, b)
    }
}

impl<T: Copy + Eq> BinaryOp for MaxContiguousMany<T> {
    fn op(self, other: Self) -> Self {
        let (a, b) = (self, other);

        let max = max(max(a.max, b.max), join(a.right, b.left));

        let left = if a.left.0 == a.length && a.left.1 == b.left.1 {
            (a.left.0 + b.left.0, a.left.1)
        } else {
            a.left
        };

        let right = if b.right.0 == b.length && a.right.1 == b.right.1 {
            (a.right.0 + b.right.0, b.right.1)
        } else {
            b.right
        };

        let length = a.length + b.length;

        Self {
            max,
            left,
            right,
            length,
        }
    }
}

impl_algebra!(
    [T: Copy + Eq]; MaxContiguousMany<T>;
    assoc;
);
