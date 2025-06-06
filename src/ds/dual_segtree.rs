//! モノイド列の区間更新・点取得($O(\log n)$, $O(\log n)$)ができる。
#![allow(clippy::wrong_self_convention)]

pub use crate::algebra::traits::Monoid;
use crate::misc::range::range_bounds_to_range;
use std::ops::RangeBounds;

/// モノイド列の区間更新・点取得($O(\log n)$, $O(\log n)$)ができる。
pub struct DualSegtree<M: Monoid> {
    original_size: usize,
    size: usize,
    data: Vec<M::Element>,
    monoid: M,
}

impl<M: Monoid> DualSegtree<M>
where
    M::Element: Clone,
{
    /// **Time complexity** $O(n)$
    pub fn new(n: usize, monoid: M) -> Self {
        let size = n.next_power_of_two() * 2;
        DualSegtree {
            original_size: n,
            size,
            data: vec![monoid.id(); size],
            monoid,
        }
    }

    fn propagate(&mut self, i: usize) {
        if i < self.size / 2 {
            self.data[i << 1] = self
                .monoid
                .op(self.data[i << 1].clone(), self.data[i].clone());
            self.data[(i << 1) | 1] = self
                .monoid
                .op(self.data[(i << 1) | 1].clone(), self.data[i].clone());
            self.data[i] = self.monoid.id();
        }
    }

    fn propagate_top_down(&mut self, mut i: usize) {
        let mut temp = vec![];
        while i > 1 {
            i >>= 1;
            temp.push(i);
        }

        for &i in temp.iter().rev() {
            self.propagate(i);
        }
    }

    /// **Time complexity** $O(\log n)$
    pub fn get(&mut self, i: usize) -> M::Element {
        self.propagate_top_down(i + self.size / 2);
        self.data[i + self.size / 2].clone()
    }

    /// スライスで初期化する。
    pub fn from_slice(&mut self, a: &[M::Element]) {
        self.data = vec![self.monoid.id(); self.size];
        for (i, e) in a.iter().enumerate() {
            self.data[i + self.size / 2] = e.clone();
        }
    }

    /// 遅延操作を完了させたモノイド列を`Vec`で返す。
    pub fn to_vec(&mut self) -> Vec<M::Element> {
        for i in 1..self.size {
            self.propagate(i);
        }

        self.data[self.size / 2..self.size / 2 + self.original_size].to_vec()
    }

    /// **Time complexity** $O(\log n)$
    pub fn update(&mut self, range: impl RangeBounds<usize>, value: M::Element) {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        self.propagate_top_down(l);
        self.propagate_top_down(r);

        while l < r {
            if (r & 1) == 1 {
                r -= 1;
                self.data[r] = self.monoid.op(self.data[r].clone(), value.clone());
            }
            if (l & 1) == 1 {
                self.data[l] = self.monoid.op(self.data[l].clone(), value.clone());
                l += 1;
            }
            l >>= 1;
            r >>= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::sum::*;
    use my_testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let n = 100;
        let m = Sum::<u32>::new();

        let mut a = vec![m.id(); n];
        let mut seg = DualSegtree::new(n, m);

        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let lr = rand_range(&mut rng, 0..n);
            let x = rng.gen_range(0..10000);

            seg.update(lr.clone(), x);
            a[lr].iter_mut().for_each(|e| *e += x);

            assert_eq!(a, seg.to_vec());
        }
    }
}
