//! 順序付き集合
use crate::ds::ordered_map::*;

/// 順序付き集合
#[derive(Default)]
pub struct OrderedSet<K: Ord> {
    map: OrderedMap<K, ()>,
}

impl<K: Ord> OrderedSet<K> {
    /// 空の`OrderedSet`を返す。
    pub fn new() -> Self {
        Self {
            map: OrderedMap::new(),
        }
    }

    /// 要素数を返す。
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// 要素数が`0`ならば`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// `key`が存在するとき、それが何番目のキーであるかを`Ok`で返す。
    /// そうでないとき、仮に`key`があったとき何番目のキーであったか、を`Err`で返す。
    pub fn binary_search(&self, key: &K) -> Result<usize, usize> {
        self.map.binary_search(key)
    }

    /// `l`以上`r`未満の値の個数を返す。
    pub fn count(&self, l: &K, r: &K) -> usize {
        let r = match self.binary_search(r) {
            Ok(i) | Err(i) => i,
        };
        let l = match self.binary_search(l) {
            Ok(i) | Err(i) => i,
        };
        r.saturating_sub(l)
    }

    /// `key`以下の最大のキーをもつキーを返す。
    pub fn max_le(&self, key: &K) -> Option<&K> {
        self.map.max_le(key).map(|(k, _)| k)
    }

    /// `key`以上の最小のキーをもつキーを返す。
    pub fn min_ge(&self, key: &K) -> Option<&K> {
        self.map.min_ge(key).map(|(k, _)| k)
    }

    /// `key`をキーとして持つならば`true`を返す。
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains(key)
    }

    /// `key`が存在するとき、`key`を挿入して、`true`を返す。
    pub fn insert(&mut self, key: K) -> bool {
        self.map.insert(key, ()).is_some()
    }

    /// キー`key`があれば、そのキーを削除して、`true`を返す。
    pub fn remove(&mut self, key: &K) -> bool {
        self.map.remove(key).is_some()
    }

    /// `i`番目のキーへの参照を返す。
    pub fn get_by_index(&self, i: usize) -> Option<&K> {
        self.map.get_by_index(i).map(|(k, _)| k)
    }

    /// `i`番目の要素を削除して、そのキーを返す。
    pub fn remove_by_index(&mut self, i: usize) -> Option<K> {
        self.map.remove_by_index(i).map(|(k, _)| k)
    }

    /// 順序付き辞書のすべての要素を順番に`f`に渡す。
    pub fn for_each(&self, mut f: impl FnMut(&K)) {
        self.map.for_each(|k, _| f(k));
    }

    // pub fn pop_first(&mut self) -> Option<K>
    // pub fn pop_last(&mut self) -> Option<K>
    // pub fn first(&self) -> Option<&K>
    // pub fn last(&self) -> Option<&K>
}
