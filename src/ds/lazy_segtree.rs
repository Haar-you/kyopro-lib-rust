//! モノイド列の区間更新・区間取得(*O*(log n), *O*(log n))ができる。
use crate::algebra::action::Action;
use crate::utils::range::range_bounds_to_range;
use std::ops::RangeBounds;

/// モノイド列の区間更新・区間取得(*O*(log n), *O*(log n))ができる。
pub struct LazySegtree<A: Action> {
    size: usize,
    original_size: usize,
    data: Vec<A::Output>,
    lazy: Vec<A::Lazy>,
    action: A,
}

impl<A: Action + Copy> LazySegtree<A>
where
    A::Output: Clone + PartialEq,
    A::Lazy: Clone + PartialEq,
{
    pub fn new(n: usize, a: A) -> Self {
        let size = n.next_power_of_two() * 2;
        Self {
            size,
            original_size: n,
            data: vec![a.fold_id(); size],
            lazy: vec![a.update_id(); size],
            action: a,
        }
    }

    pub fn new_with_vec(s: Vec<A::Output>, a: A) -> Self {
        let n = s.len();
        let size = n.next_power_of_two() * 2;
        let mut this = Self {
            size,
            original_size: n,
            data: vec![a.fold_id(); size],
            lazy: vec![a.update_id(); size],
            action: a,
        };

        for (i, x) in s.into_iter().enumerate() {
            this.data[size / 2 + i] = x;
        }

        for i in (1..size / 2).rev() {
            this.data[i] = a.fold(this.data[i << 1].clone(), this.data[i << 1 | 1].clone());
        }

        this
    }

    fn propagate(&mut self, i: usize) {
        if self.lazy[i] == self.action.update_id() {
            return;
        }
        if i < self.size / 2 {
            let l = i << 1;
            let r = i << 1 | 1;

            self.lazy[l] = self
                .action
                .update(self.lazy[i].clone(), self.lazy[l].clone());
            self.lazy[r] = self
                .action
                .update(self.lazy[i].clone(), self.lazy[r].clone());
        }
        let len = (self.size / 2) >> (31 - (i as u32).leading_zeros());
        self.data[i] = self
            .action
            .convert(self.data[i].clone(), self.lazy[i].clone(), len);
        self.lazy[i] = self.action.update_id();
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
            self.propagate(i << 1 | 1);
            self.data[i] = self
                .action
                .fold(self.data[i << 1].clone(), self.data[i << 1 | 1].clone());
        }
    }

    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> A::Output {
        let (l, r) = range_bounds_to_range(range, 0, self.original_size);

        self.propagate_top_down(l + self.size / 2);
        if r < self.size / 2 {
            self.propagate_top_down(r + self.size / 2);
        }

        let mut ret_l = self.action.fold_id();
        let mut ret_r = self.action.fold_id();

        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        while l < r {
            if r & 1 == 1 {
                r -= 1;
                self.propagate(r);
                ret_r = self.action.fold(self.data[r].clone(), ret_r.clone());
            }
            if l & 1 == 1 {
                self.propagate(l);
                ret_l = self.action.fold(ret_l.clone(), self.data[l].clone());
                l += 1;
            }
            r >>= 1;
            l >>= 1;
        }

        self.action.fold(ret_l, ret_r)
    }

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
                    self.lazy[r] = self.action.update(x.clone(), self.lazy[r].clone());
                }
                if l & 1 == 1 {
                    self.lazy[l] = self.action.update(x.clone(), self.lazy[l].clone());
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
    use crate::testtools::*;
    use rand::Rng;

    #[test]
    fn add_sum() {
        let n = 100;
        let q = 100;
        let range = 1000;

        let mut seg = LazySegtree::new(n, AddSum::<u64>::new());
        let mut vec = vec![0; n];

        let mut rng = rand::thread_rng();

        for _ in 0..q {
            let lr = rand_range(&mut rng, 0..n);

            match rng.gen::<u32>() % 2 {
                0 => {
                    let x = rng.gen_range(0..range);

                    seg.update(lr.clone(), x);
                    vec[lr].iter_mut().for_each(|y| *y += x);
                }
                1 => {
                    assert_eq!(seg.fold(lr.clone()), vec[lr].iter().sum());
                }
                _ => unreachable!(),
            }
        }
    }
}
