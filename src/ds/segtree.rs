//! モノイド列の点更新・区間取得(*O*(log n), *O*(log n))ができる。
pub use crate::algebra::traits::Monoid;
use crate::utils::range::range_bounds_to_range;
use std::ops::{Index, RangeBounds};

/// モノイド列の点更新・区間取得(*O*(log n), *O*(log n))ができる。
#[derive(Clone)]
pub struct Segtree<M: Monoid> {
    original_size: usize,
    size: usize,
    data: Vec<M::Element>,
    monoid: M,
}

impl<M: Monoid> Segtree<M>
where
    M::Element: Clone,
{
    /// **Time complexity O(n)**
    pub fn new(n: usize, monoid: M) -> Self {
        let size = n.next_power_of_two() * 2;
        Segtree {
            original_size: n,
            size,
            data: vec![monoid.id(); size],
            monoid,
        }
    }

    /// **Time complexity O(log n)**
    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> M::Element {
        let (l, r) = range_bounds_to_range(range, 0, self.size / 2);

        let mut ret_l = self.monoid.id();
        let mut ret_r = self.monoid.id();

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                ret_r = self.monoid.op(self.data[r].clone(), ret_r);
            }
            if l & 1 == 1 {
                ret_l = self.monoid.op(ret_l, self.data[l].clone());
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        self.monoid.op(ret_l, ret_r)
    }

    /// **Time complexity O(log n)**
    pub fn assign(&mut self, i: usize, value: M::Element) {
        let mut i = i + self.size / 2;
        self.data[i] = value;

        while i > 1 {
            i >>= 1;
            self.data[i] = self
                .monoid
                .op(self.data[i << 1].clone(), self.data[i << 1 | 1].clone());
        }
    }

    /// **Time complexity O(log n)**
    pub fn update(&mut self, i: usize, value: M::Element) {
        self.assign(
            i,
            self.monoid.op(self.data[i + self.size / 2].clone(), value),
        );
    }
}

impl<M: Monoid> From<&Segtree<M>> for Vec<M::Element>
where
    M::Element: Clone,
{
    fn from(from: &Segtree<M>) -> Self {
        from.data[from.size / 2..from.size / 2 + from.original_size].to_vec()
    }
}

impl<M: Monoid> Index<usize> for Segtree<M> {
    type Output = M::Element;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[self.size / 2 + i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testtools::*;
    use rand::Rng;

    fn random_test_helper<T, M, F>(size: usize, m: M, mut gen_value: F)
    where
        T: Clone + Eq + std::fmt::Debug,
        M: Monoid<Element = T> + Clone,
        F: FnMut() -> T,
    {
        let mut rng = rand::thread_rng();

        let mut other = vec![m.id().clone(); size];
        let mut s = Segtree::new(size, m.clone());

        for _ in 0..1000 {
            let ty = rng.gen_range(0..2);

            if ty == 0 {
                let i = rng.gen_range(0..size);
                let x = gen_value();

                other[i] = m.op(other[i].clone(), x.clone());
                s.update(i, x);
            } else {
                let lr = rand_range(&mut rng, 0..size);

                let mut temp = m.id().clone();
                for i in lr.clone() {
                    temp = m.op(temp.clone(), other[i].clone());
                }

                assert_eq!(s.fold(lr), temp);
            }

            let i = rng.gen_range(0..size);
            assert_eq!(s[i], other[i]);
        }

        assert_eq!(Vec::<T>::from(&s), other);
    }

    use crate::algebra::bitxor::BitXor;
    use crate::algebra::max::Max;
    use crate::algebra::min::Min;
    use crate::algebra::sum::Sum;

    #[test]
    fn test_sum() {
        let mut rng = rand::thread_rng();
        random_test_helper(10, Sum::<i32>::new(), || rng.gen::<i32>() % 10000);
    }

    #[test]
    fn test_xor() {
        let mut rng = rand::thread_rng();
        random_test_helper(10, BitXor::<u32>::new(), || rng.gen::<u32>() % 10000);
    }

    #[test]
    fn test_min() {
        let mut rng = rand::thread_rng();
        random_test_helper(10, Min::<i32>::new(), || rng.gen::<i32>() % 10000);
    }

    #[test]
    fn test_max() {
        let mut rng = rand::thread_rng();
        random_test_helper(10, Max::<i32>::new(), || rng.gen::<i32>() % 10000);
    }
}
