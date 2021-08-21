use crate::math::{factorial::FactorialTable, modint::FF};
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

pub fn bell_number_table<T: FF + From<usize>>(n: usize) -> Vec<Vec<T>> {
    let mut ret = vec![vec![T::from(0); n + 1]; n + 1];
    ret[0][0] = T::from(1);

    for i in 1..=n {
        ret[i][1] = T::from(1);
        ret[i][i] = T::from(1);
    }

    for i in 3..=n {
        for j in 2..i {
            ret[i][j] = ret[i - 1][j - 1] + T::from(j) * ret[i - 1][j];
        }
    }

    for i in 0..=n {
        for j in 1..=n {
            ret[i][j] = ret[i][j] + ret[i][j - 1];
        }
    }

    ret
}
