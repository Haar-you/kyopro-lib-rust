pub use crate::ds::traits::IndexableMut;
use std::{
    mem::size_of,
    ops::{Add, Mul, Range},
};

pub struct SegmentTreeLinearAdd<T> {
    hsize: usize,
    data: Vec<(T, T)>,
    from: Vec<usize>,
    zero: T,
}

impl<T> SegmentTreeLinearAdd<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + From<u32>,
{
    pub fn new(n: usize, zero: T) -> Self {
        let size = n.next_power_of_two() * 2;
        let hsize = size / 2;
        let mut from = vec![0; size];

        let mut s = 0;
        for (i, x) in from.iter_mut().enumerate().skip(1) {
            *x = s;
            let l = hsize >> (size_of::<usize>() as u32 * 8 - 1 - i.leading_zeros());
            s += l;
            if s == hsize {
                s = 0;
            }
        }

        Self {
            hsize,
            data: vec![(zero, zero); size],
            from,
            zero,
        }
    }

    pub fn update(&mut self, Range { start: l, end: r }: Range<usize>, (a, b): (T, T)) {
        let mut l_ = l + self.hsize;
        let mut r_ = r + self.hsize;

        while l_ < r_ {
            if r_ & 1 == 1 {
                r_ -= 1;
                self.data[r_] = Self::add(
                    self.data[r_],
                    (b + a * T::from((self.from[r_] - l) as u32), a),
                );
            }
            if l_ & 1 == 1 {
                self.data[l_] = Self::add(
                    self.data[l_],
                    (b + a * T::from((self.from[l_] - l) as u32), a),
                );
                l_ += 1;
            }

            l_ >>= 1;
            r_ >>= 1;
        }
    }

    fn add((a, b): (T, T), (c, d): (T, T)) -> (T, T) {
        (a + c, b + d)
    }

    fn propagate(&mut self, i: usize) {
        if i < self.hsize {
            self.data[i << 1] = Self::add(self.data[i << 1], self.data[i]);

            let len = self.hsize >> (size_of::<usize>() as u32 * 8 - i.leading_zeros());
            self.data[i].0 = self.data[i].0 + self.data[i].1 * T::from(len as u32);
            self.data[i << 1 | 1] = Self::add(self.data[i << 1 | 1], self.data[i]);

            self.data[i] = (self.zero, self.zero);
        }
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
}

impl<T> IndexableMut<usize> for SegmentTreeLinearAdd<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + From<u32>,
{
    type Output = T;

    fn get(&mut self, i: usize) -> Self::Output {
        self.propagate_top_down(i + self.hsize);
        self.data[i + self.hsize].0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 100;
        let mut seg = SegmentTreeLinearAdd::<u64>::new(n, 0);
        let mut vec = vec![0; n];

        for _ in 0..300 {
            let l = rng.gen::<usize>() % n;
            let r = l + rng.gen::<usize>() % (n - l) + 1;

            let a = rng.gen::<u64>() % 100;
            let b = rng.gen::<u64>() % 100;

            seg.update(l..r, (a as u64, b as u64));

            for (k, i) in (l..r).enumerate() {
                vec[i] += a * k as u64 + b;
            }

            assert_eq!((0..n).map(|i| seg.get(i)).collect::<Vec<_>>(), vec);
        }
    }
}
