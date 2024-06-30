//! Convex Hull Trick
//!
//! # Problems
//!
//! - [EDPC Z - Frog 3](https://atcoder.jp/contests/dp/submissions/54932537)

use crate::trait_alias;
use crate::utils::linear::*;
use std::{
    collections::VecDeque,
    ops::{Add, Mul, Sub},
};

trait_alias!(
    Elem,
    Copy + PartialEq + PartialOrd + Sub<Output = Self> + Mul<Output = Self> + Add<Output = Self>
);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Mode {
    Max,
    Min,
}

impl Mode {
    pub(self) fn cmp<T: PartialOrd + Copy>(self, a: T, b: T) -> bool {
        match self {
            Mode::Max => a <= b,
            Mode::Min => a >= b,
        }
    }
}

fn is_needless<T: Elem>(a: &Linear<T>, b: &Linear<T>, c: &Linear<T>) -> bool {
    (a.b - b.b) * (a.a - c.a) >= (a.b - c.b) * (a.a - b.a)
}

#[derive(Clone, Debug)]
pub struct ConvexHullTrick<T> {
    lines: VecDeque<Linear<T>>,
    mode: Mode,
    last_query: Option<T>,
    last_slope: Option<T>,
}

impl<T: Elem> ConvexHullTrick<T> {
    pub fn new(mode: Mode) -> Self {
        Self {
            lines: VecDeque::new(),
            mode,
            last_query: None,
            last_slope: None,
        }
    }

    /// 最小値を求めたいならば、傾きは単調減少でなければならない。
    /// 最大値を求めたいならば、傾きは単調増加でなければならない。
    pub fn add(&mut self, line @ Linear { a, b }: Linear<T>) {
        if let Some(p) = self.last_slope {
            if !self.mode.cmp(p, a) {
                panic!("`a` must be monotonic increasing / decreasing if `mode` is Max / Min");
            }
        }
        self.last_slope = Some(a);

        if let Some(l) = self.lines.back() {
            if l.a == a {
                if !self.mode.cmp(l.b, b) {
                    return;
                }
                self.lines.pop_back();
            }
        }
        while self.lines.len() >= 2
            && is_needless(
                &line,
                self.lines.back().unwrap(),
                self.lines.get(self.lines.len() - 2).unwrap(),
            )
        {
            self.lines.pop_back();
        }

        self.lines.push_back(line);
    }

    /// クエリの座標は単調増加でなければならない。
    pub fn query(&mut self, x: T) -> T {
        if let Some(p) = self.last_query {
            if x < p {
                panic!("x must be monotonic increasing.");
            }
        }
        self.last_query = Some(x);

        while self.lines.len() >= 2
            && self
                .mode
                .cmp(self.lines[0].apply(x), self.lines[1].apply(x))
        {
            self.lines.pop_front();
        }

        self.lines[0].apply(x)
    }
}
