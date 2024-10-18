use crate::math::factorial::FactorialTable;
use crate::num::ff::*;
use std::cmp::min;

impl<Modulo: FF> FactorialTable<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    pub fn bell_number(&self, n: usize, k: usize) -> Modulo::Element {
        match n {
            0 => self.modulo.from_u64(1),
            _ => {
                let k = min(n, k);
                let mut t = vec![self.modulo.from_u64(1); k];

                for i in 1..k {
                    t[i] = match i % 2 {
                        0 => t[i - 1] + self.inv_facto(i),
                        _ => t[i - 1] - self.inv_facto(i),
                    }
                }

                (1..=k)
                    .map(|i| {
                        t[k - i] * self.modulo.from_u64(i as u64).pow(n as u64) * self.inv_facto(i)
                    })
                    .fold(self.modulo.from_u64(0), std::ops::Add::add)
            }
        }
    }
}
