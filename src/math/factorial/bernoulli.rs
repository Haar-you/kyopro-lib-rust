use crate::math::{factorial::FactorialTable, ff_traits::FF};

impl<T: FF + From<usize>> FactorialTable<T> {
    pub fn bernoulli_number(&self, n: usize) -> Vec<T> {
        let mut ret = vec![T::from(0); n + 1];

        ret[0] = T::from(1);

        for i in 1..=n {
            for k in 0..i {
                let t = ret[k];
                ret[i] += self.comb(i + 1, k) * t;
            }

            ret[i] /= T::from(i + 1);
            ret[i] = -ret[i];
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modulo;
    use crate::math::modint::*;

    modulo!(M, 1000000007);
    type Mint = ModInt<M>;

    #[test]
    fn test() {
        let ft = FactorialTable::<Mint>::new(100);

        assert_eq!(
            ft.bernoulli_number(5),
            [
                Mint::from(1),
                Mint::frac(-1, 2),
                Mint::frac(1, 6),
                Mint::from(0),
                Mint::frac(-1, 30),
                Mint::from(0)
            ]
        );
    }
}
