use crate::algebra::traits::Monoid;
use crate::utils::nullable_usize::NullableUsize;
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

#[derive(Clone, Debug)]
pub struct DynamicDualSegtree<M: Monoid> {
    data: Vec<Node<M::Output>>,
    root: NullableUsize,
    monoid: M,
    to: usize,
}

impl<T: Clone, M: Monoid<Output = T>> DynamicDualSegtree<M> {
    pub fn new(monoid: M) -> Self {
        Self {
            data: vec![Node::new(monoid.id())],
            root: NullableUsize(0),
            monoid,
            to: 1,
        }
    }

    unsafe fn propagate(&mut self, cur: usize, from: usize, to: usize) {
        if to - from > 1 {
            let value = self.data.get_unchecked(cur).value.clone();

            let left = self.data.get_unchecked(cur).left;
            if left.is_null() {
                let t = Node::new(self.monoid.id());
                self.data.get_unchecked_mut(cur).left = NullableUsize(self.data.len());
                self.data.push(t);
            }
            let left = self.data.get_unchecked(cur).left.0;
            let lv = self.data.get_unchecked(left).value.clone();
            self.data.get_unchecked_mut(left).value = self.monoid.op(value.clone(), lv);

            let right = self.data.get_unchecked(cur).right;
            if right.is_null() {
                let t = Node::new(self.monoid.id());
                self.data.get_unchecked_mut(cur).right = NullableUsize(self.data.len());
                self.data.push(t);
            }
            let right = self.data.get_unchecked(cur).right.0;
            let rv = self.data.get_unchecked(right).value.clone();
            self.data.get_unchecked_mut(right).value = self.monoid.op(value, rv);

            self.data.get_unchecked_mut(cur).value = self.monoid.id();
        }
    }

    fn update_(&mut self, cur: usize, from: usize, to: usize, s: usize, t: usize, value: &T) {
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
                return;
            } else if s <= from && to <= t {
                let cur_value = unsafe { self.data.get_unchecked(cur).value.clone() };
                unsafe {
                    self.data.get_unchecked_mut(cur).value =
                        self.monoid.op(value.clone(), cur_value);
                }
            } else {
                let mid = (from + to) / 2;
                unsafe {
                    self.propagate(cur, from, to);
                }
                let cur = unsafe { self.data.get_unchecked(cur) };
                let left = cur.left;
                let right = cur.right;
                self.update_(left.0, from, mid, s, t, value);
                self.update_(right.0, mid, to, s, t, value);
            }
        }
    }

    pub fn update(&mut self, Range { start: s, end: t }: Range<usize>, value: T) {
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

    fn get_(&mut self, cur: usize, from: usize, to: usize, i: usize) -> T {
        if !(from..to).contains(&i) {
            return self.monoid.id();
        }

        if to - from == 1 {
            unsafe { self.data.get_unchecked(cur).value.clone() }
        } else {
            unsafe {
                self.propagate(cur, from, to);
            }
            let mid = (from + to) / 2;
            let cur = unsafe { self.data.get_unchecked(cur) };
            if i < mid {
                self.get_(cur.left.0, from, mid, i)
            } else {
                self.get_(cur.right.0, mid, to, i)
            }
        }
    }

    pub fn get(&mut self, i: usize) -> T {
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
