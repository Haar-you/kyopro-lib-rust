use crate::math::{factorial::FactorialTable, ff::traits::*};

impl<Modulo: FF> FactorialTable<Modulo>
where
    Modulo::Output: FFElem,
{
    pub fn bernoulli_number(&self, n: usize) -> Vec<Modulo::Output> {
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
    use crate::math::ff::const_modint::*;

    #[test]
    fn test() {
        let modulo = ConstModIntBuilder::<1000000007>::new();
        let ft = FactorialTable::new(100, modulo.clone());

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
