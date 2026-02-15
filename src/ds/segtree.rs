//! モノイド列の点更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
pub use crate::algebra::traits::Monoid;
use crate::misc::range::range_bounds_to_range;
use std::ops::{Index, RangeBounds};

/// モノイド列の点更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
#[derive(Clone)]
pub struct Segtree<M: Monoid> {
    monoid: M,
    original_size: usize,
    size: usize,
    data: Vec<M::Element>,
}

impl<M: Monoid> Segtree<M>
where
    M::Element: Clone,
{
    /// **Time complexity** $O(n)$
    pub fn new(monoid: M, n: usize) -> Self {
        let size = n.next_power_of_two() * 2;
        Self {
            original_size: n,
            size,
            data: vec![monoid.id(); size],
            monoid,
        }
    }

    /// モノイド列から`Segtree`を構築する。
    ///
    /// **Time complexity** $O(|s|)$
    pub fn from_vec(monoid: M, s: Vec<M::Element>) -> Self {
        let mut this = Self::new(monoid, s.len());

        for (i, x) in s.iter().enumerate() {
            this.data[i + this.size / 2] = x.clone();
        }

        for i in (1..this.size / 2).rev() {
            this.data[i] = this
                .monoid
                .op(this.data[i << 1].clone(), this.data[(i << 1) | 1].clone());
        }

        this
    }

    /// モノイド列をスライスで返す。
    pub fn to_slice(&self) -> &[M::Element] {
        &self.data[self.size / 2..self.size / 2 + self.original_size]
    }

    /// **Time complexity** $O(\log n)$
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

    /// **Time complexity** $O(\log n)$
    pub fn assign(&mut self, i: usize, value: M::Element) {
        let mut i = i + self.size / 2;
        self.data[i] = value;

        while i > 1 {
            i >>= 1;
            self.data[i] = self
                .monoid
                .op(self.data[i << 1].clone(), self.data[(i << 1) | 1].clone());
        }
    }

    /// **Time complexity** $O(\log n)$
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
    use crate::algebra::bit::BitXor;
    use crate::algebra::matrix::ProdMatrix;
    use crate::algebra::min_max::{Max, Min};
    use crate::algebra::semiring::add_mul_mod::AddMulMod;
    use crate::algebra::sum::Sum;
    use crate::algebra::traits::*;
    use crate::linalg::matrix::MatrixOnSemiring;
    use crate::num::{ff::*, modint::ModIntBuilder};

    use my_testtools::*;
    use rand::Rng;

    fn random_test_helper<M, F>(monoid: M, size: usize, mut gen_value: F)
    where
        M: Monoid + Clone,
        M::Element: Clone + Eq + std::fmt::Debug,
        F: FnMut() -> M::Element,
    {
        let mut rng = rand::thread_rng();

        let mut other = vec![monoid.id(); size];
        let mut s = Segtree::new(monoid.clone(), size);

        for _ in 0..1000 {
            let ty = rng.gen_range(0..2);

            if ty == 0 {
                let i = rng.gen_range(0..size);
                let x = gen_value();

                other[i] = monoid.op(other[i].clone(), x.clone());
                s.update(i, x);
            } else {
                let lr = rand_range(&mut rng, 0..size);

                let ans = other[lr.clone()].iter().cloned().fold_m(&monoid);

                assert_eq!(s.fold(lr), ans);
            }

            let i = rng.gen_range(0..size);
            assert_eq!(s[i], other[i]);
        }

        assert_eq!(Vec::from(&s), other);
    }

    #[test]
    fn test_sum() {
        let mut rng = rand::thread_rng();
        random_test_helper(Sum::<i32>::new(), 10, || rng.gen::<i32>() % 10000);
    }

    #[test]
    fn test_xor() {
        let mut rng = rand::thread_rng();
        random_test_helper(BitXor::<u32>::new(), 10, || rng.gen::<u32>() % 10000);
    }

    #[test]
    fn test_min() {
        let mut rng = rand::thread_rng();
        random_test_helper(Min::<i32>::new(), 10, || rng.gen::<i32>() % 10000);
    }

    #[test]
    fn test_max() {
        let mut rng = rand::thread_rng();
        random_test_helper(Max::<i32>::new(), 10, || rng.gen::<i32>() % 10000);
    }

    #[test]
    fn test_matrix_prod() {
        let mut rng = rand::thread_rng();

        let n = 10;

        let modulo = ModIntBuilder::new(10_u32.pow(9) + 7);
        let ring = AddMulMod(modulo);
        let monoid = ProdMatrix::new(ring, n);

        random_test_helper(monoid, 100, || {
            let mut a = MatrixOnSemiring::zero(ring, n, n);
            for i in 0..n {
                for j in 0..n {
                    *a.get_mut(i, j).unwrap() = modulo.from_u64(rng.gen());
                }
            }
            a
        });
    }
}
