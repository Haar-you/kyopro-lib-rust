//! 動的遅延セグメント木
use crate::algebra::{act::Act, traits::*};
use std::ops::Range;
use std::ptr;

#[derive(Clone, Debug)]
struct Node<M: Monoid, A: Act<M>> {
    value: M::Element,
    lazy: A::Element,
    left: *mut Node<M, A>,
    right: *mut Node<M, A>,
}

impl<M: Monoid, A: Act<M>> Node<M, A> {
    fn new(monoid: &M, act: &A) -> Self {
        Self {
            value: monoid.id(),
            lazy: act.id(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }
}

/// 動的遅延セグメント木
#[derive(Clone, Debug)]
pub struct DynamicLazySegtree<M: Monoid, A: Act<M>> {
    monoid: M,
    act: A,
    root: *mut Node<M, A>,
    to: usize,
}

impl<M: Monoid, A: Act<M>> DynamicLazySegtree<M, A> {
    /// `DynamicLazySegtree<A>`を生成する。
    pub fn new(monoid: M, act: A) -> Self {
        Self {
            root: Box::into_raw(Box::new(Node::new(&monoid, &act))),
            to: 1,
            monoid,
            act,
        }
    }
}

impl<M: Monoid, A: Act<M>> DynamicLazySegtree<M, A>
where
    M::Element: Clone + PartialEq,
    A::Element: Clone + PartialEq,
{
    fn _propagate(&self, t: *mut Node<M, A>, from: usize, to: usize) {
        assert!(!t.is_null());
        let lazy = unsafe { (*t).lazy.clone() };

        if lazy == self.act.id() {
            return;
        }
        if to - from > 1 {
            unsafe {
                if (*t).left.is_null() {
                    (*t).left = Box::into_raw(Box::new(Node::new(&self.monoid, &self.act)));
                }
                let left = (*t).left;
                (*left).lazy = self.act.op((*left).lazy.clone(), lazy.clone());

                if (*t).right.is_null() {
                    (*t).right = Box::into_raw(Box::new(Node::new(&self.monoid, &self.act)));
                }
                let right = (*t).right;
                (*right).lazy = self.act.op((*right).lazy.clone(), lazy.clone());
            }
        }
        let len = to - from;
        unsafe {
            (*t).value = self.act.act_n(&self.monoid, (*t).value.clone(), lazy, len);
            (*t).lazy = self.act.id();
        }
    }

    fn _update(
        &self,
        mut cur: *mut Node<M, A>,
        from: usize,
        to: usize,
        s: usize,
        t: usize,
        value: A::Element,
    ) -> *mut Node<M, A> {
        if cur.is_null() {
            cur = Box::into_raw(Box::new(Node::new(&self.monoid, &self.act)));
        }

        self._propagate(cur, from, to);

        if to - from == 1 {
            if s <= from && to <= t {
                unsafe {
                    (*cur).lazy = self.act.op((*cur).lazy.clone(), value);
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
                (*cur).lazy = self.act.op((*cur).lazy.clone(), value);
            }
            self._propagate(cur, from, to);
            return cur;
        }

        let mid = (from + to) / 2;
        unsafe {
            (*cur).left = self._update((*cur).left, from, mid, s, t, value.clone());
            (*cur).right = self._update((*cur).right, mid, to, s, t, value);
            (*cur).value = self
                .monoid
                .op((*(*cur).left).value.clone(), (*(*cur).right).value.clone());
        }
        cur
    }

    /// 範囲`s..t`を`value`で更新する。
    pub fn update(&mut self, Range { start: s, end: t }: Range<usize>, value: A::Element) {
        loop {
            if t <= self.to {
                break;
            }
            self.to *= 2;

            let mut new_root = Box::new(Node::new(&self.monoid, &self.act));
            new_root.left = self.root;

            self.root = Box::into_raw(new_root);
        }

        self._update(self.root, 0, self.to, s, t, value);
    }

    fn _fold(
        &self,
        cur: *mut Node<M, A>,
        from: usize,
        to: usize,
        s: usize,
        t: usize,
    ) -> M::Element {
        if cur.is_null() {
            return self.monoid.id();
        }

        self._propagate(cur, from, to);
        if to <= s || t <= from {
            return self.monoid.id();
        }
        if s <= from && to <= t {
            return unsafe { (*cur).value.clone() };
        }

        let mid = (from + to) / 2;
        let lv = self._fold(unsafe { (*cur).left }, from, mid, s, t);
        let rv = self._fold(unsafe { (*cur).right }, mid, to, s, t);

        self.monoid.op(lv, rv)
    }

    /// 範囲`s..t`で計算を集約する。
    pub fn fold(&mut self, Range { start: s, end: t }: Range<usize>) -> M::Element {
        self._fold(self.root, 0, self.to, s, t)
    }
}
