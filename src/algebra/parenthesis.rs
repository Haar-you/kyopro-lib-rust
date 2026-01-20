//! 括弧列
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc223/tasks/abc223_f>
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 括弧列
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ParenSeq {
    /// 対応する`(`がない`)`の個数。
    pub close: u64,
    /// 対応する`)`がない`(`の個数。
    pub open: u64,
}

impl ParenSeq {
    /// 空の括弧列を返す。
    pub fn empty() -> Self {
        Self { close: 0, open: 0 }
    }

    /// 括弧列が正しい括弧列かを判定する。
    pub fn is_correct(self) -> bool {
        self.close == 0 && self.open == 0
    }

    /// 連続した`n`個の開いた括弧`(`からなる列。
    pub fn open(n: u64) -> Self {
        Self { close: 0, open: n }
    }

    /// 連続した`n`個の閉じた括弧`)`からなる列。
    pub fn close(n: u64) -> Self {
        Self { close: n, open: 0 }
    }

    /// 括弧列`self`の右に括弧列`right`を結合して、対応のとれた括弧対をすべて潰した括弧列を返す。
    pub fn concat(self, right: Self) -> Self {
        let Self { close: a, open: b } = self;
        let Self { close: c, open: d } = right;

        Self {
            close: a + c.saturating_sub(b),
            open: d + b.saturating_sub(c),
        }
    }
}

/// 括弧列の結合を演算とするモノイド
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Composition;

impl_algebra!(Composition; set: ParenSeq; op: |_, a: ParenSeq, b| a.concat(b);
              id: |_| ParenSeq::empty(), |_, a: &ParenSeq| a.is_correct();
              assoc;
);
