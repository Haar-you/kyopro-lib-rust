//! 順序付き辞書
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/ordered_set>

use std::cell::Cell;
use std::cmp::Ordering;
use std::ptr;

struct Node<K, V> {
    key: K,
    value: V,
    size: usize,
    lc: *mut Self,
    rc: *mut Self,
    par: *mut Self,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            size: 1,
            lc: ptr::null_mut(),
            rc: ptr::null_mut(),
            par: ptr::null_mut(),
        }
    }

    fn set_value(this: *mut Self, mut value: V) -> V {
        assert!(!this.is_null());
        std::mem::swap(unsafe { &mut (*this).value }, &mut value);
        value
    }

    fn rotate(this: *mut Self) {
        let p = Self::get_par(this).unwrap();
        let pp = Self::get_par(p).unwrap();

        if Self::left_of(p).unwrap() == this {
            let c = Self::right_of(this).unwrap();
            Self::set_left(p, c);
            Self::set_right(this, p);
        } else {
            let c = Self::left_of(this).unwrap();
            Self::set_right(p, c);
            Self::set_left(this, p);
        }

        unsafe {
            if !pp.is_null() {
                if (*pp).lc == p {
                    (*pp).lc = this;
                }
                if (*pp).rc == p {
                    (*pp).rc = this;
                }
            }

            assert!(!this.is_null());
            (*this).par = pp;
        }

        Self::update(p);
        Self::update(this);
    }

    fn status(this: *mut Self) -> i32 {
        let par = Self::get_par(this).unwrap();

        if par.is_null() {
            return 0;
        }
        if unsafe { (*par).lc } == this {
            return 1;
        }
        if unsafe { (*par).rc } == this {
            return -1;
        }

        unreachable!()
    }

    fn pushdown(this: *mut Self) {
        if !this.is_null() {
            Self::update(this);
        }
    }

    fn update(this: *mut Self) {
        assert!(!this.is_null());
        unsafe {
            (*this).size = 1 + Self::size_of((*this).lc) + Self::size_of((*this).rc);
        }
    }

    fn splay(this: *mut Self) {
        while Self::status(this) != 0 {
            let par = Self::get_par(this).unwrap();

            if Self::status(par) == 0 {
                Self::rotate(this);
            } else if Self::status(this) == Self::status(par) {
                Self::rotate(par);
                Self::rotate(this);
            } else {
                Self::rotate(this);
                Self::rotate(this);
            }
        }
    }

    fn get(root: *mut Self, mut index: usize) -> *mut Self {
        if root.is_null() {
            return root;
        }

        let mut cur = root;

        loop {
            Self::pushdown(cur);

            let left = Self::left_of(cur).unwrap();
            let lsize = Self::size_of(left);

            match index.cmp(&lsize) {
                Ordering::Less => {
                    cur = left;
                }
                Ordering::Equal => {
                    Self::splay(cur);
                    return cur;
                }
                Ordering::Greater => {
                    cur = Self::right_of(cur).unwrap();
                    index -= lsize + 1;
                }
            }
        }
    }

    fn merge(left: *mut Self, right: *mut Self) -> *mut Self {
        if left.is_null() {
            return right;
        }
        if right.is_null() {
            return left;
        }

        let cur = Self::get(left, Self::size_of(left) - 1);

        Self::set_right(cur, right);
        Self::update(right);
        Self::update(cur);

        cur
    }

    fn split(root: *mut Self, index: usize) -> (*mut Self, *mut Self) {
        if root.is_null() {
            return (ptr::null_mut(), ptr::null_mut());
        }
        if index >= Self::size_of(root) {
            return (root, ptr::null_mut());
        }

        let cur = Self::get(root, index);
        let left = Self::left_of(cur).unwrap();

        if !left.is_null() {
            unsafe {
                (*left).par = ptr::null_mut();
            }
            Self::update(left);
        }
        assert!(!cur.is_null());
        unsafe {
            (*cur).lc = ptr::null_mut();
        }
        Self::update(cur);

        (left, cur)
    }

    fn traverse(cur: *mut Self, f: &mut impl FnMut(&K, &mut V)) {
        if !cur.is_null() {
            Self::pushdown(cur);
            Self::traverse(Self::left_of(cur).unwrap(), f);
            f(unsafe { &(*cur).key }, unsafe { &mut (*cur).value });
            Self::traverse(Self::right_of(cur).unwrap(), f);
        }
    }

    fn set_left(this: *mut Self, left: *mut Self) {
        assert!(!this.is_null());
        unsafe {
            (*this).lc = left;
            if !left.is_null() {
                (*left).par = this;
            }
        }
    }

    fn set_right(this: *mut Self, right: *mut Self) {
        assert!(!this.is_null());
        unsafe {
            (*this).rc = right;
            if !right.is_null() {
                (*right).par = this;
            }
        }
    }

    fn size_of(this: *mut Self) -> usize {
        if this.is_null() {
            0
        } else {
            unsafe { (*this).size }
        }
    }

    fn left_of(this: *mut Self) -> Option<*mut Self> {
        (!this.is_null()).then_some(unsafe { (*this).lc })
    }

    fn right_of(this: *mut Self) -> Option<*mut Self> {
        (!this.is_null()).then_some(unsafe { (*this).rc })
    }

    fn get_par(this: *mut Self) -> Option<*mut Self> {
        (!this.is_null()).then_some(unsafe { (*this).par })
    }

    fn clear(this: *mut Self) {
        if !this.is_null() {
            let lc = Self::left_of(this).unwrap();
            let rc = Self::right_of(this).unwrap();

            let _ = unsafe { Box::from_raw(this) };

            Self::clear(lc);
            Self::clear(rc);
        }
    }

    fn key_of<'a>(this: *mut Self) -> Option<&'a K> {
        (!this.is_null()).then(|| unsafe { &(*this).key })
    }
}

impl<K: Ord, V> Node<K, V> {
    fn binary_search(this: *mut Self, key: &K) -> Result<usize, usize> {
        if this.is_null() {
            Err(0)
        } else {
            let left = Self::left_of(this).unwrap();
            let right = Self::right_of(this).unwrap();
            let c = Self::size_of(left);
            match Self::key_of(this).unwrap().cmp(key) {
                Ordering::Equal => Ok(c),
                Ordering::Greater => Self::binary_search(left, key),
                Ordering::Less => Self::binary_search(right, key)
                    .map(|a| a + c + 1)
                    .map_err(|a| a + c + 1),
            }
        }
    }
}

/// 順序付き辞書
pub struct OrderedMap<K, V> {
    root: Cell<*mut Node<K, V>>,
}

impl<K: Ord, V> OrderedMap<K, V> {
    /// 空の`OrderedMap`を返す。
    pub fn new() -> Self {
        Self {
            root: Cell::new(ptr::null_mut()),
        }
    }

    /// 要素数を返す。
    pub fn len(&self) -> usize {
        Node::size_of(self.root.get())
    }

    /// 要素数が`0`ならば`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.root.get().is_null()
    }

    /// `key`が存在するとき、それが何番目のキーであるかを`Ok`で返す。
    /// そうでないとき、仮に`key`があったとき何番目のキーであったか、を`Err`で返す。
    pub fn binary_search(&self, key: &K) -> Result<usize, usize> {
        Node::binary_search(self.root.get(), key)
    }

    /// `key`以下の最大のキーをもつキーと値のペアを返す。
    pub fn max_le(&self, key: &K) -> Option<(&K, &V)> {
        match self.binary_search(key) {
            Ok(i) => self.get_by_index(i),
            Err(i) => {
                if i > 0 {
                    self.get_by_index(i - 1)
                } else {
                    None
                }
            }
        }
    }

    /// `key`以上の最小のキーをもつキーと値のペアを返す。
    pub fn min_ge(&self, key: &K) -> Option<(&K, &V)> {
        match self.binary_search(key) {
            Ok(i) | Err(i) => self.get_by_index(i),
        }
    }

    /// `key`をキーとして持つならば`true`を返す。
    pub fn contains(&self, key: &K) -> bool {
        Node::binary_search(self.root.get(), key).is_ok()
    }

    /// `key`がすでに存在している場合、値を`value`で更新して、古い値を`Some`で返す。
    /// そうでないとき、`key`に`value`を紐付けて、`None`を返す。
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match Node::binary_search(self.root.get(), &key) {
            Ok(i) => {
                let (l, r) = Node::split(self.root.get(), i);
                let (m, r) = Node::split(r, 1);
                let old = Node::set_value(m, value);

                let r = Node::merge(m, r);
                let root = Node::merge(l, r);
                self.root.set(root);
                Some(old)
            }
            Err(i) => {
                let (l, r) = Node::split(self.root.get(), i);
                let node = Box::into_raw(Box::new(Node::new(key, value)));
                let root = Node::merge(l, Node::merge(node, r));
                self.root.set(root);
                None
            }
        }
    }

    /// キー`key`に対応する値の参照を返す。
    pub fn get(&self, key: &K) -> Option<&V> {
        let k = Node::binary_search(self.root.get(), key).ok()?;
        self.get_by_index(k).map(|(_, v)| v)
    }

    /// キー`key`に対応する値の可変参照を返す
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let k = Node::binary_search(self.root.get(), key).ok()?;
        self.get_value_mut_by_index(k)
    }

    /// キー`key`があれば、そのキーと対応する値を削除して、その値を`Some`で返す。
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let i = Node::binary_search(self.root.get(), key).ok()?;
        self.remove_by_index(i).map(|(_, v)| v)
    }

    /// `i`番目のキーとその対応する値のペアへの参照を返す。
    pub fn get_by_index(&self, i: usize) -> Option<(&K, &V)> {
        if i >= self.len() {
            None
        } else {
            let t = Node::get(self.root.get(), i);
            self.root.set(t);
            (!t.is_null()).then(|| unsafe { (&(*t).key, &(*t).value) })
        }
    }

    /// `i`番目のキーへの参照を返す。
    pub fn get_key_by_index(&self, i: usize) -> Option<&K> {
        self.get_by_index(i).map(|(k, _)| k)
    }

    /// `i`番目のキーに対応する値への参照を返す。
    pub fn get_value_by_index(&self, i: usize) -> Option<&V> {
        self.get_by_index(i).map(|(_, v)| v)
    }

    /// `i`番目のキーに対応する値への可変参照を返す。
    pub fn get_value_mut_by_index(&mut self, i: usize) -> Option<&mut V> {
        if i >= self.len() {
            None
        } else {
            let t = Node::get(self.root.get(), i);
            self.root.set(t);
            (!t.is_null()).then(|| unsafe { &mut (*t).value })
        }
    }

    /// `i`番目の要素を削除して、そのキーと値のペアを返す。
    pub fn remove_by_index(&mut self, i: usize) -> Option<(K, V)> {
        let (l, r) = Node::split(self.root.get(), i);
        let (m, r) = Node::split(r, 1);
        self.root.set(Node::merge(l, r));

        (!m.is_null()).then(|| unsafe {
            let m = Box::from_raw(m);
            let node = *m;
            (node.key, node.value)
        })
    }

    /// 順序付き辞書のすべての要素を順番に`f`に渡す。
    pub fn for_each(&self, mut f: impl FnMut(&K, &mut V)) {
        Node::traverse(self.root.get(), &mut f);
    }

    // pub fn pop_first(&mut self) -> Option<V>
    // pub fn pop_last(&mut self) -> Option<V>
    // pub fn first(&self) -> Option<&V>
    // pub fn last(&self) -> Option<&V>
    // pub fn first_mut(&mut self) -> Option<&mut V>
    // pub fn last_mut(&mut self) -> Option<&mut V>
}

impl<K, V> std::ops::Drop for OrderedMap<K, V> {
    fn drop(&mut self) {
        Node::clear(self.root.get());
    }
}

impl<K: Ord, V> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let mut map = OrderedMap::<u32, u32>::new();
        let mut ans = BTreeMap::<u32, u32>::new();

        let q = 10000;

        for _ in 0..q {
            let x: u32 = rng.gen_range(0..1000);
            let y: u32 = rng.gen();

            assert_eq!(map.insert(x, y), ans.insert(x, y));

            let x = rng.gen_range(0..1000);

            assert_eq!(map.remove(&x), ans.remove(&x));

            let x = rng.gen_range(0..1000);

            assert_eq!(map.get(&x), ans.get(&x));

            assert_eq!(map.len(), ans.len());
        }
    }
}
