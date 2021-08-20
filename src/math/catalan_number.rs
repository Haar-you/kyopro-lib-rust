use crate::math::{factorial::FactorialTable, modint::FF};

impl<T: FF + From<usize>> FactorialTable<T> {
    pub fn catalan_number(&self, n: usize) -> T {
        if n == 0 {
            T::from(1)
        } else {
            self.comb(2 * n, n) - self.comb(2 * n, n - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generics_int;
    use crate::math::modint::ModInt;
    use crate::misc::generics_int::GenericsInt;

    generics_int!(G1000000007, 1000000007);
    type Mint = ModInt<G1000000007>;

    #[test]
    fn test() {
        let ft = FactorialTable::<Mint>::new(100);

        let catalans = (0..=30).map(|i| ft.catalan_number(i)).collect::<Vec<_>>();

        // https://oeis.org/A000108/list
        let ans: Vec<u64> = vec![
            1,
            1,
            2,
            5,
            14,
            42,
            132,
            429,
            1430,
            4862,
            16796,
            58786,
            208012,
            742900,
            2674440,
            9694845,
            35357670,
            129644790,
            477638700,
            1767263190,
            6564120420,
            24466267020,
            91482563640,
            343059613650,
            1289904147324,
            4861946401452,
            18367353072152,
            69533550916004,
            263747951750360,
            1002242216651368,
            3814986502092304,
        ];
        let ans = ans.into_iter().map(|x| Mint::from(x)).collect::<Vec<_>>();

        assert_eq!(catalans, ans);
    }
}
