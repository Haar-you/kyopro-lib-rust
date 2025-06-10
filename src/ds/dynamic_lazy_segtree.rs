//! 動的遅延セグメント木
use crate::algebra::action::Action;
use std::ops::Range;
use std::ptr;

#[derive(Clone, Debug)]
struct Node<A: Action> {
    value: A::Output,
    lazy: A::Lazy,
    left: *mut Node<A>,
    right: *mut Node<A>,
}

impl<A: Action> Node<A> {
    fn new() -> Self {
        Self {
            value: A::fold_id(),
            lazy: A::update_id(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }
}

/// 動的遅延セグメント木
#[derive(Clone, Debug)]
pub struct DynamicLazySegtree<A: Action> {
    root: *mut Node<A>,
    to: usize,
}

impl<A: Action> Default for DynamicLazySegtree<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Action> DynamicLazySegtree<A> {
    /// `DynamicLazySegtree<A>`を生成する。
    pub fn new() -> Self {
        Self {
            root: Box::into_raw(Box::new(Node::new())),
            to: 1,
        }
    }
}

impl<A: Action> DynamicLazySegtree<A>
where
    A::Output: Clone + PartialEq,
    A::Lazy: Clone + PartialEq,
{
    fn _propagate(&self, t: *mut Node<A>, from: usize, to: usize) {
        assert!(!t.is_null());
        let lazy = unsafe { (*t).lazy.clone() };

        if lazy == A::update_id() {
            return;
        }
        if to - from > 1 {
            unsafe {
                if (*t).left.is_null() {
                    (*t).left = Box::into_raw(Box::new(Node::new()));
                }
                let left = (*t).left;
                (*left).lazy = A::update((*left).lazy.clone(), lazy.clone());

                if (*t).right.is_null() {
                    (*t).right = Box::into_raw(Box::new(Node::new()));
                }
                let right = (*t).right;
                (*right).lazy = A::update((*right).lazy.clone(), lazy.clone());
            }
        }
        let len = to - from;
        unsafe {
            (*t).value = A::convert((*t).value.clone(), lazy, len);
            (*t).lazy = A::update_id();
        }
    }

    fn _update(
        &self,
        mut cur: *mut Node<A>,
        from: usize,
        to: usize,
        s: usize,
        t: usize,
        value: A::Lazy,
    ) -> *mut Node<A> {
        if cur.is_null() {
            cur = Box::into_raw(Box::new(Node::new()));
        }

        self._propagate(cur, from, to);

        if to - from == 1 {
            if s <= from && to <= t {
                unsafe {
                    (*cur).lazy = A::update((*cur).lazy.clone(), value);
                }
            }
            self._propagate(cur, from, to);
            return cur;
        }

        if to < s || t < from {
            return cur;
        }
        if s <= from && to <= t {
            unsafe {
                (*cur).lazy = A::update((*cur).lazy.clone(), value);
            }
            self._propagate(cur, from, to);
            return cur;
        }

        let mid = (from + to) / 2;
        unsafe {
            (*cur).left = self._update((*cur).left, from, mid, s, t, value.clone());
            (*cur).right = self._update((*cur).right, mid, to, s, t, value);
            (*cur).value = A::fold((*(*cur).left).value.clone(), (*(*cur).right).value.clone());
        }
        cur
    }

    /// 範囲`s..t`を`value`で更新する。
    pub fn update(&mut self, Range { start: s, end: t }: Range<usize>, value: A::Lazy) {
        loop {
            if t <= self.to {
                break;
            }
            self.to *= 2;

            let mut new_root = Box::new(Node::new());
            new_root.left = self.root;

            self.root = Box::into_raw(new_root);
        }

        self._update(self.root, 0, self.to, s, t, value);
    }

    fn _fold(&self, cur: *mut Node<A>, from: usize, to: usize, s: usize, t: usize) -> A::Output {
        if cur.is_null() {
            return A::fold_id();
        }

        self._propagate(cur, from, to);
        if to <= s || t <= from {
            return A::fold_id();
        }
        if s <= from && to <= t {
            return unsafe { (*cur).value.clone() };
        }

        let mid = (from + to) / 2;
        let lv = self._fold(unsafe { (*cur).left }, from, mid, s, t);
        let rv = self._fold(unsafe { (*cur).right }, mid, to, s, t);

        A::fold(lv, rv)
    }

    /// 範囲`s..t`で計算を集約する。
    pub fn fold(&mut self, Range { start: s, end: t }: Range<usize>) -> A::Output {
        self._fold(self.root, 0, self.to, s, t)
    }
}
