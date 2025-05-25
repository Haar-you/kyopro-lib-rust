//! モノイド列の区間更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
use crate::algebra::action::Action;
use crate::misc::range::range_bounds_to_range;
use std::ops::RangeBounds;

/// モノイド列の区間更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
pub struct LazySegtree<A: Action> {
    size: usize,
    original_size: usize,
    data: Vec<A::Output>,
    lazy: Vec<A::Lazy>,
}

impl<A: Action> LazySegtree<A>
where
    A::Output: Clone + PartialEq,
    A::Lazy: Clone + PartialEq,
{
    /// 長さ`n`の[`LazySegtree`]を生成する。
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two() * 2;
        Self {
            size,
            original_size: n,
            data: vec![A::fold_id(); size],
            lazy: vec![A::update_id(); size],
        }
    }

    /// [`Vec`]から[`LazySegtree`]を構築する。
    pub fn new_with_vec(s: Vec<A::Output>) -> Self {
        let n = s.len();
        let size = n.next_power_of_two() * 2;
        let mut this = Self {
            size,
            original_size: n,
            data: vec![A::fold_id(); size],
            lazy: vec![A::update_id(); size],
        };

        for (i, x) in s.into_iter().enumerate() {
            this.data[size / 2 + i] = x;
        }

        for i in (1..size / 2).rev() {
            this.data[i] = A::fold(this.data[i << 1].clone(), this.data[(i << 1) | 1].clone());
        }

        this
    }

    fn propagate(&mut self, i: usize) {
        if self.lazy[i] == A::update_id() {
            return;
        }
        if i < self.size / 2 {
            let l = i << 1;
            let r = (i << 1) | 1;

            self.lazy[l] = A::update(self.lazy[l].clone(), self.lazy[i].clone());
            self.lazy[r] = A::update(self.lazy[r].clone(), self.lazy[i].clone());
        }
        let len = (self.size / 2) >> (31 - (i as u32).leading_zeros());
        self.data[i] = A::convert(self.data[i].clone(), self.lazy[i].clone(), len);
        self.lazy[i] = A::update_id();
    }

    fn propagate_top_down(&mut self, mut i: usize) {
        let mut temp = vec![];
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
            self.data[i] = A::fold(self.data[i << 1].clone(), self.data[(i << 1) | 1].clone());
        }
    }

    /// 区間`range`で計算を集約して返す。
    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> A::Output {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        self.propagate_top_down(l + self.size / 2);
        if r < self.size / 2 {
            self.propagate_top_down(r + self.size / 2);
        }

        let mut ret_l = A::fold_id();
        let mut ret_r = A::fold_id();

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                self.propagate(r);
                ret_r = A::fold(self.data[r].clone(), ret_r.clone());
            }
            if l & 1 == 1 {
                self.propagate(l);
                ret_l = A::fold(ret_l.clone(), self.data[l].clone());
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        A::fold(ret_l, ret_r)
    }

    /// 区間`range`を値`x`で更新する。
    pub fn update(&mut self, range: impl RangeBounds<usize>, x: A::Lazy) {
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
                    self.lazy[r] = A::update(self.lazy[r].clone(), x.clone());
                }
                if l & 1 == 1 {
                    self.lazy[l] = A::update(self.lazy[l].clone(), x.clone());
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
    use crate::algebra::add_sum::*;
    use crate::algebra::sum::*;
    use my_testtools::*;
    use rand::Rng;

    #[test]
    fn add_sum() {
        let n = 100;
        let q = 100;
        let range = 1000;

        let mut seg = LazySegtree::<AddSum<u64>>::new(n);
        let mut vec = vec![Sum::id(); n];

        let mut rng = rand::thread_rng();

        for _ in 0..q {
            let lr = rand_range(&mut rng, 0..n);

            match rng.gen::<u32>() % 2 {
                0 => {
                    let x = rng.gen_range(0..range);

                    seg.update(lr.clone(), Sum(x));
                    vec[lr].iter_mut().for_each(|y| y.op_assign(Sum(x)));
                }
                1 => {
                    assert_eq!(seg.fold(lr.clone()), vec[lr].iter().cloned().fold_m());
                }
                _ => unreachable!(),
            }
        }
    }
}
