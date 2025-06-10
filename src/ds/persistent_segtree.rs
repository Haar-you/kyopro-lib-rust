//! 永続セグメントツリー

use std::cell::RefCell;
use std::ops::RangeBounds;
use std::rc::Rc;

use crate::algebra::traits::Monoid;
use crate::misc::range::range_bounds_to_range;

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

/// 永続セグメントツリー
#[derive(Clone, Debug)]
pub struct PersistentSegtree<M: Monoid> {
    root: Option<Rc<RefCell<Node<M>>>>,
    to: usize,
    original_size: usize,
}

impl<M: Monoid + Clone> PersistentSegtree<M> {
    /// 長さ`n`の[`PersistentSegtree`]を生成する。
    pub fn new(n: usize) -> Self {
        let seq = vec![M::id(); n];
        Self::from_vec(seq)
    }

    /// [`Vec`]から[`PersistentSegtree`]を構築する。
    pub fn from_vec(a: Vec<M>) -> Self {
        let n = a.len();
        let to = n.next_power_of_two();
        let root = Some(Self::__init(0, to, &a));
        Self {
            root,
            to,
            original_size: n,
        }
    }

    fn __init(from: usize, to: usize, seq: &[M]) -> Rc<RefCell<Node<M>>> {
        if to - from == 1 {
            Rc::new(RefCell::new(Node::new(seq[from].clone())))
        } else {
            let mid = (from + to) / 2;
            let mut node = Node::new(M::id());

            let lv = if seq.len() > from {
                let left = Self::__init(from, mid, seq);
                let lv = left.borrow().value.clone();
                node.left = Some(left);
                lv
            } else {
                M::id()
            };

            let rv = if seq.len() > mid {
                let right = Self::__init(mid, to, seq);
                let rv = right.borrow().value.clone();
                node.right = Some(right);
                rv
            } else {
                M::id()
            };

            node.value = M::op(lv, rv);

            Rc::new(RefCell::new(node))
        }
    }

    fn __set(
        node: Rc<RefCell<Node<M>>>,
        from: usize,
        to: usize,
        pos: usize,
        value: &M,
    ) -> Rc<RefCell<Node<M>>> {
        if to <= pos || pos < from {
            node
        } else if pos <= from && to <= pos + 1 {
            Rc::new(RefCell::new(Node::new(value.clone())))
        } else {
            let mid = (from + to) / 2;

            let lp = node
                .borrow()
                .left
                .clone()
                .map(|left| Self::__set(left, from, mid, pos, value));
            let rp = node
                .borrow()
                .right
                .clone()
                .map(|right| Self::__set(right, mid, to, pos, value));

            let mut s = Node::new(M::op(
                lp.as_ref().map_or(M::id(), |l| l.borrow().value.clone()),
                rp.as_ref().map_or(M::id(), |r| r.borrow().value.clone()),
            ));

            s.left = lp;
            s.right = rp;

            Rc::new(RefCell::new(s))
        }
    }

    /// `i`番目の要素を`value`にする。
    pub fn assign(&self, i: usize, value: M) -> Self {
        let new_root = Self::__set(self.root.clone().unwrap(), 0, self.to, i, &value);

        Self {
            root: Some(new_root),
            to: self.to,
            original_size: self.original_size,
        }
    }

    fn __fold(node: Rc<RefCell<Node<M>>>, from: usize, to: usize, l: usize, r: usize) -> M {
        if l <= from && to <= r {
            node.borrow().value.clone()
        } else if to <= l || r <= from {
            M::id()
        } else {
            let mid = (from + to) / 2;

            let lv = node
                .borrow()
                .left
                .clone()
                .map_or(M::id(), |left| Self::__fold(left, from, mid, l, r));

            let rv = node
                .borrow()
                .right
                .clone()
                .map_or(M::id(), |right| Self::__fold(right, mid, to, l, r));

            M::op(lv, rv)
        }
    }

    /// 範囲`range`で計算を集約して返す。
    pub fn fold(&self, range: impl RangeBounds<usize>) -> M {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        Self::__fold(self.root.clone().unwrap(), 0, self.to, start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algebra::sum::*, misc::vec_map::VecMap};

    #[test]
    fn test() {
        let a = vec![0, 1, 3, 9, 4, 8, 2];
        let a = a.map(Sum);
        let seg = PersistentSegtree::<Sum<u64>>::from_vec(a);

        dbg!(seg.fold(0..5));

        let s1 = seg.assign(0, Sum(10));

        dbg!(s1.fold(0..5));
        dbg!(seg.fold(0..5));

        let s2 = seg.assign(2, Sum(6));

        dbg!(s1.fold(0..5));
        dbg!(s2.fold(0..5));
        dbg!(seg.fold(0..5));
    }
}
