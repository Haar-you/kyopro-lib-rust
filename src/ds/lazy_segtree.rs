//! モノイド列の区間更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
use crate::algebra::action::Action;
use crate::misc::range::range_bounds_to_range;
use std::ops::RangeBounds;

/// モノイド列の区間更新・区間取得($O(\log n)$, $O(\log n)$)ができる。
pub struct LazySegtree<A: Action> {
    action: A,
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
    pub fn new(action: A, n: usize) -> Self {
        let size = n.next_power_of_two() * 2;
        Self {
            size,
            original_size: n,
            data: vec![action.fold_id(); size],
            lazy: vec![action.update_id(); size],
            action,
        }
    }

    /// [`Vec`]から[`LazySegtree`]を構築する。
    ///
    /// **Time complexity** $O(|s|)$
    pub fn from_vec(action: A, s: Vec<A::Output>) -> Self {
        let n = s.len();
        let size = n.next_power_of_two() * 2;
        let mut this = Self {
            size,
            original_size: n,
            data: vec![action.fold_id(); size],
            lazy: vec![action.update_id(); size],
            action,
        };

        for (i, x) in s.into_iter().enumerate() {
            this.data[size / 2 + i] = x;
        }

        for i in (1..size / 2).rev() {
            this.data[i] = this
                .action
                .fold(this.data[i << 1].clone(), this.data[(i << 1) | 1].clone());
        }

        this
    }

    /// 遅延操作を完了させたモノイド列をスライスで返す。
    ///
    /// **Time complexity** $O(n)$
    pub fn to_slice(&mut self) -> &[A::Output] {
        for i in 1..self.size {
            self.propagate(i);
        }

        &self.data[self.size / 2..self.size / 2 + self.original_size]
    }

    fn propagate(&mut self, i: usize) {
        if self.lazy[i] == self.action.update_id() {
            return;
        }
        if i < self.size / 2 {
            let l = i << 1;
            let r = (i << 1) | 1;

            self.lazy[l] = self
                .action
                .update(self.lazy[l].clone(), self.lazy[i].clone());
            self.lazy[r] = self
                .action
                .update(self.lazy[r].clone(), self.lazy[i].clone());
        }
        let len = (self.size / 2) >> (31 - (i as u32).leading_zeros());
        self.data[i] = self
            .action
            .convert(self.data[i].clone(), self.lazy[i].clone(), len);
        self.lazy[i] = self.action.update_id();
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
                .action
                .fold(self.data[i << 1].clone(), self.data[(i << 1) | 1].clone());
        }
    }

    /// `i`番目の値を返す。
    pub fn get(&mut self, i: usize) -> A::Output {
        self.propagate_top_down(i + self.size / 2);
        self.data[i + self.size / 2].clone()
    }

    /// 区間`range`で計算を集約して返す。
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

    /// `i`番目の値を`value`で置き換える。
    pub fn assign(&mut self, i: usize, value: A::Output) {
        self.propagate_top_down(i + self.size / 2);
        self.data[i + self.size / 2] = value;
        self.bottom_up(i + self.size / 2);
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
                    self.lazy[r] = self.action.update(self.lazy[r].clone(), x.clone());
                }
                if l & 1 == 1 {
                    self.lazy[l] = self.action.update(self.lazy[l].clone(), x.clone());
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
    use crate::algebra::action::add_sum::*;
    use crate::algebra::sum::*;
    use my_testtools::*;
    use rand::Rng;

    #[test]
    fn add_sum() {
        let n = 100;
        let q = 100;
        let range = 1000;

        let action = AddSum(Sum::<u64>::new(), Sum::<u64>::new());

        let mut seg = LazySegtree::new(action, n);
        let mut vec = vec![action.fold_id(); n];

        let mut rng = rand::thread_rng();

        for _ in 0..q {
            let lr = rand_range(&mut rng, 0..n);

            match rng.gen::<u32>() % 2 {
                0 => {
                    let x = rng.gen_range(0..range);

                    seg.update(lr.clone(), x);
                    vec[lr]
                        .iter_mut()
                        .for_each(|y| action.monoid_lazy().op_assign_r(y, x));
                }
                1 => {
                    assert_eq!(
                        seg.fold(lr.clone()),
                        vec[lr].iter().cloned().fold_m(action.monoid_output())
                    );
                }
                _ => unreachable!(),
            }
        }
    }
}
