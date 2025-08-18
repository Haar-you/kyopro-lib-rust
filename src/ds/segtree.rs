//! モノイド列の点更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
pub use crate::algebra::traits::Monoid;
use crate::misc::range::range_bounds_to_range;
use std::ops::{Index, RangeBounds};

/// モノイド列の点更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
#[derive(Clone)]
pub struct Segtree<M: Monoid> {
    original_size: usize,
    size: usize,
    data: Vec<M>,
}

impl<M: Monoid + Clone> Segtree<M> {
    /// **Time complexity** $O(n)$
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two() * 2;
        Segtree {
            original_size: n,
            size,
            data: vec![M::id(); size],
        }
    }

    /// モノイド列から`Segtree`を構築する。
    ///
    /// **Time complexity** $O(|s|)$
    pub fn from_vec(s: Vec<M>) -> Self {
        let mut this = Self::new(s.len());

        for (i, x) in s.iter().enumerate() {
            this.data[i + this.size / 2] = x.clone();
        }

        for i in (1..this.size / 2).rev() {
            this.data[i] = this.data[i << 1]
                .clone()
                .op(this.data[(i << 1) | 1].clone());
        }

        this
    }

    /// モノイド列をスライスで返す。
    pub fn to_slice(&self) -> &[M] {
        &self.data[self.size / 2..self.size / 2 + self.original_size]
    }

    /// **Time complexity** $O(\log n)$
    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> M {
        let (l, r) = range_bounds_to_range(range, 0, self.size / 2);

        let mut ret_l = M::id();
        let mut ret_r = M::id();

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                ret_r = M::op(self.data[r].clone(), ret_r);
            }
            if l & 1 == 1 {
                ret_l = M::op(ret_l, self.data[l].clone());
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        M::op(ret_l, ret_r)
    }

    /// **Time complexity** $O(\log n)$
    pub fn assign(&mut self, i: usize, value: M) {
        let mut i = i + self.size / 2;
        self.data[i] = value;

        while i > 1 {
            i >>= 1;
            self.data[i] = M::op(self.data[i << 1].clone(), self.data[(i << 1) | 1].clone());
        }
    }

    /// **Time complexity** $O(\log n)$
    pub fn update(&mut self, i: usize, value: M) {
        self.assign(i, M::op(self.data[i + self.size / 2].clone(), value));
    }
}

impl<M: Monoid + Clone> From<&Segtree<M>> for Vec<M> {
    fn from(from: &Segtree<M>) -> Self {
        from.data[from.size / 2..from.size / 2 + from.original_size].to_vec()
    }
}

impl<M: Monoid> Index<usize> for Segtree<M> {
    type Output = M;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[self.size / 2 + i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::traits::*;
    use my_testtools::*;
    use rand::Rng;

    fn random_test_helper<M, F>(size: usize, mut gen_value: F)
    where
        M: Monoid + Clone + Eq + std::fmt::Debug,
        F: FnMut() -> M,
    {
        let mut rng = rand::thread_rng();

        let mut other = vec![M::id(); size];
        let mut s = Segtree::new(size);

        for _ in 0..1000 {
            let ty = rng.gen_range(0..2);

            if ty == 0 {
                let i = rng.gen_range(0..size);
                let x = gen_value();

                other[i] = M::op(other[i].clone(), x.clone());
                s.update(i, x);
            } else {
                let lr = rand_range(&mut rng, 0..size);

                let ans = other[lr.clone()].iter().cloned().fold_m();

                assert_eq!(s.fold(lr), ans);
            }

            let i = rng.gen_range(0..size);
            assert_eq!(s[i], other[i]);
        }

        assert_eq!(Vec::<M>::from(&s), other);
    }

    use crate::algebra::bit::BitXor;
    use crate::algebra::min_max::{Max, Min};
    use crate::algebra::sum::Sum;

    #[test]
    fn test_sum() {
        let mut rng = rand::thread_rng();
        random_test_helper(10, || Sum(rng.gen::<i32>() % 10000));
    }

    #[test]
    fn test_xor() {
        let mut rng = rand::thread_rng();
        random_test_helper(10, || BitXor(rng.gen::<u32>() % 10000));
    }

    #[test]
    fn test_min() {
        let mut rng = rand::thread_rng();
        random_test_helper(10, || Min(rng.gen::<i32>() % 10000));
    }

    #[test]
    fn test_max() {
        let mut rng = rand::thread_rng();
        random_test_helper(10, || Max(rng.gen::<i32>() % 10000));
    }
}
