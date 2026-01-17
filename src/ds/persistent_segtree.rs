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
    monoid: M,
    root: Option<Rc<RefCell<Node<M::Element>>>>,
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
        let root = Some(Self::__init(&monoid, 0, to, &a));
        Self {
            monoid,
            root,
            to,
            original_size: n,
        }
    }

    fn __init(
        monoid: &M,
        from: usize,
        to: usize,
        seq: &[M::Element],
    ) -> Rc<RefCell<Node<M::Element>>> {
        if to - from == 1 {
            Rc::new(RefCell::new(Node::new(seq[from].clone())))
        } else {
            let mid = (from + to) / 2;
            let mut node = Node::new(monoid.id());

            let lv = if seq.len() > from {
                let left = Self::__init(monoid, from, mid, seq);
                let lv = left.borrow().value.clone();
                node.left = Some(left);
                lv
            } else {
                monoid.id()
            };

            let rv = if seq.len() > mid {
                let right = Self::__init(monoid, mid, to, seq);
                let rv = right.borrow().value.clone();
                node.right = Some(right);
                rv
            } else {
                monoid.id()
            };

            node.value = monoid.op(lv, rv);

            Rc::new(RefCell::new(node))
        }
    }

    fn __set(
        monoid: &M,
        node: Rc<RefCell<Node<M::Element>>>,
        from: usize,
        to: usize,
        pos: usize,
        value: &M::Element,
    ) -> Rc<RefCell<Node<M::Element>>> {
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
                .map(|left| Self::__set(monoid, left, from, mid, pos, value));
            let rp = node
                .borrow()
                .right
                .clone()
                .map(|right| Self::__set(monoid, right, mid, to, pos, value));

            let mut s = Node::new(
                monoid.op(
                    lp.as_ref()
                        .map_or(monoid.id(), |l| l.borrow().value.clone()),
                    rp.as_ref()
                        .map_or(monoid.id(), |r| r.borrow().value.clone()),
                ),
            );

            s.left = lp;
            s.right = rp;

            Rc::new(RefCell::new(s))
        }
    }

    /// `i`番目の要素を`value`にする。
    pub fn assign(&self, i: usize, value: M::Element) -> Self {
        let new_root = Self::__set(
            &self.monoid,
            self.root.clone().unwrap(),
            0,
            self.to,
            i,
            &value,
        );

        Self {
            monoid: self.monoid.clone(),
            root: Some(new_root),
            to: self.to,
            original_size: self.original_size,
        }
    }

    fn __fold(
        monoid: &M,
        node: Rc<RefCell<Node<M::Element>>>,
        from: usize,
        to: usize,
        l: usize,
        r: usize,
    ) -> M::Element {
        if l <= from && to <= r {
            node.borrow().value.clone()
        } else if to <= l || r <= from {
            monoid.id()
        } else {
            let mid = (from + to) / 2;

            let lv = node.borrow().left.clone().map_or(monoid.id(), |left| {
                Self::__fold(monoid, left, from, mid, l, r)
            });

            let rv = node.borrow().right.clone().map_or(monoid.id(), |right| {
                Self::__fold(monoid, right, mid, to, l, r)
            });

            monoid.op(lv, rv)
        }
    }

    /// 範囲`range`で計算を集約して返す。
    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Element {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        Self::__fold(
            &self.monoid,
            self.root.clone().unwrap(),
            0,
            self.to,
            start,
            end,
        )
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
