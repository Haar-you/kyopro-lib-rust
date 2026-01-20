//! 第二種スターリング数$S(n, k)$を計算する。
//!
//! # References
//! - <https://maspypy.com/stirling-%e6%95%b0%e3%82%92-p-%e3%81%a7%e5%89%b2%e3%81%a3%e3%81%9f%e4%bd%99%e3%82%8a%e3%81%ae%e8%a8%88%e7%ae%97>
//! # Problems
//! - <https://judge.yosupo.jp/problem/stirling_number_of_the_second_kind_small_p_large_n>
use crate::math::combinatorics::binomial_coefficient::ExtLucas;

/// 第二種スターリング数$S(n, k)$を計算する。
pub struct StirlingSecondSmallP {
    p: u64,
    bc: ExtLucas,
    s: Vec<Vec<u64>>,
}

impl StirlingSecondSmallP {
    /// $\mod p$ ($p$は素数)で前計算をする。
    ///
    /// **Time complexity** $O(p^2)$
    ///
    /// **Space complexity** $O(p^2)$
    pub fn new(p: u64) -> Self {
        let bc = ExtLucas::new(p, 1);
        let mut s = vec![vec![0; p as usize + 1]; p as usize + 1];

        s[0][0] = 1;

        for (i, si) in s.iter_mut().enumerate().skip(1) {
            si[1] = 1;
            si[i] = 1;
        }

        for i in 3..=p as usize {
            for j in 2..i {
                s[i][j] = (s[i - 1][j - 1] + j as u64 * s[i - 1][j] % p) % p;
            }
        }

        Self { p, bc, s }
    }

    /// $S(n, k)$を計算する。
    ///
    /// **Time complexity** $O(1)$
    pub fn calc(&self, n: u64, k: u64) -> u64 {
        if n <= self.p && k <= self.p {
            return self.s[n as usize][k as usize];
        }

        let i = k / self.p;
        let j = k % self.p;

        let mut b = (n - i) % (self.p - 1);
        if b == 0 {
            b = self.p - 1;
        }
        let a = (n - i - b) / (self.p - 1);

        if b == self.p - 1 {
            self.bc.calc(a, i) * self.s[self.p as usize - 1][j as usize] % self.p
                + self.bc.calc(a, i - 1) * self.s[0][j as usize] % self.p
        } else {
            self.bc.calc(a, i) * self.s[b as usize][j as usize] % self.p
        }
    }
}
