use crate::num::{one_zero::Zero, traits::Signed};
use std::ops::{Add, Range, Sub};

pub struct Imos1D<T> {
    data: Vec<T>,
}

impl<T: Copy + Signed + Zero<Output = T> + Add<Output = T> + Sub<Output = T>> Imos1D<T> {
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![T::zero(); n],
        }
    }

    pub fn update(&mut self, Range { start: l, end: r }: Range<usize>, value: T) {
        self.data[l] = self.data[l] + value;
        if let Some(x) = self.data.get_mut(r) {
            *x = *x - value;
        }
    }

    pub fn build(mut self) -> Vec<T> {
        for i in 1..self.data.len() {
            self.data[i] = self.data[i] + self.data[i - 1];
        }

        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let n = 100;
        let t = 1000;

        let mut rng = rand::thread_rng();

        let mut a = Imos1D::<i32>::new(n);
        let mut ans = vec![0; n];

        for _ in 0..t {
            let lr = rand_range(&mut rng, 0..n + 1);
            let x = rng.gen_range(-100..=100);

            a.update(lr.clone(), x);

            for i in lr {
                ans[i] += x;
            }
        }

        assert_eq!(a.build(), ans);
    }
}
