use crate::algebra::traits::Monoid;
use crate::ds::traits::RangeUpdatable;

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
            size: size,
            data: vec![monoid.id(); size],
            monoid: monoid,
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
        for i in 0..a.len() {
            self.data[i + self.size / 2] = a[i].clone();
        }
    }
}

impl<T, M> RangeUpdatable<T> for DualSegmentTree<T, M>
where
    T: Clone,
    M: Monoid<Output = T>,
{
    fn range_update(&mut self, l: usize, r: usize, value: T) {
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
