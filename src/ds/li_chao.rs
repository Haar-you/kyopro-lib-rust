use crate::algo::bsearch::lower_bound;
use std::{
    cmp::{max, min},
    mem::swap,
    ops::{Add, Mul},
};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Mode {
    Max,
    Min,
}

impl Mode {
    fn op<T: Copy + Ord>(self, a: T, b: T) -> T {
        match self {
            Mode::Max => max(a, b),
            Mode::Min => min(a, b),
        }
    }

    fn cmp<T: Copy + Ord>(self, a: T, b: T) -> bool {
        match self {
            Mode::Max => a > b,
            Mode::Min => a < b,
        }
    }
}

fn apply<T>(l: (T, T), x: T) -> T
where
    T: Add<Output = T> + Mul<Output = T>,
{
    l.0 * x + l.1
}

pub struct LiChaoTree<T> {
    xs: Vec<T>,
    size: usize,
    data: Vec<Option<(T, T)>>,
    range: Vec<(usize, usize)>,
    mode: Mode,
}

impl<T> LiChaoTree<T>
where
    T: Copy + Ord + Default + Add<Output = T> + Mul<Output = T>,
{
    fn init_range(
        range: &mut Vec<(usize, usize)>,
        size: usize,
        i: usize,
        left: usize,
        right: usize,
    ) {
        if i < size * 2 {
            range[i] = (left, right);
            let mid = (left + right) / 2;
            Self::init_range(range, size, i << 1, left, mid);
            Self::init_range(range, size, i << 1 | 1, mid, right);
        }
    }

    pub fn new(xs_: &[T], mode: Mode) -> Self {
        let mut xs = xs_.to_vec();
        xs.sort();
        xs.dedup();

        let size = xs.len().next_power_of_two();

        xs.resize(size, *xs.last().unwrap());

        let data = vec![None; size * 2];
        let mut range = vec![(0, 0); size * 2];
        Self::init_range(&mut range, size, 1, 0, size);

        Self {
            xs,
            size,
            data,
            range,
            mode,
        }
    }

    fn update(&mut self, i: usize, mut new_line: (T, T), l: usize, r: usize) {
        if let Some(line) = self.data[i] {
            let m = (l + r) / 2;
            let lx = self.xs[l];
            let mx = self.xs[m];
            let rx = self.xs[r - 1];

            let left = self.mode.cmp(apply(new_line, lx), apply(line, lx));
            let mid = self.mode.cmp(apply(new_line, mx), apply(line, mx));
            let right = self.mode.cmp(apply(new_line, rx), apply(line, rx));

            if left && right {
                self.data[i] = Some(new_line);
                return;
            }

            if !left && !right {
                return;
            }

            if mid {
                swap(self.data[i].as_mut().unwrap(), &mut new_line);
            }

            if left != mid {
                self.update(i << 1, new_line, l, m);
            } else {
                self.update(i << 1 | 1, new_line, m, r);
            }
        } else {
            self.data[i] = Some(new_line);
        }
    }

    pub fn add_line(&mut self, line: (T, T)) {
        self.update(1, line, 0, self.size);
    }

    pub fn add_segment(&mut self, segment: (T, T), (l, r): (T, T)) {
        let mut l = lower_bound(&self.xs, &l) + self.size;
        let mut r = lower_bound(&self.xs, &r) + self.size;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                self.update(r, segment, self.range[r].0, self.range[r].1);
            }
            if l & 1 == 1 {
                self.update(l, segment, self.range[l].0, self.range[l].1);
                l += 1;
            }

            r >>= 1;
            l >>= 1;
        }
    }

    pub fn query(&self, x: T) -> Option<T> {
        let i = self.xs.binary_search(&x).expect("`x` must be in `xs`");
        let mut k = i + self.size;
        let mut ret = None;

        while k > 0 {
            if let Some(line) = self.data[k] {
                let a = apply(line, self.xs[i]);
                ret = Some(ret.map_or(a, |y| self.mode.op(y, a)));
            }

            k >>= 1;
        }

        ret
    }
}
