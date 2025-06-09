//! 階乗
pub mod bell;
pub mod bernoulli;
pub mod catalan;
pub mod stirling_second;

use crate::num::ff::*;

/// 有限体上での階乗の計算を行う構造体。
#[derive(Clone, Debug)]
pub struct FactorialTable<Modulo: FF> {
    factorial: Vec<Modulo::Element>,
    invs: Vec<Modulo::Element>,
    modulo: Modulo,
}

impl<Modulo: FF> FactorialTable<Modulo>
where
    Modulo::Element: FFElem + Copy,
{
    /// **Time complexity** $O(n)$
    ///
    /// **Space complexity** $O(n)$
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
    /// **Time complexity** $O(1)$
    pub fn facto(&self, n: usize) -> Modulo::Element {
        self.factorial[n]
    }

    /// nの階乗の逆元
    ///
    /// **Time complexity** $O(1)$
    pub fn inv_facto(&self, n: usize) -> Modulo::Element {
        self.invs[n]
    }

    /// n個からk個とりだす順列の個数 (${}_n \mathrm{ P }_k$)
    ///
    /// **Time complexity** $O(1)$
    pub fn perm(&self, n: usize, k: usize) -> Modulo::Element {
        if n < k {
            self.modulo.from_u64(0)
        } else {
            self.factorial[n] * self.invs[n - k]
        }
    }

    /// n個からk個とりだす組み合わせの個数 (${}_n \mathrm{ C }_k$)
    ///
    /// **Time complexity** $O(1)$
    pub fn comb(&self, n: usize, k: usize) -> Modulo::Element {
        if n < k {
            self.modulo.from_u64(0)
        } else {
            self.perm(n, k) * self.invs[k]
        }
    }

    /// n個から重複を許してk個選ぶ場合の数 (${}_n \mathrm{ H }_k$)
    ///
    /// **Time complexity** $O(1)$
    pub fn h(&self, n: usize, k: usize) -> Modulo::Element {
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
    use crate::{
        math::stirling_second_table::stirling_second_table, num::const_modint::*, num::modint::*,
    };

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

    #[test]
    fn test_stirling_second() {
        use super::stirling_second::StirlingSecond;

        let modulo = ConstModIntBuilder::<1000000007>;
        let ft = FactorialTable::new(2000000, modulo);
        let n = 100;

        let ans = stirling_second_table(n, modulo);

        for i in 0..=n {
            for j in 0..=n {
                assert_eq!(ans[i][j], ft.stirling_second(i, j));
            }
        }
    }
}
