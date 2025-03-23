//! 前計算による素因数分解

/// 前計算による素因数分解
///
/// **Space complexity** $O(n)$
pub struct FactorizeSieve {
    p: Vec<usize>,
}

impl FactorizeSieve {
    /// `n`以下の非負整数を素因数分解できる[`FactorizeSieve`]を構築する。
    pub fn new(n: usize) -> Self {
        let mut p = vec![0; n + 1];

        for i in 2..=n {
            if p[i] == 0 {
                for j in (i..=n).step_by(i) {
                    if p[j] == 0 {
                        p[j] = i;
                    }
                }
            }
        }

        Self { p }
    }

    /// `n`の素因数を列挙する。
    pub fn factorize(&self, mut n: usize) -> Vec<usize> {
        let mut ret = vec![];

        while n > 1 {
            ret.push(self.p[n]);
            n /= self.p[n];
        }

        ret
    }
}
