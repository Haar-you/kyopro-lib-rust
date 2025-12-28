//! 符号付き第一種スターリング数$s(n, k)$を計算する。
//!
//! # References
//! - <https://maspypy.com/stirling-%e6%95%b0%e3%82%92-p-%e3%81%a7%e5%89%b2%e3%81%a3%e3%81%9f%e4%bd%99%e3%82%8a%e3%81%ae%e8%a8%88%e7%ae%97>
//! # Problems
//! - <https://judge.yosupo.jp/problem/stirling_number_of_the_first_kind_small_p_large_n>
use crate::math::combinatorics::binomial_coefficient::ExtLucas;

/// 符号付き第一種スターリング数$s(n, k)$を計算する。
pub struct StirlingFirstSmallP {
    p: u64,
    bc: ExtLucas,
    s: Vec<Vec<u64>>,
}

impl StirlingFirstSmallP {
    /// $\mod p$ ($p$は素数)で前計算をする。
    ///
    /// **Time complexity** $O(p^2)$
    ///
    /// **Space complexity** $O(p^2)$
    pub fn new(p: u64) -> Self {
        let bc = ExtLucas::new(p, 1);
        let mut s = vec![vec![0; p as usize + 1]; p as usize + 1];
        s[0][0] = 1;

        for i in 1..=p as usize {
            for j in 1..=i {
                let mut t = (i as u64 - 1) * s[i - 1][j] % p;
                if t != 0 {
                    t = p - t;
                }

                s[i][j] = t + s[i - 1][j - 1];
                if s[i][j] >= p {
                    s[i][j] %= p;
                }
            }
        }

        Self { p, bc, s }
    }

    /// $s(n, k)$を計算する。
    ///
    /// **Time complexity** $O(1)$
    pub fn calc(&self, n: u64, k: u64) -> u64 {
        let i = n / self.p;
        let j = n % self.p;

        let mut b = (k - i) % (self.p - 1);
        if b == 0 && j > 0 {
            b = self.p - 1;
        }
        let a = ((k - i) - b) / (self.p - 1);

        let mut ret = self.bc.calc(i, a) * self.s[j as usize][b as usize] % self.p;
        if (i - a) % 2 == 1 && ret != 0 {
            ret = self.p - ret;
        }

        ret
    }
}
