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

#[derive(Clone)]
struct Manager<M> {
    monoid: M,
}

struct Node<M: Monoid> {
    value: M::Element,
    sum: M::Element,
    size: usize,
    rev: bool,
    lc: *mut Self,
    rc: *mut Self,
    par: *mut Self,
}

impl<M: Monoid> Manager<M>
where
    M::Element: Clone,
{
    fn new(monoid: M) -> Self {
        Self { monoid }
    }

    fn create(&self, value: M::Element) -> Node<M> {
        Node {
            value,
            sum: self.monoid.id(),
            size: 1,
            rev: false,
            lc: ptr::null_mut(),
            rc: ptr::null_mut(),
            par: ptr::null_mut(),
        }
    }

    fn get_sum(&self, this: *mut Node<M>) -> M::Element {
        if this.is_null() {
            self.monoid.id()
        } else {
            unsafe { (*this).sum.clone() }
        }
    }

    fn set_value(&self, this: *mut Node<M>, value: M::Element) {
        assert!(!this.is_null());
        unsafe {
            (*this).value = value;
        }
    }

    fn rotate(&self, this: *mut Node<M>) {
        let p = self.par_of(this).unwrap();
        let pp = self.par_of(p).unwrap();

        if self.left_of(p).unwrap() == this {
            let c = self.right_of(this).unwrap();
            self.set_left(p, c);
            self.set_right(this, p);
        } else {
            let c = self.left_of(this).unwrap();
            self.set_right(p, c);
            self.set_left(this, p);
        }

        if !pp.is_null() {
            let pp = unsafe { &mut *pp };
            if (*pp).lc == p {
                (*pp).lc = this;
            }
            if (*pp).rc == p {
                (*pp).rc = this;
            }
        }

        assert!(!this.is_null());
        let this = unsafe { &mut *this };
        this.par = pp;

        self.update(p);
        self.update(this);
    }

    fn status(&self, this: *mut Node<M>) -> i32 {
        let par = self.par_of(this).unwrap();
        if par.is_null() {
            return 0;
        }
        let par = unsafe { &*par };
        if par.lc == this {
            return 1;
        }
        if par.rc == this {
            return -1;
        }

        unreachable!()
    }

    fn reverse(&self, this: *mut Node<M>) {
        if !this.is_null() {
            unsafe {
                (*this).rev ^= true;
            }
        }
    }

    fn pushdown(&self, this: *mut Node<M>) {
        if !this.is_null() {
            let this = unsafe { &mut *this };
            if this.rev {
                std::mem::swap(&mut this.lc, &mut this.rc);
                self.reverse(this.lc);
                self.reverse(this.rc);
                this.rev = false;
            }
            self.update(this);
        }
    }

    fn update(&self, this: *mut Node<M>) {
        assert!(!this.is_null());
        let this = unsafe { &mut *this };
        this.size = 1 + self.size_of(this.lc) + self.size_of(this.rc);

        this.sum = this.value.clone();
        if !this.lc.is_null() {
            this.sum = self.monoid.op(this.sum.clone(), self.get_sum(this.lc));
        }
        if !this.rc.is_null() {
            this.sum = self.monoid.op(this.sum.clone(), self.get_sum(this.rc));
        }
    }

    fn splay(&self, this: *mut Node<M>) {
        while self.status(this) != 0 {
            let par = self.par_of(this).unwrap();

            if self.status(par) == 0 {
                self.rotate(this);
            } else if self.status(this) == self.status(par) {
                self.rotate(par);
                self.rotate(this);
            } else {
                self.rotate(this);
                self.rotate(this);
            }
        }
    }

    fn get(&self, root: *mut Node<M>, mut index: usize) -> *mut Node<M> {
        if root.is_null() {
            return root;
        }

        let mut cur = root;

        loop {
            self.pushdown(cur);

            let left = self.left_of(cur).unwrap();
            let lsize = self.size_of(left);

            match index.cmp(&lsize) {
                Ordering::Less => {
                    cur = left;
                }
                Ordering::Equal => {
                    self.splay(cur);
                    return cur;
                }
                Ordering::Greater => {
                    cur = self.right_of(cur).unwrap();
                    index -= lsize + 1;
                }
            }
        }
    }

    fn merge(&self, left: *mut Node<M>, right: *mut Node<M>) -> *mut Node<M> {
        if left.is_null() {
            return right;
        }
        if right.is_null() {
            return left;
        }

        let cur = self.get(left, self.size_of(left) - 1);

        self.set_right(cur, right);
        self.update(right);
        self.update(cur);

        cur
    }

    fn split(&self, root: *mut Node<M>, index: usize) -> (*mut Node<M>, *mut Node<M>) {
        if root.is_null() {
            return (ptr::null_mut(), ptr::null_mut());
        }
        if index >= self.size_of(root) {
            return (root, ptr::null_mut());
        }

        let cur = self.get(root, index);
        let left = self.left_of(cur).unwrap();

        if !left.is_null() {
            self.set_par(left, ptr::null_mut());
            self.update(left);
        }
        self.set_left(cur, ptr::null_mut());
        self.update(cur);

        (left, cur)
    }

    fn traverse(&self, cur: *mut Node<M>, f: &mut impl FnMut(&M::Element)) {
        if !cur.is_null() {
            let cur = unsafe { &mut *cur };
            self.pushdown(cur);
            self.traverse(self.left_of(cur).unwrap(), f);
            f(&cur.value);
            self.traverse(self.right_of(cur).unwrap(), f);
        }
    }
}

impl<M: Monoid> Manager<M> {
    fn set_left(&self, this: *mut Node<M>, left: *mut Node<M>) {
        assert!(!this.is_null());
        unsafe { (*this).lc = left };
        if !left.is_null() {
            unsafe { (*left).par = this };
        }
    }

    fn set_right(&self, this: *mut Node<M>, right: *mut Node<M>) {
        assert!(!this.is_null());
        unsafe { (*this).rc = right };
        if !right.is_null() {
            unsafe { (*right).par = this };
        }
    }

    fn set_par(&self, this: *mut Node<M>, par: *mut Node<M>) {
        assert!(!this.is_null());
        unsafe { (*this).par = par };
    }

    fn size_of(&self, this: *mut Node<M>) -> usize {
        if this.is_null() {
            0
        } else {
            unsafe { (*this).size }
        }
    }

    fn left_of(&self, this: *mut Node<M>) -> Option<*mut Node<M>> {
        (!this.is_null()).then(|| unsafe { (*this).lc })
    }

    fn right_of(&self, this: *mut Node<M>) -> Option<*mut Node<M>> {
        (!this.is_null()).then(|| unsafe { (*this).rc })
    }

    fn par_of(&self, this: *mut Node<M>) -> Option<*mut Node<M>> {
        (!this.is_null()).then(|| unsafe { (*this).par })
    }

    fn clear(&self, this: *mut Node<M>) {
        if !this.is_null() {
            let lc = self.left_of(this).unwrap();
            let rc = self.right_of(this).unwrap();

            let _ = unsafe { Box::from_raw(this) };

            self.clear(lc);
            self.clear(rc);
        }
    }
}

/// スプレーツリー
pub struct SplayTree<M: Monoid> {
    man: Manager<M>,
    root: Cell<*mut Node<M>>,
}

impl<M: Monoid + Clone> SplayTree<M>
where
    M::Element: Clone,
{
    /// モノイド`m`をもつ`SplayTree<M>`を生成
    pub fn new(monoid: M) -> Self {
        Self {
            man: Manager::new(monoid),
            root: Cell::new(ptr::null_mut()),
        }
    }

    /// 値`value`をもつノード一つのみからなる`SplayTree<M>`を生成
    pub fn singleton(monoid: M, value: M::Element) -> Self {
        let man = Manager::new(monoid);
        let root = Box::new(man.create(value));

        Self {
            man,
            root: Cell::new(Box::into_raw(root)),
        }
    }

    /// スプレーツリーの要素数を返す
    pub fn len(&self) -> usize {
        self.man.size_of(self.root.get())
    }

    /// スプレーツリーが要素を持たなければ`true`を返す
    pub fn is_empty(&self) -> bool {
        self.root.get().is_null()
    }

    /// `index`番目の要素の参照を返す
    pub fn get(&self, index: usize) -> Option<&M::Element> {
        self.root.set(self.man.get(self.root.get(), index));
        let node = self.root.get();

        (!node.is_null()).then(|| unsafe { &(*node).value })
    }

    /// `index`番目の要素を`value`に変更する
    pub fn set(&mut self, index: usize, value: M::Element) {
        let root = self.man.get(self.root.get(), index);
        self.man.set_value(root, value);
        self.man.update(root);
        self.root.set(root);
    }

    /// 右側にスプレーツリーを結合する
    pub fn merge_right(&mut self, right: Self) {
        let root = self.man.merge(self.root.get(), right.root.get());
        right.root.set(ptr::null_mut());
        self.root.set(root);
    }

    /// 左側にスプレーツリーを結合する
    pub fn merge_left(&mut self, left: Self) {
        let root = self.man.merge(left.root.get(), self.root.get());
        left.root.set(ptr::null_mut());
        self.root.set(root);
    }

    /// 左側に`index`個の要素があるように、左右で分割する
    pub fn split(self, index: usize) -> (Self, Self) {
        let (l, r) = self.man.split(self.root.get(), index);
        self.root.set(ptr::null_mut());
        (
            Self {
                man: self.man.clone(),
                root: Cell::new(l),
            },
            Self {
                man: self.man.clone(),
                root: Cell::new(r),
            },
        )
    }

    /// 要素を`index`番目になるように挿入する
    pub fn insert(&mut self, index: usize, value: M::Element) {
        let (l, r) = self.man.split(self.root.get(), index);
        let node = Box::into_raw(Box::new(self.man.create(value)));
        let root = self.man.merge(l, self.man.merge(node, r));
        self.root.set(root);
    }

    /// `index`番目の要素を削除して、値を返す
    pub fn remove(&mut self, index: usize) -> Option<M::Element> {
        let (l, r) = self.man.split(self.root.get(), index);
        let (m, r) = self.man.split(r, 1);

        self.root.set(self.man.merge(l, r));

        (!m.is_null()).then(|| {
            let m = unsafe { Box::from_raw(m) };
            m.value
        })
    }

    /// `start..end`の範囲を反転させる
    pub fn reverse(&mut self, Range { start, end }: Range<usize>) {
        let (m, r) = self.man.split(self.root.get(), end);
        let (l, m) = self.man.split(m, start);

        self.man.reverse(m);

        let m = self.man.merge(l, m);
        let root = self.man.merge(m, r);
        self.root.set(root);
    }

    /// `start..end`の範囲でのモノイドの演算の結果を返す
    pub fn fold(&self, Range { start, end }: Range<usize>) -> M::Element {
        let (m, r) = self.man.split(self.root.get(), end);
        let (l, m) = self.man.split(m, start);

        let ret = self.man.get_sum(m);

        let m = self.man.merge(l, m);
        let root = self.man.merge(m, r);
        self.root.set(root);

        ret
    }

    /// 先頭に値を追加する
    pub fn push_first(&mut self, value: M::Element) {
        let left = Self::singleton(self.man.monoid.clone(), value);
        self.merge_left(left);
    }
    /// 末尾に値を追加する
    pub fn push_last(&mut self, value: M::Element) {
        let right = Self::singleton(self.man.monoid.clone(), value);
        self.merge_right(right);
    }
    /// 先頭の値を削除する
    pub fn pop_first(&mut self) -> Option<M::Element> {
        self.remove(0)
    }
    /// 末尾の値を削除する
    pub fn pop_last(&mut self) -> Option<M::Element> {
        self.remove(self.len().checked_sub(1)?)
    }

    /// 列の要素を始めから辿り、その参照を`f`に渡す。
    pub fn for_each(&self, mut f: impl FnMut(&M::Element)) {
        self.man.traverse(self.root.get(), &mut f);
    }
}

impl<M: Monoid> std::ops::Drop for SplayTree<M> {
    fn drop(&mut self) {
        self.man.clear(self.root.get());
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::sum::*;
    use my_testtools::rand_range;

    use rand::Rng;

    use super::*;

    #[test]
    fn test_empty() {
        let monoid = Sum::<u64>::new();
        let mut s = SplayTree::new(monoid);

        assert!(s.pop_first().is_none());
        assert!(s.pop_last().is_none());
    }

    #[test]
    fn test() {
        let t = 100;

        let mut rng = rand::thread_rng();

        let m = Sum::<u64>::new();
        let mut a = vec![];
        let mut st = SplayTree::new(m);

        for _ in 0..t {
            assert_eq!(a.len(), st.len());
            let n = a.len();

            let i = rng.gen_range(0..=n);
            let x = rng.gen::<u32>() as u64;

            a.insert(i, x);
            st.insert(i, x);

            assert_eq!(a.len(), st.len());
            let n = a.len();

            let Range { start: l, end: r } = rand_range(&mut rng, 0..n);
            assert_eq!(a[l..r].iter().cloned().fold_m(&m), st.fold(l..r));
        }
    }
}
