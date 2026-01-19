//! モノイド列の区間更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
use crate::algebra::{act::Act, traits::*};
use crate::misc::range::range_bounds_to_range;
use std::ops::RangeBounds;

/// モノイド列の区間更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
pub struct LazySegtree<M: Monoid, A: Act<M>> {
    monoid: M,
    act: A,
    size: usize,
    original_size: usize,
    data: Vec<M::Element>,
    lazy: Vec<A::Element>,
}

impl<M, A> LazySegtree<M, A>
where
    M: Monoid,
    A: Act<M>,
    M::Element: Clone + PartialEq,
    A::Element: Clone + PartialEq,
{
    /// 長さ`n`の[`LazySegtree`]を生成する。
    pub fn new(monoid: M, act: A, n: usize) -> Self {
        let size = n.next_power_of_two() * 2;
        Self {
            size,
            original_size: n,
            data: vec![monoid.id(); size],
            lazy: vec![act.monoid().id(); size],
            monoid,
            act,
        }
    }

    /// [`Vec`]から[`LazySegtree`]を構築する。
    ///
    /// **Time complexity** $O(|s|)$
    pub fn from_vec(monoid: M, act: A, s: Vec<M::Element>) -> Self {
        let n = s.len();
        let size = n.next_power_of_two() * 2;
        let mut this = Self {
            size,
            original_size: n,
            data: vec![monoid.id(); size],
            lazy: vec![act.id(); size],
            monoid,
            act,
        };

        for (i, x) in s.into_iter().enumerate() {
            this.data[size / 2 + i] = x;
        }

        for i in (1..size / 2).rev() {
            this.data[i] = this
                .monoid
                .op(this.data[i << 1].clone(), this.data[(i << 1) | 1].clone());
        }

        this
    }

    /// 遅延操作を完了させたモノイド列をスライスで返す。
    ///
    /// **Time complexity** $O(n)$
    pub fn to_slice(&mut self) -> &[M::Element] {
        for i in 1..self.size {
            self.propagate(i);
        }

        &self.data[self.size / 2..self.size / 2 + self.original_size]
    }

    fn propagate(&mut self, i: usize) {
        if self.lazy[i] == self.act.id() {
            return;
        }
        if i < self.size / 2 {
            let l = i << 1;
            let r = (i << 1) | 1;

            self.lazy[l] = self.act.op(self.lazy[l].clone(), self.lazy[i].clone());
            self.lazy[r] = self.act.op(self.lazy[r].clone(), self.lazy[i].clone());
        }
        let len = (self.size / 2) >> (31 - (i as u32).leading_zeros());
        self.data[i] = self.act.act_n(
            &self.monoid,
            self.data[i].clone(),
            self.lazy[i].clone(),
            len,
        );
        self.lazy[i] = self.act.id();
    }

    fn propagate_top_down(&mut self, mut i: usize) {
        let mut temp = vec![i];
        while i > 1 {
            i >>= 1;
            temp.push(i);
        }

        for i in temp.into_iter().rev() {
            self.propagate(i);
        }
    }

    fn bottom_up(&mut self, mut i: usize) {
        while i > 1 {
            i >>= 1;
            self.propagate(i << 1);
            self.propagate((i << 1) | 1);
            self.data[i] = self
                .monoid
                .op(self.data[i << 1].clone(), self.data[(i << 1) | 1].clone());
        }
    }

    /// `i`番目の値を返す。
    pub fn get(&mut self, i: usize) -> M::Element {
        self.propagate_top_down(i + self.size / 2);
        self.data[i + self.size / 2].clone()
    }

    /// 区間`range`で計算を集約して返す。
    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> M::Element {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        self.propagate_top_down(l + self.size / 2);
        if r < self.size / 2 {
            self.propagate_top_down(r + self.size / 2);
        }

        let mut ret_l = self.monoid.id();
        let mut ret_r = self.monoid.id();

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                self.propagate(r);
                ret_r = self.monoid.op(self.data[r].clone(), ret_r.clone());
            }
            if l & 1 == 1 {
                self.propagate(l);
                ret_l = self.monoid.op(ret_l.clone(), self.data[l].clone());
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        self.monoid.op(ret_l, ret_r)
    }

    /// `i`番目の値を`value`で置き換える。
    pub fn assign(&mut self, i: usize, value: M::Element) {
        self.propagate_top_down(i + self.size / 2);
        self.data[i + self.size / 2] = value;
        self.bottom_up(i + self.size / 2);
    }

    /// 区間`range`を値`x`で更新する。
    pub fn update(&mut self, range: impl RangeBounds<usize>, x: A::Element) {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        self.propagate_top_down(l + self.size / 2);
        if r < self.size / 2 {
            self.propagate_top_down(r + self.size / 2);
        }

        {
            let mut l = l + self.size / 2;
            let mut r = r + self.size / 2;

            while l < r {
                if r & 1 == 1 {
                    r -= 1;
                    self.lazy[r] = self.act.op(self.lazy[r].clone(), x.clone());
                }
                if l & 1 == 1 {
                    self.lazy[l] = self.act.op(self.lazy[l].clone(), x.clone());
                    l += 1;
                }
                r >>= 1;
                l >>= 1;
            }
        }

        self.bottom_up(l + self.size / 2);
        if r < self.size / 2 {
            self.bottom_up(r + self.size / 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use my_testtools::*;
    use rand::Rng;

    fn test<M, A, F>(n: usize, q: usize, monoid: M, act: A, mut gen: F)
    where
        M: Monoid<Element: Copy + PartialEq + std::fmt::Debug> + Clone,
        A: Act<M, Element: Copy + PartialEq> + Clone,
        F: FnMut() -> A::Element,
    {
        let mut seg = LazySegtree::new(monoid.clone(), act.clone(), n);
        let mut vec = vec![monoid.id(); n];

        let mut rng = rand::thread_rng();

        for _ in 0..q {
            let lr = rand_range(&mut rng, 0..n);

            match rng.gen::<u32>() % 2 {
                0 => {
                    let x = gen();

                    seg.update(lr.clone(), x);
                    vec[lr]
                        .iter_mut()
                        .for_each(|y| *y = act.act(&monoid, *y, x));
                }
                1 => {
                    assert_eq!(
                        seg.fold(lr.clone()),
                        vec[lr].iter().cloned().fold_m(&monoid)
                    );
                }
                _ => unreachable!(),
            }
        }
    }

    use crate::algebra;

    #[test]
    fn add_sum() {
        let mut rng = rand::thread_rng();
        let m = algebra::sum::Sum::<u64>::new();
        test(100, 100, m, algebra::act::add_sum::AddSum(m), || {
            rng.gen_range(0..1000)
        });
    }

    #[test]
    fn chmax_max() {
        let mut rng = rand::thread_rng();
        let m = algebra::min_max::Max::<i64>::new();
        test(100, 100, m, algebra::act::chmax_max::ChmaxMax(m), || {
            rng.gen_range(-1000..1000)
        });
    }

    #[test]
    fn chmin_min() {
        let mut rng = rand::thread_rng();
        let m = algebra::min_max::Min::<i64>::new();
        test(100, 100, m, algebra::act::chmin_min::ChminMin(m), || {
            rng.gen_range(-1000..1000)
        });
    }
}
