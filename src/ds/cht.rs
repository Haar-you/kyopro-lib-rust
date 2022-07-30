use std::{
    collections::VecDeque,
    ops::{Add, Mul, Sub},
};

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

fn is_needless<T>(a: &(T, T), b: &(T, T), c: &(T, T)) -> bool
where
    T: Sub<Output = T> + Mul<Output = T> + PartialOrd + Copy,
{
    (a.1 - b.1) * (a.0 - c.0) >= (a.1 - c.1) * (a.0 - b.0)
}

fn apply<T>(l: (T, T), x: T) -> T
where
    T: Add<Output = T> + Mul<Output = T>,
{
    l.0 * x + l.1
}

#[derive(Clone, Debug)]
pub struct ConvexHullTrick<T> {
    lines: VecDeque<(T, T)>,
    mode: Mode,
    last_query: Option<T>,
    last_slope: Option<T>,
}

impl<T> ConvexHullTrick<T>
where
    T: Copy + PartialEq + PartialOrd + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    pub fn new(mode: Mode) -> Self {
        Self {
            lines: VecDeque::new(),
            mode,
            last_query: None,
            last_slope: None,
        }
    }

    pub fn add(&mut self, (a, b): (T, T)) {
        if let Some(p) = self.last_slope {
            if !self.mode.cmp(p, a) {
                panic!("`a` must be monotonic increasing / decreasing if `mode` is Max / Min");
            }
        }
        self.last_slope = Some(a);

        if let Some(l) = self.lines.back() {
            if l.0 == a {
                if !self.mode.cmp(l.1, b) {
                    return;
                }
                self.lines.pop_back();
            }
        }
        while self.lines.len() >= 2
            && is_needless(
                &(a, b),
                self.lines.back().unwrap(),
                self.lines.get(self.lines.len() - 2).unwrap(),
            )
        {
            self.lines.pop_back();
        }

        self.lines.push_back((a, b));
    }

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
                .cmp(apply(self.lines[0], x), apply(self.lines[1], x))
        {
            self.lines.pop_front();
        }

        apply(self.lines[0], x)
    }
}
