//! モノイド列の区間更新・点取得($O(\log n)$, $O(\log n)$)ができる。
#![allow(clippy::wrong_self_convention)]

pub use crate::algebra::traits::Monoid;
use crate::misc::range::range_bounds_to_range;
use std::cell::RefCell;
use std::ops::RangeBounds;

/// モノイド列の区間更新・点取得($O(\log n)$, $O(\log n)$)ができる。
#[derive(Clone)]
pub struct DualSegtree<M: Monoid> {
    original_size: usize,
    size: usize,
    data: RefCell<Vec<M>>,
}

impl<M: Monoid + Clone> DualSegtree<M> {
    /// **Time complexity** $O(n)$
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two() * 2;
        let data = RefCell::new(vec![M::id(); size]);
        Self {
            original_size: n,
            size,
            data,
        }
    }

    /// モノイド列から`DualSegtree`を構築する。
    ///
    /// **Time complexity** $O(|a|)$
    pub fn from_vec(a: Vec<M>) -> Self {
        let size = a.len().next_power_of_two() * 2;
        let original_size = a.len();
        let mut data = vec![M::id(); size];
        for (i, e) in a.into_iter().enumerate() {
            data[i + size / 2] = e.clone();
        }
        Self {
            original_size,
            size,
            data: RefCell::new(data),
        }
    }

    fn propagate(&self, i: usize) {
        let mut data = self.data.borrow_mut();

        if i < self.size / 2 {
            data[i << 1] = M::op(data[i << 1].clone(), data[i].clone());
            data[(i << 1) | 1] = M::op(data[(i << 1) | 1].clone(), data[i].clone());
            data[i] = M::id();
        }
    }

    fn propagate_top_down(&self, mut i: usize) {
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
    pub fn get(&self, i: usize) -> M {
        self.propagate_top_down(i + self.size / 2);
        self.data.borrow()[i + self.size / 2].clone()
    }

    /// 遅延操作を完了させたモノイド列を`Vec`で返す。
    pub fn to_vec(&self) -> Vec<M> {
        for i in 1..self.size {
            self.propagate(i);
        }

        self.data.borrow()[self.size / 2..self.size / 2 + self.original_size].to_vec()
    }

    /// **Time complexity** $O(\log n)$
    pub fn update(&mut self, range: impl RangeBounds<usize>, value: M) {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        self.propagate_top_down(l);
        self.propagate_top_down(r);

        let mut data = self.data.borrow_mut();

        while l < r {
            if (r & 1) == 1 {
                r -= 1;
                data[r] = M::op(data[r].clone(), value.clone());
            }
            if (l & 1) == 1 {
                data[l] = M::op(data[l].clone(), value.clone());
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
        let mut rng = rand::thread_rng();
        let n = 100;

        let mut a = std::iter::repeat_with(|| {
            let x = rng.gen_range(0..10000);
            Sum(x)
        })
        .take(n)
        .collect::<Vec<_>>();
        let mut seg = DualSegtree::<Sum<u32>>::from_vec(a.clone());

        for _ in 0..100 {
            let lr = rand_range(&mut rng, 0..n);
            let x = rng.gen_range(0..10000);

            seg.update(lr.clone(), Sum(x));
            a[lr].iter_mut().for_each(|e| e.op_assign_r(Sum(x)));

            assert_eq!(a, seg.to_vec());
        }
    }
}
