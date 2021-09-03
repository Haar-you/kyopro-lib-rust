use crate::math::{factorial::FactorialTable, ff_traits::FF};
use std::cmp::min;

impl<T: FF + From<usize>> FactorialTable<T> {
    pub fn bell_number(&self, n: usize, k: usize) -> T {
        match n {
            0 => T::from(1),
            _ => {
                let k = min(n, k);
                let mut t = vec![T::from(1); k];

                for i in 1..k {
                    t[i] = match i % 2 {
                        0 => t[i - 1] + self.inv_facto(i),
                        _ => t[i - 1] - self.inv_facto(i),
                    }
                }

                (1..=k)
                    .map(|i| t[k - i] * T::from(i).pow(n as u64) * self.inv_facto(i))
                    .sum()
            }
        }
    }
}
