//! Mexを求められるデータ構造
//!
//! # Problems
//! - <https://atcoder.jp/contests/hhkb2020/tasks/hhkb2020_c>
//!
//! # References
//! - <https://rsk0315.hatenablog.com/entry/2020/10/11/125049>

use std::collections::BTreeMap;

/// 整数の追加・削除とMexを求められるデータ構造
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IntegerSet {
    data: BTreeMap<i64, i64>,
}

impl IntegerSet {
    /// 空の`IntegerSet`を生成
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// `x`を含む半開区間を返す
    ///
    /// **Time complexity** $O(\log n)$
    pub fn interval(&self, x: i64) -> Option<(i64, i64)> {
        if let Some((&k, &v)) = self.data.range(..=x).next_back() {
            if k <= x && x < v {
                return Some((k, v));
            }
        }
        None
    }

    /// `x`を含むかを判定
    ///
    /// **Time complexity** $O(\log n)$
    pub fn contains(&self, x: i64) -> bool {
        self.interval(x).is_some()
    }

    /// `x`を追加する
    ///
    /// **Time complexity** $O(\log n)$
    pub fn insert(&mut self, x: i64) {
        if let Some((&k, &v)) = self.data.range(..=x).next_back() {
            if k <= x && x < v {
                return;
            } else if x == v {
                if self.data.contains_key(&(x + 1)) {
                    let v = self.data.remove(&(x + 1)).unwrap();
                    self.data.insert(k, v);
                } else {
                    self.data.insert(k, x + 1);
                }
                return;
            }
        }

        if self.data.contains_key(&(x + 1)) {
            let v = self.data.remove(&(x + 1)).unwrap();
            self.data.insert(x, v);
        } else {
            self.data.insert(x, x + 1);
        }
    }

    /// `x`を削除する
    ///
    /// **Time complexity** $O(\log n)$
    pub fn remove(&mut self, x: i64) {
        if let Some((k, v)) = self.interval(x) {
            self.data.remove(&k);

            if k != x {
                self.data.insert(k, x);
            }
            if x + 1 != v {
                self.data.insert(x + 1, v);
            }
        }
    }

    /// `self`に含まれない数のうち`x`以上で最小のもの
    ///
    /// **Time complexity** $O(\log n)$
    pub fn mex(&self, x: i64) -> i64 {
        self.interval(x).map_or(x, |(_, v)| v)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 100;
        let t = 1000;
        let l = 200;

        let mut s = IntegerSet::new();
        let mut a = BTreeSet::new();

        for _ in 0..n {
            let x = rng.gen_range(-l..=l);
            s.insert(x);
            a.insert(x);
        }

        for _ in 0..t {
            let x = rng.gen_range(-l..=l);
            s.insert(x);
            a.insert(x);

            let x = rng.gen_range(-l..=l);
            s.remove(x);
            a.remove(&x);

            let x = rng.gen_range(-l..=l);

            let mut mex = x;
            while a.contains(&mex) {
                mex += 1;
            }

            assert_eq!(s.mex(x), mex);
        }
    }
}
