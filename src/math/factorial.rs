use std::ops::{ Mul, Div };

#[derive(Clone, Debug)]
pub struct FactorialTable<T> {
    factorial: Vec<T>,
    invs: Vec<T>
}

impl<T> FactorialTable<T>
where
    T: Copy + Mul<Output = T> + Div<Output = T> + From<usize>
{
    /// Time complexity O(n)
    ///
    /// Space complexity O(n)
    pub fn new(n: usize) -> Self {
        let mut factorial = vec![T::from(1); n + 1];
        let mut invs = vec![T::from(1); n + 1];

        for i in 1 ..= n {
            factorial[i] = factorial[i - 1] * T::from(i);
        }

        invs[n] = T::from(1) / factorial[n];

        for i in (0 .. n).rev() {
            invs[i] = invs[i + 1] * T::from(i + 1);
        }

        Self { factorial, invs }
    }

    /// nの階乗
    ///
    /// Time complexity O(1)
    pub fn facto(&self, n: usize) -> T {
        self.factorial[n]
    }

    /// nの階乗の逆元
    ///
    /// Time complexity O(1)
    pub fn inv_facto(&self, n: usize) -> T {
        self.invs[n]
    }

    /// n個からk個とりだす順列の個数 (nPk)
    ///
    /// Time complexity O(1)
    pub fn perm(&self, n: usize, k: usize) -> T {
        if n < k {
            T::from(0)
        }
        else {
            self.factorial[n] * self.invs[n - k]
        }
    }

    /// n個からk個とりだす組み合わせの個数 (nCk)
    ///
    /// Time complexity O(1)
    pub fn comb(&self, n: usize, k: usize) -> T {
        if n < k {
            T::from(0)
        }
        else {
            self.perm(n, k) * self.invs[k]
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::math::modint::{ ModInt };
    use crate::misc::generics_int::{ GenericsInt };
    use crate::generics_int;

    generics_int!(G1000000007, 1000000007);
    type Mint = ModInt<G1000000007>;

    #[test]
    fn test() {
        let ft = FactorialTable::<Mint>::new(2000000);

        // https://yukicoder.me/problems/no/117
        assert_eq!(ft.comb(1, 1000000), Mint::from(0));
        assert_eq!(ft.comb(0, 0), Mint::from(1));
        assert_eq!(ft.perm(1000000, 1000000), Mint::from(641102369));
        assert_eq!(ft.perm(1, 10), Mint::from(0));
    }
}
