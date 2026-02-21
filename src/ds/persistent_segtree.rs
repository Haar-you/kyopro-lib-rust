//! 永続セグメントツリー

use std::ops::RangeBounds;
use std::ptr;

use crate::algebra::traits::Monoid;
use crate::misc::range::range_bounds_to_range;

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    left: *mut Self,
    right: *mut Self,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }
}

/// 永続セグメントツリー
#[derive(Clone, Debug)]
pub struct PersistentSegtree<M: Monoid> {
    monoid: M,
    root: *mut Node<M::Element>,
    to: usize,
    original_size: usize,
}

impl<M: Monoid + Clone> PersistentSegtree<M>
where
    M::Element: Clone,
{
    /// 長さ`n`の[`PersistentSegtree`]を生成する。
    pub fn new(monoid: M, n: usize) -> Self {
        let seq = vec![monoid.id(); n];
        Self::from_vec(monoid, seq)
    }

    /// [`Vec`]から[`PersistentSegtree`]を構築する。
    pub fn from_vec(monoid: M, a: Vec<M::Element>) -> Self {
        let n = a.len();
        let to = n.next_power_of_two();
        let root = Self::__init(&monoid, 0, to, &a);
        Self {
            monoid,
            root,
            to,
            original_size: n,
        }
    }

    fn __init(monoid: &M, from: usize, to: usize, seq: &[M::Element]) -> *mut Node<M::Element> {
        if to - from == 1 {
            Box::into_raw(Box::new(Node::new(seq[from].clone())))
        } else {
            let mid = (from + to) / 2;
            let mut node = Node::new(monoid.id());

            let lv = if seq.len() > from {
                let left = Self::__init(monoid, from, mid, seq);
                node.left = left;
                assert!(!left.is_null());
                unsafe { (*left).value.clone() }
            } else {
                monoid.id()
            };

            let rv = if seq.len() > mid {
                let right = Self::__init(monoid, mid, to, seq);
                node.right = right;
                assert!(!right.is_null());
                unsafe { (*right).value.clone() }
            } else {
                monoid.id()
            };

            node.value = monoid.op(lv, rv);

            Box::into_raw(Box::new(node))
        }
    }

    fn __set(
        monoid: &M,
        node: *mut Node<M::Element>,
        from: usize,
        to: usize,
        pos: usize,
        value: &M::Element,
    ) -> *mut Node<M::Element> {
        assert!(!node.is_null());

        if to <= pos || pos < from {
            node
        } else if pos <= from && to <= pos + 1 {
            Box::into_raw(Box::new(Node::new(value.clone())))
        } else {
            let mid = (from + to) / 2;

            let left = unsafe { (*node).left };
            let right = unsafe { (*node).right };

            let lp = if !left.is_null() {
                Self::__set(monoid, left, from, mid, pos, value)
            } else {
                left
            };

            let rp = if !right.is_null() {
                Self::__set(monoid, right, mid, to, pos, value)
            } else {
                right
            };

            let mut value = monoid.id();
            if !lp.is_null() {
                value = monoid.op(value, unsafe { (*lp).value.clone() });
            }
            if !rp.is_null() {
                value = monoid.op(value, unsafe { (*rp).value.clone() });
            }

            let mut s = Node::new(value);
            s.left = lp;
            s.right = rp;

            Box::into_raw(Box::new(s))
        }
    }

    /// `i`番目の要素を`value`にする。
    pub fn assign(&self, i: usize, value: M::Element) -> Self {
        let new_root = Self::__set(&self.monoid, self.root, 0, self.to, i, &value);

        Self {
            monoid: self.monoid.clone(),
            root: new_root,
            to: self.to,
            original_size: self.original_size,
        }
    }

    fn __fold(
        monoid: &M,
        node: *mut Node<M::Element>,
        from: usize,
        to: usize,
        l: usize,
        r: usize,
    ) -> M::Element {
        assert!(!node.is_null());

        if l <= from && to <= r {
            unsafe { (*node).value.clone() }
        } else if to <= l || r <= from {
            monoid.id()
        } else {
            let mid = (from + to) / 2;

            let left = unsafe { (*node).left };
            let right = unsafe { (*node).right };

            let lv = if left.is_null() {
                monoid.id()
            } else {
                Self::__fold(monoid, left, from, mid, l, r)
            };

            let rv = if right.is_null() {
                monoid.id()
            } else {
                Self::__fold(monoid, right, mid, to, l, r)
            };

            monoid.op(lv, rv)
        }
    }

    /// 範囲`range`で計算を集約して返す。
    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Element {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        Self::__fold(&self.monoid, self.root, 0, self.to, start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::sum::*;

    #[test]
    fn test() {
        let m = Sum::<u64>::new();

        let a = vec![0, 1, 3, 9, 4, 8, 2];
        let seg = PersistentSegtree::from_vec(m, a);

        dbg!(seg.fold(0..5));

        let s1 = seg.assign(0, 10);

        dbg!(s1.fold(0..5));
        dbg!(seg.fold(0..5));

        let s2 = seg.assign(2, 6);

        dbg!(s1.fold(0..5));
        dbg!(s2.fold(0..5));
        dbg!(seg.fold(0..5));
    }
}
