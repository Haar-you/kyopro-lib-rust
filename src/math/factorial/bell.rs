//! ベル数
//!
//! # References
//! - <https://ja.wikipedia.org/wiki/%E3%83%99%E3%83%AB%E6%95%B0>
//! - <https://manabitimes.jp/math/892>

use crate::math::factorial::FactorialTable;
use crate::num::ff::*;
use std::cmp::min;

/// ベル数
pub trait BellNumber {
    /// 計算結果の型
    type Output;
    /// ベル数$B(n, k)$を計算する。
    fn bell_number(&self, n: usize, k: usize) -> Self::Output;
}

impl<Modulo: FF> BellNumber for FactorialTable<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Modulo::Element;

    fn bell_number(&self, n: usize, k: usize) -> Self::Output {
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
