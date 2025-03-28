//! ベルヌーイ数
use crate::math::factorial::FactorialTable;
use crate::num::ff::*;

/// ベルヌーイ数
pub trait BernoulliNumber {
    /// 計算結果の型
    type Output;
    /// ベルヌーイ数$B_0 \ldots B_n$を計算する。
    fn bernoulli_number(&self, n: usize) -> Vec<Self::Output>;
}

impl<Modulo: FF> BernoulliNumber for FactorialTable<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    type Output = Modulo::Element;

    fn bernoulli_number(&self, n: usize) -> Vec<Self::Output> {
        let mut ret = vec![self.modulo.from_u64(0); n + 1];

        ret[0] = self.modulo.from_u64(1);

        for i in 1..=n {
            for k in 0..i {
                let t = ret[k];
                ret[i] += self.comb(i + 1, k) * t;
            }

            ret[i] /= self.modulo.from_u64(i as u64 + 1);
            ret[i] = -ret[i];
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::const_modint::*;

    #[test]
    fn test() {
        let modulo = ConstModIntBuilder::<1000000007>;
        let ft = FactorialTable::new(100, modulo);

        assert_eq!(
            ft.bernoulli_number(5),
            [
                modulo.from_u64(1),
                modulo.frac(-1, 2),
                modulo.frac(1, 6),
                modulo.from_u64(0),
                modulo.frac(-1, 30),
                modulo.from_u64(0)
            ]
        );
    }
}
