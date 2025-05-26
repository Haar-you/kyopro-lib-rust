//! 動的セグメント木
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/point_set_range_composite_large_array>
use crate::algebra::traits::Monoid;
use crate::misc::nullable_usize::NullableUsize;
use std::ops::Range;

#[derive(Debug)]
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

/// 動的セグメント木
#[derive(Debug)]
pub struct DynamicSegtree<M: Monoid> {
    data: Vec<Node<M>>,
    root: NullableUsize,
    to: usize,
}

impl<M: Monoid + Clone> Default for DynamicSegtree<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: Monoid + Clone> DynamicSegtree<M> {
    /// [`DynamicSegtree<M>`]を生成する。
    pub fn new() -> Self {
        Self {
            data: vec![Node::new(M::id())],
            root: NullableUsize(0),
            to: 1,
        }
    }

    fn assign_dfs(
        &mut self,
        cur_id: NullableUsize,
        cur_from: usize,
        cur_to: usize,
        i: usize,
        value: M,
    ) {
        if cur_to - cur_from == 1 {
            self.data[cur_id.0].value = value;
        } else {
            let mid = (cur_from + cur_to) / 2;
            if (cur_from..mid).contains(&i) {
                if self.data[cur_id.0].left.is_null() {
                    let new_node = Node::new(value.clone());
                    self.data.push(new_node);
                    self.data[cur_id.0].left = NullableUsize(self.data.len() - 1);
                }
                self.assign_dfs(self.data[cur_id.0].left, cur_from, mid, i, value);
            } else {
                if self.data[cur_id.0].right.is_null() {
                    let new_node = Node::new(value.clone());
                    self.data.push(new_node);
                    self.data[cur_id.0].right = NullableUsize(self.data.len() - 1);
                }
                self.assign_dfs(self.data[cur_id.0].right, mid, cur_to, i, value);
            }

            let left = self.data[cur_id.0].left;
            let right = self.data[cur_id.0].right;

            self.data[cur_id.0].value = M::op(
                if left.is_null() {
                    M::id()
                } else {
                    self.data[left.0].value.clone()
                },
                if right.is_null() {
                    M::id()
                } else {
                    self.data[right.0].value.clone()
                },
            );
        }
    }

    /// `i`番目の要素を`value`で更新する。
    pub fn assign(&mut self, i: usize, value: M) {
        loop {
            if i < self.to {
                break;
            }

            self.to *= 2;
            let mut new_root = Node::new(self.data[self.root.0].value.clone());
            new_root.left = self.root;
            self.data.push(new_root);
            self.root = NullableUsize(self.data.len() - 1);
        }

        self.assign_dfs(self.root, 0, self.to, i, value);
    }

    fn fold_dfs(
        &self,
        cur_id: NullableUsize,
        cur_from: usize,
        cur_to: usize,
        from: usize,
        to: usize,
    ) -> M {
        let cur = &self.data[cur_id.0];

        if cur_to <= from || to <= cur_from {
            M::id()
        } else if from <= cur_from && cur_to <= to {
            cur.value.clone()
        } else {
            let mid = (cur_from + cur_to) / 2;
            let lv = if cur.left.is_null() {
                M::id()
            } else {
                self.fold_dfs(cur.left, cur_from, mid, from, to)
            };
            let rv = if cur.right.is_null() {
                M::id()
            } else {
                self.fold_dfs(cur.right, mid, cur_to, from, to)
            };

            M::op(lv, rv)
        }
    }

    /// 範囲`start..end`で計算を集約する。
    pub fn fold(&self, Range { start, end }: Range<usize>) -> M {
        self.fold_dfs(self.root, 0, self.to, start, end)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::algebra::sum::*;
    use my_testtools::rand_range;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let mut seg = DynamicSegtree::<Sum<u64>>::new();
        let mut map = BTreeMap::<usize, Sum<u64>>::new();

        let t = 100;

        for _ in 0..t {
            let i = rng.gen_range::<usize, _>(0..usize::MAX / 2);
            let x = rng.gen::<u64>() % 1000000;

            seg.assign(i, Sum(x));
            map.entry(i).or_insert(Sum::id()).op_assign_r(Sum(x));

            let lr = rand_range(&mut rng, 0..usize::MAX / 2);

            let res = seg.fold(lr.clone());
            let ans = map.range(lr).map(|(_, v)| v).cloned().fold_m();

            assert_eq!(res, ans);
        }
    }
}
