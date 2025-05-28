//! Splay Tree
//!
//! # Problems
//! - <https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1508>
//! - <https://judge.yosupo.jp/problem/range_reverse_range_sum>
//!
//! # References
//! - <https://en.wikipedia.org/wiki/Splay_tree>

use std::cell::Cell;
use std::cmp::Ordering;
use std::ops::Range;
use std::ptr;

use crate::algebra::traits::Monoid;

struct Node<M: Monoid> {
    value: M,
    sum: M,
    size: usize,
    rev: bool,
    lc: *mut Node<M>,
    rc: *mut Node<M>,
    par: *mut Node<M>,
}

impl<M: Monoid + Clone> Node<M> {
    fn new(value: M) -> Self {
        Self {
            value,
            sum: M::id(),
            size: 1,
            rev: false,
            lc: ptr::null_mut(),
            rc: ptr::null_mut(),
            par: ptr::null_mut(),
        }
    }

    fn get_sum(this: *mut Self) -> M {
        assert!(!this.is_null());
        unsafe { (*this).sum.clone() }
    }

    fn set_value(this: *mut Self, value: M) {
        assert!(!this.is_null());
        unsafe {
            (*this).value = value;
        }
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

    fn reverse(this: *mut Self) {
        if !this.is_null() {
            unsafe {
                (*this).rev ^= true;
            }
        }
    }

    fn pushdown(this: *mut Self) {
        if !this.is_null() {
            unsafe {
                if (*this).rev {
                    std::mem::swap(&mut (*this).lc, &mut (*this).rc);
                    Self::reverse((*this).lc);
                    Self::reverse((*this).rc);
                    (*this).rev = false;
                }
            }
            Self::update(this);
        }
    }

    fn update(this: *mut Self) {
        assert!(!this.is_null());
        unsafe {
            (*this).size = 1 + Self::size_of((*this).lc) + Self::size_of((*this).rc);

            (*this).sum = (*this).value.clone();
            if !(*this).lc.is_null() {
                (*this).sum = M::op(Self::get_sum(this), Self::get_sum((*this).lc));
            }
            if !(*this).rc.is_null() {
                (*this).sum = M::op(Self::get_sum(this), Self::get_sum((*this).rc));
            }
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

    fn traverse(cur: *mut Self, f: &mut impl FnMut(&M)) {
        if !cur.is_null() {
            Self::pushdown(cur);
            Self::traverse(Self::left_of(cur).unwrap(), f);
            f(unsafe { &(*cur).value });
            Self::traverse(Self::right_of(cur).unwrap(), f);
        }
    }
}

impl<M: Monoid> Node<M> {
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
}

/// スプレーツリー
pub struct SplayTree<M: Monoid> {
    root: Cell<*mut Node<M>>,
}

impl<M: Monoid + Clone> Default for SplayTree<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: Monoid + Clone> SplayTree<M> {
    /// モノイド`m`をもつ`SplayTree<M>`を生成
    pub fn new() -> Self {
        Self {
            root: Cell::new(ptr::null_mut()),
        }
    }

    /// 値`value`をもつノード一つのみからなる`SplayTree<M>`を生成
    pub fn singleton(value: M) -> Self {
        let root = Box::new(Node::new(value));

        Self {
            root: Cell::new(Box::into_raw(root)),
        }
    }

    /// スプレーツリーの要素数を返す
    pub fn len(&self) -> usize {
        Node::size_of(self.root.get())
    }

    /// スプレーツリーが要素を持たなければ`true`を返す
    pub fn is_empty(&self) -> bool {
        self.root.get().is_null()
    }

    /// `index`番目の要素の参照を返す
    pub fn get(&self, index: usize) -> Option<&M> {
        self.root.set(Node::get(self.root.get(), index));
        let node = self.root.get();

        if node.is_null() {
            None
        } else {
            unsafe { Some(&(*node).value) }
        }
    }

    /// `index`番目の要素を`value`に変更する
    pub fn set(&mut self, index: usize, value: M) {
        let root = Node::get(self.root.get(), index);
        Node::set_value(root, value);
        Node::update(root);
        self.root.set(root);
    }

    /// 右側にスプレーツリーを結合する
    pub fn merge_right(&mut self, right: Self) {
        let root = Node::merge(self.root.get(), right.root.get());
        right.root.set(ptr::null_mut());
        self.root.set(root);
    }

    /// 左側にスプレーツリーを結合する
    pub fn merge_left(&mut self, left: Self) {
        let root = Node::merge(left.root.get(), self.root.get());
        left.root.set(ptr::null_mut());
        self.root.set(root);
    }

    /// 左側に`index`個の要素があるように、左右で分割する
    pub fn split(self, index: usize) -> (Self, Self) {
        let (l, r) = Node::split(self.root.get(), index);
        self.root.set(ptr::null_mut());
        (Self { root: Cell::new(l) }, Self { root: Cell::new(r) })
    }

    /// 要素を`index`番目になるように挿入する
    pub fn insert(&mut self, index: usize, value: M) {
        let (l, r) = Node::split(self.root.get(), index);
        let node = Box::into_raw(Box::new(Node::new(value)));
        let root = Node::merge(l, Node::merge(node, r));
        self.root.set(root);
    }

    /// `index`番目の要素を削除して、値を返す
    pub fn remove(&mut self, index: usize) -> Option<M> {
        let (l, r) = Node::split(self.root.get(), index);
        let (m, r) = Node::split(r, 1);

        if m.is_null() {
            return None;
        }

        let value = unsafe {
            let m = Box::from_raw(m);
            ptr::read(&m.value)
        };

        self.root.set(Node::merge(l, r));

        Some(value)
    }

    /// `start..end`の範囲を反転させる
    pub fn reverse(&mut self, Range { start, end }: Range<usize>) {
        let (m, r) = Node::split(self.root.get(), end);
        let (l, m) = Node::split(m, start);

        Node::reverse(m);

        let m = Node::merge(l, m);
        let root = Node::merge(m, r);
        self.root.set(root);
    }

    /// `start..end`の範囲でのモノイドの演算の結果を返す
    pub fn fold(&self, Range { start, end }: Range<usize>) -> M {
        let (m, r) = Node::split(self.root.get(), end);
        let (l, m) = Node::split(m, start);

        let ret = if m.is_null() {
            M::id()
        } else {
            Node::get_sum(m)
        };

        let m = Node::merge(l, m);
        let root = Node::merge(m, r);
        self.root.set(root);

        ret
    }

    /// 先頭に値を追加する
    pub fn push_first(&mut self, value: M) {
        let left = Self::singleton(value);
        self.merge_left(left);
    }
    /// 末尾に値を追加する
    pub fn push_last(&mut self, value: M) {
        let right = Self::singleton(value);
        self.merge_right(right);
    }
    /// 先頭の値を削除する
    pub fn pop_first(&mut self) -> Option<M> {
        self.remove(0)
    }
    /// 末尾の値を削除する
    pub fn pop_last(&mut self) -> Option<M> {
        if self.is_empty() {
            None
        } else {
            self.remove(self.len() - 1)
        }
    }

    /// 列の要素を始めから辿り、その参照を`f`に渡す。
    pub fn for_each(&self, mut f: impl FnMut(&M)) {
        Node::traverse(self.root.get(), &mut f);
    }
}

impl<M: Monoid> std::ops::Drop for SplayTree<M> {
    fn drop(&mut self) {
        Node::clear(self.root.get());
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::sum::*;
    use my_testtools::rand_range;

    use rand::Rng;

    use super::*;

    #[test]
    fn test() {
        let t = 100;

        let mut rng = rand::thread_rng();

        let mut a = vec![];
        let mut st = SplayTree::<Sum<u64>>::new();

        for _ in 0..t {
            assert_eq!(a.len(), st.len());
            let n = a.len();

            let i = rng.gen_range(0..=n);
            let x = Sum(rng.gen::<u32>() as u64);

            a.insert(i, x);
            st.insert(i, x);

            assert_eq!(a.len(), st.len());
            let n = a.len();

            let Range { start: l, end: r } = rand_range(&mut rng, 0..n);
            assert_eq!(a[l..r].iter().cloned().fold_m(), st.fold(l..r));
        }
    }
}
