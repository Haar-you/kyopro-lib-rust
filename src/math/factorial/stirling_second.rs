//! 第二種スターリング数
use crate::{math::factorial::FactorialTable, num::ff::*};

/// 第二種スターリング数
pub trait StirlingSecond {
    /// 計算結果の型
    type Output;
    /// 第二種スターリング数$S(n,k)$を計算する。
    fn stirling_second(&self, n: usize, k: usize) -> Self::Output;
}

impl<Modulo: FF> StirlingSecond for FactorialTable<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Modulo::Element;

    fn stirling_second(&self, n: usize, k: usize) -> Self::Output {
        match (n, k) {
            (0, 0) => self.modulo.from_u64(1),
            (0, _) => self.modulo.from_u64(0),
            _ => {
                let mut ret = self.modulo.from_u64(0);

                for i in 1..=k {
                    if (k - i) % 2 == 0 {
                        ret += self.comb(k, i) * self.modulo.from_u64(i as u64).pow(n as u64);
                    } else {
                        ret -= self.comb(k, i) * self.modulo.from_u64(i as u64).pow(n as u64);
                    }
                }
                ret * self.inv_facto(k)
            }
        }
    }
}
