//! ローリングハッシュの結合
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc331/tasks/abc331_f>

pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// ローリングハッシュ
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Hash {
    hash: u64,
    power: u64,
}

impl Hash {
    /// ハッシュ値を返す。
    pub fn hash(&self) -> u64 {
        self.hash
    }
}

/// ローリングハッシュ生成器
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Hasher {
    base: u64,
    m: u64,
}

impl Hasher {
    /// 基数`base`と除数`m`のハッシュ生成器を返す。
    pub fn new(base: u64, m: u64) -> Self {
        Self { base, m }
    }

    /// 値`value`からハッシュ値を作る。
    pub fn make(&self, value: u64) -> Hash {
        Hash {
            hash: value % self.m,
            power: self.base,
        }
    }

    /// ハッシュを結合する。
    pub fn concat(&self, left: Hash, right: Hash) -> Hash {
        let hash = (left.hash + right.hash * left.power % self.m) % self.m;
        let power = left.power * right.power % self.m;
        Hash { hash, power }
    }
}

impl_algebra!(
    Hasher;
    set: Hash;
    op: |s: &Self, a: Hash, b: Hash| s.concat(a, b);
    id: |_| Hash { hash: 0, power: 1};
    assoc;
);
