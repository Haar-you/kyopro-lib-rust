use crate::algebra::action::Action;
use std::ops::Range;

pub struct LazySegtree<T, U, A> {
    size: usize,
    data: Vec<T>,
    lazy: Vec<U>,
    action: A,
}

impl<T: Clone + Eq, U: Clone + Eq, A: Clone + Action<FType = T, UType = U>> LazySegtree<T, U, A> {
    pub fn new(n: usize, a: A) -> Self {
        let size = n.next_power_of_two() * 2;
        Self {
            size,
            data: vec![a.fold_id(); size],
            lazy: vec![a.update_id(); size],
            action: a,
        }
    }

    fn propagate(&mut self, i: usize) {
        if self.lazy[i] == self.action.update_id() {
            return;
        }
        if i < self.size / 2 {
            self.lazy[i << 1] = self
                .action
                .update(self.lazy[i].clone(), self.lazy[i << 1].clone());
            self.lazy[i << 1 | 1] = self
                .action
                .update(self.lazy[i].clone(), self.lazy[i << 1 | 1].clone());
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

        for &i in temp.iter().rev() {
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

    pub fn fold(&mut self, Range { start: l, end: r }: Range<usize>) -> T {
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

    pub fn update(&mut self, Range { start: l, end: r }: Range<usize>, x: U) {
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
                    self.propagate(r);
                }
                if l & 1 == 1 {
                    self.lazy[l] = self.action.update(x.clone(), self.lazy[l].clone());
                    self.propagate(l);
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

        let mut seg = LazySegtree::new(n, AddSum::<u64, u64>::new());
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
