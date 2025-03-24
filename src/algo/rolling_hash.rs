//! Rolling Hash
use std::ops::Range;

/// 文字列のハッシュ値を計算する構造体。
pub struct RollingHash {
    m: u64,
    b: u64,
    pow: Vec<u64>,
}

impl RollingHash {
    /// 最大長`size`、基数`b`と剰余の除数`m`を設定した[`RollingHash`]を用意する。
    pub fn new(size: usize, m: u64, b: u64) -> Self {
        let mut pow = vec![1; size + 1];

        for i in 1..=size {
            pow[i] = pow[i - 1] * b % m;
        }

        Self { m, b, pow }
    }

    /// 文字列`s`のハッシュを計算する。
    pub fn hash(&self, s: &str) -> u64 {
        s.as_bytes()
            .iter()
            .fold(0, |acc, c| (acc * self.b + *c as u64) % self.m)
    }

    /// 文字列`s`からハッシュ計算テーブルを作る。
    pub fn hash_table(&self, s: &str) -> Table {
        let mut ret = vec![1; s.len() + 1];

        for (i, c) in s.as_bytes().iter().enumerate() {
            ret[i + 1] = (ret[i] * self.b + *c as u64) % self.m;
        }

        Table {
            table: ret,
            rh: self,
        }
    }
}

/// [`RollingHash::hash_table`]で返される、部分列のハッシュ値計算用の構造体。
pub struct Table<'a> {
    table: Vec<u64>,
    rh: &'a RollingHash,
}

impl<'a> Table<'a> {
    /// 範囲`l..r`でのハッシュを計算する。
    pub fn hash(&self, Range { start: l, end: r }: Range<usize>) -> u64 {
        let m = self.rh.m;
        (self.table[r] - self.table[l] * self.rh.pow[r - l] % m + m * m) % m
    }
}
