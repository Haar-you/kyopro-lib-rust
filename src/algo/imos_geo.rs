//! 等比級数のimos法
use crate::num::one_zero::{One, Zero};
use std::ops::{Add, Mul, Range, Sub};

/// 等比級数のimos法
pub struct ImosGeo<T> {
    data: Vec<T>,
    r: T,
    pow: Vec<T>,
}

impl<T> ImosGeo<T>
where
    T: Copy + Zero + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// **Time complexity** $O(n)$
    pub fn new(n: usize, r: T) -> Self {
        let mut pow = vec![T::one(); n];
        for i in 1..n {
            pow[i] = pow[i - 1] * r;
        }

        Self {
            data: vec![T::zero(); n],
            r,
            pow,
        }
    }

    /// **Time complexity** $O(1)$
    pub fn update(&mut self, Range { start: l, end: r }: Range<usize>, value: T) {
        self.data[l] = self.data[l] + value;
        if let Some(x) = self.data.get_mut(r) {
            *x = *x - self.pow[r - l] * value;
        }
    }

    /// **Time complexity** $O(n)$
    pub fn build(mut self) -> Vec<T> {
        for i in 1..self.data.len() {
            self.data[i] = self.data[i] + self.data[i - 1] * self.r;
        }

        self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::num::const_modint::*;
    use crate::testtools::rand_range;

    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let ff = ConstModIntBuilder::<1000000007>;

        let n = 100;
        let t = 100;

        for r in 2..=100 {
            let r = ff.from_u64(r);

            let mut a = ImosGeo::new(n, r);
            let mut ans = vec![ff.from_u64(0); n];

            for _ in 0..t {
                let lr = rand_range(&mut rng, 0..n);
                let x = ff.from_i64(rng.gen_range(-100..=100));

                a.update(lr.clone(), x);

                let mut x = x;
                for i in lr {
                    ans[i] += x;
                    x *= r;
                }
            }

            assert_eq!(a.build(), ans);
        }
    }
}
