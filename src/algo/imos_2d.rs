use crate::num::{one_zero::Zero, traits::Signed};
use std::ops::{Add, Range, Sub};

pub struct Imos2D<T> {
    data: Vec<Vec<T>>,
    n: usize,
    m: usize,
}

impl<T: Copy + Signed + Zero + Add<Output = T> + Sub<Output = T>> Imos2D<T> {
    /// **Time complexity O(nm)**
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            data: vec![vec![T::zero(); m]; n],
            n,
            m,
        }
    }

    /// **Time complexity O(1)**
    pub fn update(
        &mut self,
        Range { start: l, end: r }: Range<usize>,
        Range { start: u, end: d }: Range<usize>,
        value: T,
    ) {
        self.data[l][u] = self.data[l][u] + value;
        if let Some(a) = self.data.get_mut(r) {
            if let Some(x) = a.get_mut(d) {
                *x = *x + value;
            }
        }

        if let Some(x) = self.data[l].get_mut(d) {
            *x = *x - value;
        }
        if let Some(a) = self.data.get_mut(r) {
            a[u] = a[u] - value;
        }
    }

    /// **Time complexity O(nm)**
    pub fn build(mut self) -> Vec<Vec<T>> {
        for i in 1..self.n {
            for j in 0..self.m {
                self.data[i][j] = self.data[i][j] + self.data[i - 1][j];
            }
        }

        for i in 0..self.n {
            for j in 1..self.m {
                self.data[i][j] = self.data[i][j] + self.data[i][j - 1];
            }
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
        let m = 200;
        let t = 1000;

        let mut rng = rand::thread_rng();

        let mut a = Imos2D::<i32>::new(n, m);
        let mut ans = vec![vec![0; m]; n];

        for _ in 0..t {
            let lr = rand_range(&mut rng, 0..n);
            let ud = rand_range(&mut rng, 0..m);
            let x = rng.gen_range(-100..=100);

            a.update(lr.clone(), ud.clone(), x);

            for i in lr {
                for j in ud.clone() {
                    ans[i][j] += x;
                }
            }
        }

        assert_eq!(a.build(), ans);
    }
}
