pub mod bell;
pub mod bernoulli;
pub mod catalan;

use crate::num::ff::*;

#[derive(Clone, Debug)]
pub struct FactorialTable<Modulo: FF> {
    factorial: Vec<Modulo::Output>,
    invs: Vec<Modulo::Output>,
    modulo: Modulo,
}

impl<Modulo: FF> FactorialTable<Modulo>
where
    Modulo::Output: FFElem,
{
    /// Time complexity O(n)
    ///
    /// Space complexity O(n)
    pub fn new(n: usize, modulo: Modulo) -> Self {
        let mut factorial = vec![modulo.from_u64(1); n + 1];
        let mut invs = vec![modulo.from_u64(1); n + 1];

        for i in 1..=n {
            factorial[i] = factorial[i - 1] * modulo.from_u64(i as u64);
        }

        invs[n] = modulo.from_u64(1) / factorial[n];

        for i in (0..n).rev() {
            invs[i] = invs[i + 1] * modulo.from_u64(i as u64 + 1);
        }

        Self {
            factorial,
            invs,
            modulo,
        }
    }

    /// nの階乗
    ///
    /// Time complexity O(1)
    pub fn facto(&self, n: usize) -> Modulo::Output {
        self.factorial[n]
    }

    /// nの階乗の逆元
    ///
    /// Time complexity O(1)
    pub fn inv_facto(&self, n: usize) -> Modulo::Output {
        self.invs[n]
    }

    /// n個からk個とりだす順列の個数 (nPk)
    ///
    /// Time complexity O(1)
    pub fn perm(&self, n: usize, k: usize) -> Modulo::Output {
        if n < k {
            self.modulo.from_u64(0)
        } else {
            self.factorial[n] * self.invs[n - k]
        }
    }

    /// n個からk個とりだす組み合わせの個数 (nCk)
    ///
    /// Time complexity O(1)
    pub fn comb(&self, n: usize, k: usize) -> Modulo::Output {
        if n < k {
            self.modulo.from_u64(0)
        } else {
            self.perm(n, k) * self.invs[k]
        }
    }

    pub fn h(&self, n: usize, k: usize) -> Modulo::Output {
        if n == 0 && k == 0 {
            self.modulo.from_u64(1)
        } else {
            self.comb(n + k - 1, k)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::modint::*;

    #[test]
    fn test() {
        let modulo = ModIntBuilder::new(1000000007);
        let ft = FactorialTable::new(2000000, modulo.clone());

        // https://yukicoder.me/problems/no/117
        assert_eq!(ft.comb(1, 1000000), modulo.from_u64(0));
        assert_eq!(ft.comb(0, 0), modulo.from_u64(1));
        assert_eq!(ft.perm(1000000, 1000000), modulo.from_u64(641102369));
        assert_eq!(ft.perm(1, 10), modulo.from_u64(0));
    }
}
