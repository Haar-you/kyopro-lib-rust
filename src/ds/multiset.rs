//! 同一要素を複数個挿入可能な`Set`
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc308/tasks/abc308_f>
use std::{collections::BTreeMap, ops::Bound};

/// 同一要素を複数個挿入可能な`Set`
#[derive(Debug, Clone, Default)]
pub struct MultiSet<T> {
    map: BTreeMap<T, usize>,
    size: usize,
}

impl<T: Ord + Eq + Clone> MultiSet<T> {
    /// [`MultiSet<T>`]を生成する。
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            size: 0,
        }
    }

    /// 値`value`を挿入する。
    pub fn insert(&mut self, value: T) {
        *self.map.entry(value).or_default() += 1;
        self.size += 1;
    }

    /// 値`value`を*一つだけ*削除する。
    pub fn remove(&mut self, value: T) -> bool {
        if let Some(count) = self.map.get_mut(&value) {
            *count -= 1;
            self.size -= 1;

            if *count == 0 {
                self.map.remove(&value);
            }

            true
        } else {
            false
        }
    }

    /// 先頭の要素を返す。
    pub fn first(&self) -> Option<T> {
        self.map.iter().next().map(|(k, _)| k.clone())
    }

    /// 末尾の要素を返す。
    pub fn last(&self) -> Option<T> {
        self.map.iter().next_back().map(|(k, _)| k.clone())
    }

    /// 末尾の要素を*一つだけ*削除して返す。
    pub fn pop_last(&mut self) -> Option<T> {
        if let Some((k, v)) = self.map.iter_mut().next_back() {
            *v -= 1;
            self.size -= 1;

            let k = k.clone();

            if *v == 0 {
                self.map.remove(&k);
            }

            Some(k)
        } else {
            None
        }
    }

    /// 先頭の要素を*一つだけ*削除して返す。
    pub fn pop_first(&mut self) -> Option<T> {
        if let Some((k, v)) = self.map.iter_mut().next() {
            *v -= 1;
            self.size -= 1;

            let k = k.clone();

            if *v == 0 {
                self.map.remove(&k);
            }

            Some(k)
        } else {
            None
        }
    }

    /// 値`value`が含まれていれば、`true`を返す。
    pub fn contains(&self, value: &T) -> bool {
        self.map.contains_key(value)
    }

    /// 値`value`が含まれている個数を返す。
    pub fn count(&self, value: &T) -> usize {
        self.map.get(value).cloned().unwrap_or(0)
    }

    /// 要素数を返す。
    pub fn len(&self) -> usize {
        self.size
    }

    /// 要素数が0ならば、`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// `value`以上の最小の要素を返す。
    pub fn ge(&self, value: &T) -> Option<&T> {
        self.map.range(value..).next().map(|(k, _)| k)
    }

    /// `value`以下の最大の要素を返す。
    pub fn le(&self, value: &T) -> Option<&T> {
        self.map.range(..=value).next_back().map(|(k, _)| k)
    }

    /// `value`より大きい最小の要素を返す。
    pub fn gt(&self, value: &T) -> Option<&T> {
        self.map
            .range((Bound::Excluded(value), Bound::Unbounded))
            .next()
            .map(|(k, _)| k)
    }

    /// `value`より小さい最大の要素を返す。
    pub fn lt(&self, value: &T) -> Option<&T> {
        self.map.range(..value).next_back().map(|(k, _)| k)
    }
}

impl<T: Ord + Eq + Clone> FromIterator<T> for MultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret = Self::new();
        for x in iter {
            ret.insert(x);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let set = MultiSet::from_iter([1, 1, 2, 3, 5, 5, 6, 7, 10]);

        assert_eq!(set.ge(&1), Some(&1));
        assert_eq!(set.le(&1), Some(&1));
        assert_eq!(set.gt(&1), Some(&2));
        assert_eq!(set.lt(&1), None);

        assert_eq!(set.ge(&10), Some(&10));
        assert_eq!(set.le(&10), Some(&10));
        assert_eq!(set.gt(&10), None);
        assert_eq!(set.lt(&10), Some(&7));

        assert_eq!(set.ge(&4), Some(&5));
        assert_eq!(set.le(&4), Some(&3));
        assert_eq!(set.gt(&4), Some(&5));
        assert_eq!(set.lt(&4), Some(&3));
    }
}
