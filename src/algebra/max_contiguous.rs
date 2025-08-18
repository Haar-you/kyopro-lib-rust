//! `bool`値列の結合による、連続する`true`列の最大長
//!
//! # Problems
//! - <https://codeforces.com/contest/484/problem/E>

pub use crate::algebra::traits::*;
use crate::impl_algebra;

use std::cmp::max;

/// 連続する`true`列の最大長を管理する。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct MaxContiguous {
    /// 最大連続長
    pub count: usize,
    /// 左側最大連続長
    pub left: usize,
    /// 右側最大連続長
    pub right: usize,
    /// 区間長
    pub length: usize,
}

impl MaxContiguous {
    /// `value`を値にもつ`MaxContiguous`を生成する。
    pub fn new(value: bool) -> Self {
        let value = if value { 1 } else { 0 };
        Self {
            count: value,
            left: value,
            right: value,
            length: 1,
        }
    }
}

impl_algebra!(
    MaxContiguous;
    op: |a: Self, b: Self| {
        let count = max(a.count, b.count).max(a.right + b.left);
        let left = if a.count == a.length {
            a.count + b.left
        } else {
            a.left
        };
        let right = if b.count == b.length {
            b.count + a.right
        } else {
            b.right
        };
        let length = a.length + b.length;

        Self {
            count, left, right, length
        }
    };
    id: Self { count: 0, left: 0, right: 0, length: 0 };
    assoc;
);
