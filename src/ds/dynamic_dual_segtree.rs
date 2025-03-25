//! 動的双対セグメント木
use crate::algebra::traits::Monoid;
use crate::misc::nullable_usize::NullableUsize;
use std::ops::Range;

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    left: NullableUsize,
    right: NullableUsize,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: NullableUsize::NULL,
            right: NullableUsize::NULL,
        }
    }
}

/// 動的双対セグメント木
#[derive(Clone, Debug)]
pub struct DynamicDualSegtree<M: Monoid> {
    data: Vec<Node<M::Element>>,
    root: NullableUsize,
    monoid: M,
    to: usize,
}

impl<M: Monoid> DynamicDualSegtree<M>
where
    M::Element: Clone,
{
    /// [`DynamicDualSegtree<M>`]を生成する。
    pub fn new(monoid: M) -> Self {
        Self {
            data: vec![Node::new(monoid.id())],
            root: NullableUsize(0),
            monoid,
            to: 1,
        }
    }

    fn propagate(&mut self, cur: usize, from: usize, to: usize) {
        if to - from > 1 {
            let mut cur_node = self.data[cur].clone();
            let value = cur_node.value.clone();

            let left = cur_node.left;
            if left.is_null() {
                let t = Node::new(self.monoid.id());
                cur_node.left = NullableUsize(self.data.len());
                self.data.push(t);
            }
            let left = cur_node.left.0;
            let lv = self.data[left].value.clone();
            self.data[left].value = self.monoid.op(value.clone(), lv);

            let right = cur_node.right;
            if right.is_null() {
                let t = Node::new(self.monoid.id());
                cur_node.right = NullableUsize(self.data.len());
                self.data.push(t);
            }
            let right = cur_node.right.0;
            let rv = self.data[right].value.clone();
            self.data[right].value = self.monoid.op(value, rv);

            cur_node.value = self.monoid.id();
            self.data[cur] = cur_node;
        }
    }

    #[allow(clippy::collapsible_else_if)]
    fn update_(
        &mut self,
        cur: usize,
        from: usize,
        to: usize,
        s: usize,
        t: usize,
        value: &M::Element,
    ) {
        if to - from == 1 {
            if s <= from && to <= t {
                let cur_value = unsafe { self.data.get_unchecked(cur).value.clone() };
                unsafe {
                    self.data.get_unchecked_mut(cur).value =
                        self.monoid.op(value.clone(), cur_value);
                }
            }
        } else {
            if to < s || t < from {
            } else if s <= from && to <= t {
                let cur_value = unsafe { self.data.get_unchecked(cur).value.clone() };
                unsafe {
                    self.data.get_unchecked_mut(cur).value =
                        self.monoid.op(value.clone(), cur_value);
                }
            } else {
                let mid = (from + to) / 2;
                self.propagate(cur, from, to);
                let cur = unsafe { self.data.get_unchecked(cur) };
                let left = cur.left;
                let right = cur.right;
                self.update_(left.0, from, mid, s, t, value);
                self.update_(right.0, mid, to, s, t, value);
            }
        }
    }

    /// 範囲`s..t`を`value`で更新する。
    pub fn update(&mut self, Range { start: s, end: t }: Range<usize>, value: M::Element) {
        loop {
            let root = self.root.0;

            if t <= self.to {
                break;
            }
            self.to *= 2;

            let mut new_root = Node::new(self.monoid.id());
            new_root.left = NullableUsize(root);

            self.root = NullableUsize(self.data.len());
            self.data.push(new_root);
        }

        self.update_(self.root.0, 0, self.to, s, t, &value);
    }

    fn get_(&mut self, cur: usize, from: usize, to: usize, i: usize) -> M::Element {
        if !(from..to).contains(&i) {
            return self.monoid.id();
        }

        if to - from == 1 {
            unsafe { self.data.get_unchecked(cur).value.clone() }
        } else {
            self.propagate(cur, from, to);

            let mid = (from + to) / 2;
            let cur = unsafe { self.data.get_unchecked(cur) };
            if i < mid {
                self.get_(cur.left.0, from, mid, i)
            } else {
                self.get_(cur.right.0, mid, to, i)
            }
        }
    }

    /// `i`番目の要素を取得する。
    pub fn get(&mut self, i: usize) -> M::Element {
        self.get_(self.root.0, 0, self.to, i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::sum::*;
    use crate::testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let n = 1000;
        let m = Sum::<u32>::new();

        let mut a = vec![m.id(); n];
        let mut seg = DynamicDualSegtree::new(m.clone());

        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let lr = rand_range(&mut rng, 0..n);
            let x = rng.gen_range(0..10000);

            seg.update(lr.clone(), x);
            a[lr].iter_mut().for_each(|e| *e += x);

            assert_eq!(a, (0..n).map(|i| seg.get(i)).collect::<Vec<_>>());
        }
    }
}
