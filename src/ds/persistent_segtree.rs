//! 永続セグメントツリー

use std::cell::RefCell;
use std::ops::RangeBounds;
use std::rc::Rc;

use crate::algebra::traits::Monoid;
use crate::utils::range::range_bounds_to_range;

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
    root: Option<Rc<RefCell<Node<M::Element>>>>,
    monoid: M,
    to: usize,
    original_size: usize,
}

impl<M: Monoid + Clone> PersistentSegtree<M>
where
    M::Element: Clone,
{
    /// 長さ`n`の[`PersistentSegtree`]を生成する。
    pub fn new(n: usize, monoid: M) -> Self {
        let seq = vec![monoid.id(); n];
        Self::from_vec(seq, monoid)
    }

    /// [`Vec`]から[`PersistentSegtree`]を構築する。
    pub fn from_vec(a: Vec<M::Element>, monoid: M) -> Self {
        let n = a.len();
        let to = n.next_power_of_two();
        let root = Some(Self::__init(0, to, &a, &monoid));
        Self {
            root,
            monoid,
            to,
            original_size: n,
        }
    }

    fn __init(
        from: usize,
        to: usize,
        seq: &[M::Element],
        monoid: &M,
    ) -> Rc<RefCell<Node<M::Element>>> {
        if to - from == 1 {
            Rc::new(RefCell::new(Node::new(seq[from].clone())))
        } else {
            let mid = (from + to) / 2;
            let mut node = Node::new(monoid.id());

            let lv = if seq.len() > from {
                let left = Self::__init(from, mid, seq, monoid);
                let lv = left.borrow().value.clone();
                node.left = Some(left);
                lv
            } else {
                monoid.id()
            };

            let rv = if seq.len() > mid {
                let right = Self::__init(mid, to, seq, monoid);
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
        node: Rc<RefCell<Node<M::Element>>>,
        from: usize,
        to: usize,
        pos: usize,
        value: &M::Element,
        monoid: &M,
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
                .map(|left| Self::__set(left, from, mid, pos, value, monoid));
            let rp = node
                .borrow()
                .right
                .clone()
                .map(|right| Self::__set(right, mid, to, pos, value, monoid));

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
            self.root.clone().unwrap(),
            0,
            self.to,
            i,
            &value,
            &self.monoid,
        );

        Self {
            root: Some(new_root),
            monoid: self.monoid.clone(),
            to: self.to,
            original_size: self.original_size,
        }
    }

    fn __fold(
        node: Rc<RefCell<Node<M::Element>>>,
        from: usize,
        to: usize,
        l: usize,
        r: usize,
        monoid: &M,
    ) -> M::Element {
        if l <= from && to <= r {
            node.borrow().value.clone()
        } else if to <= l || r <= from {
            monoid.id()
        } else {
            let mid = (from + to) / 2;

            let lv = node.borrow().left.clone().map_or(monoid.id(), |left| {
                Self::__fold(left, from, mid, l, r, monoid)
            });

            let rv = node.borrow().right.clone().map_or(monoid.id(), |right| {
                Self::__fold(right, mid, to, l, r, monoid)
            });

            monoid.op(lv, rv)
        }
    }

    /// 範囲`range`で計算を集約して返す。
    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Element {
        let (start, end) = range_bounds_to_range(range, 0, self.original_size);
        Self::__fold(
            self.root.clone().unwrap(),
            0,
            self.to,
            start,
            end,
            &self.monoid,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::sum::*;

    #[test]
    fn test() {
        let a = vec![0, 1, 3, 9, 4, 8, 2];
        let seg = PersistentSegtree::from_vec(a, Sum::<u64>::new());

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
