pub use crate::algebra::traits::Monoid;
pub use crate::ds::traits::Updatable;
use std::ops::Range;

pub struct DualSegmentTree<T, M> {
    original_size: usize,
    size: usize,
    data: Vec<T>,
    monoid: M,
}

impl<T, M> DualSegmentTree<T, M>
where
    T: Clone,
    M: Monoid<Output = T>,
{
    pub fn new(n: usize, monoid: M) -> Self {
        let size = n.next_power_of_two() * 2;
        DualSegmentTree {
            original_size: n,
            size,
            data: vec![monoid.id(); size],
            monoid,
        }
    }

    pub fn len(&self) -> usize {
        self.size / 2
    }

    fn propagate(&mut self, i: usize) {
        if i < self.size / 2 {
            self.data[i << 1 | 0] = self
                .monoid
                .op(self.data[i].clone(), self.data[i << 1 | 0].clone());
            self.data[i << 1 | 1] = self
                .monoid
                .op(self.data[i].clone(), self.data[i << 1 | 1].clone());
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

    pub fn index(&mut self, i: usize) -> T {
        self.propagate_top_down(i + self.size / 2);
        self.data[i + self.size / 2].clone()
    }

    pub fn from_vec(&mut self, a: &[T]) {
        self.data = vec![self.monoid.id(); self.size];
        for (i, e) in a.iter().enumerate() {
            self.data[i + self.size / 2] = e.clone();
        }
    }

    pub fn to_vec(&mut self) -> Vec<T> {
        for i in 1..self.size {
            self.propagate(i);
        }

        self.data[self.size / 2..self.size / 2 + self.original_size].to_vec()
    }
}

impl<T, M> Updatable<Range<usize>> for DualSegmentTree<T, M>
where
    T: Clone,
    M: Monoid<Output = T>,
{
    type Value = T;

    fn update(&mut self, Range { start: l, end: r }: Range<usize>, value: Self::Value) {
        let mut l = l + self.size / 2;
        let mut r = r + self.size / 2;

        self.propagate_top_down(l);
        self.propagate_top_down(r);

        while l < r {
            if (r & 1) == 1 {
                r -= 1;
                self.data[r] = self.monoid.op(value.clone(), self.data[r].clone());
            }
            if (l & 1) == 1 {
                self.data[l] = self.monoid.op(value.clone(), self.data[l].clone());
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
    use rand::Rng;

    #[test]
    fn test() {
        let n = 100;
        let m = Sum::<u32>::new();

        let mut a = vec![m.id(); n];
        let mut seg = DualSegmentTree::new(n, m.clone());

        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let l = rng.gen::<usize>() % n;
            let r = l + rng.gen::<usize>() % (n - l) + 1;
            let x = rng.gen::<u32>() % 10000;

            seg.update(l..r, x);
            a[l..r].iter_mut().for_each(|e| *e += x);

            assert_eq!(a, seg.to_vec());
        }
    }
}
