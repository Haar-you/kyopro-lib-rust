//! 遅延スプレー木
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum>

use std::cell::Cell;
use std::cmp::Ordering;
use std::ops::Range;
use std::ptr;

use crate::algebra::action::*;

struct Node<A: Action> {
    value: A::Output,
    sum: A::Output,
    lazy: A::Lazy,
    size: usize,
    rev: bool,
    lc: *mut Node<A>,
    rc: *mut Node<A>,
    par: *mut Node<A>,
}

impl<A: Action> Node<A>
where
    A::Output: Clone,
    A::Lazy: Clone + PartialEq,
{
    fn new(value: A::Output) -> Self {
        Self {
            value,
            sum: A::fold_id(),
            lazy: A::update_id(),
            size: 1,
            rev: false,
            lc: ptr::null_mut(),
            rc: ptr::null_mut(),
            par: ptr::null_mut(),
        }
    }

    fn set_value(this: *mut Self, value: A::Output) {
        if !this.is_null() {
            unsafe {
                (*this).value = value;
            }
        }
    }

    fn rotate(this: *mut Self) {
        let p = Self::par_of(this).unwrap();
        let pp = Self::par_of(p).unwrap();

        Self::pushdown(this);

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

        unsafe {
            std::mem::swap(&mut (*this).sum, &mut (*p).sum);
            std::mem::swap(&mut (*this).lazy, &mut (*p).lazy);
        }

        Self::update(p);
    }

    fn status(this: *mut Self) -> i32 {
        let par = Self::par_of(this).unwrap();

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
            let this = unsafe { &mut *this };

            if this.rev {
                std::mem::swap(&mut this.lc, &mut this.rc);
                Self::reverse(this.lc);
                Self::reverse(this.rc);
                this.rev = false;
            }

            if this.lazy != A::update_id() {
                let lc = this.lc;
                if !lc.is_null() {
                    let lc = unsafe { &mut *lc };
                    lc.lazy = A::update(lc.lazy.clone(), this.lazy.clone());
                }

                let rc = this.rc;
                if !rc.is_null() {
                    let rc = unsafe { &mut *rc };
                    rc.lazy = A::update(rc.lazy.clone(), this.lazy.clone());
                }

                this.value = A::convert(this.value.clone(), this.lazy.clone(), 1);
                this.sum = A::convert(this.sum.clone(), this.lazy.clone(), this.size);
                this.lazy = A::update_id();
            }
        }
    }

    fn update(this: *mut Self) {
        assert!(!this.is_null());

        let this = unsafe { &mut *this };
        Self::pushdown(this.lc);
        Self::pushdown(this.rc);

        this.size = 1 + Self::size_of(this.lc) + Self::size_of(this.rc);

        this.sum = this.value.clone();

        if !this.lc.is_null() {
            let lc = unsafe { &mut *this.lc };
            this.sum = A::fold(this.sum.clone(), lc.sum.clone());
        }

        if !this.rc.is_null() {
            let rc = unsafe { &mut *this.rc };
            this.sum = A::fold(this.sum.clone(), rc.sum.clone());
        }
    }

    fn splay(this: *mut Self) {
        while Self::status(this) != 0 {
            let par = Self::par_of(this).unwrap();

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

    fn par_of(this: *mut Self) -> Option<*mut Self> {
        (!this.is_null()).then_some(unsafe { (*this).par })
    }
}

/// 遅延スプレー木
pub struct LazySplayTree<A: Action> {
    root: Cell<*mut Node<A>>,
}

impl<A: Action> LazySplayTree<A> {
    /// `LazySplayTree<A>`を生成
    pub fn new() -> Self {
        let root = Cell::new(ptr::null_mut());
        Self { root }
    }
}

impl<A: Action> LazySplayTree<A>
where
    A::Output: Clone,
    A::Lazy: Clone + PartialEq,
{
    /// 値`value`をもつノード一つのみからなる`SplayTree<M>`を生成
    pub fn singleton(value: A::Output) -> Self {
        let root = Cell::new(Box::into_raw(Box::new(Node::new(value))));
        Self { root }
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
    pub fn get(&self, index: usize) -> Option<&A::Output> {
        let node = Node::get(self.root.get(), index);
        self.root.set(node);

        if node.is_null() {
            None
        } else {
            unsafe { Some(&(*node).value) }
        }
    }

    /// `index`番目の要素を`value`に変更する
    pub fn set(&mut self, index: usize, value: A::Output) {
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
    pub fn insert(&mut self, index: usize, value: A::Output) {
        let (l, r) = Node::split(self.root.get(), index);
        let node = Box::into_raw(Box::new(Node::new(value)));
        let root = Node::merge(l, Node::merge(node, r));
        self.root.set(root);
    }

    /// `index`番目の要素を削除して、値を返す
    pub fn remove(&mut self, index: usize) -> Option<A::Output> {
        let (l, r) = Node::split(self.root.get(), index);
        let (m, r) = Node::split(r, 1);

        let ret = if m.is_null() {
            None
        } else {
            let m = unsafe { Box::from_raw(m) };
            Some(m.value)
        };

        self.root.set(Node::merge(l, r));
        ret
    }

    fn range(&self, start: usize, end: usize) -> (*mut Node<A>, *mut Node<A>, *mut Node<A>) {
        let (m, r) = Node::split(self.root.get(), end);
        let (l, m) = Node::split(m, start);
        (l, m, r)
    }

    /// `start..end`の範囲を反転させる
    pub fn reverse(&mut self, Range { start, end }: Range<usize>) {
        let (l, m, r) = self.range(start, end);
        Node::reverse(m);
        self.root.set(Node::merge(Node::merge(l, m), r));
    }

    /// `start..end`の範囲でのモノイドの演算の結果を返す
    pub fn fold(&self, Range { start, end }: Range<usize>) -> A::Output {
        let (l, m, r) = self.range(start, end);
        let ret = if m.is_null() {
            A::fold_id()
        } else {
            unsafe { (*m).sum.clone() }
        };
        self.root.set(Node::merge(Node::merge(l, m), r));
        ret
    }

    /// `start..end`の範囲にモノイドの演算を施す。
    pub fn update(&self, Range { start, end }: Range<usize>, lazy: A::Lazy) {
        let (l, m, r) = self.range(start, end);
        if !m.is_null() {
            unsafe {
                (*m).lazy = lazy;
            }
        };
        self.root.set(Node::merge(Node::merge(l, m), r));
    }

    /// 先頭に値を追加する
    pub fn push_first(&mut self, value: A::Output) {
        let left = Self::singleton(value);
        self.merge_left(left);
    }
    /// 末尾に値を追加する
    pub fn push_last(&mut self, value: A::Output) {
        let right = Self::singleton(value);
        self.merge_right(right);
    }
    /// 先頭の値を削除する
    pub fn pop_first(&mut self) -> Option<A::Output> {
        self.remove(0)
    }
    /// 末尾の値を削除する
    pub fn pop_last(&mut self) -> Option<A::Output> {
        if self.is_empty() {
            None
        } else {
            self.remove(self.len() - 1)
        }
    }
}

impl<A: Action> Default for LazySplayTree<A> {
    fn default() -> Self {
        Self::new()
    }
}
