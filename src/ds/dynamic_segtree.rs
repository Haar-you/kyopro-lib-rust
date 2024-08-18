use crate::algebra::traits::Monoid;
use std::ops::Range;

#[derive(Debug)]
struct Node<T> {
    from: usize,
    to: usize,
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(from: usize, to: usize, value: T) -> Self {
        Self {
            from,
            to,
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct DynamicSegtree<M: Monoid> {
    root: Option<Box<Node<M::Output>>>,
    monoid: M,
}

impl<T: Clone, M: Monoid<Output = T> + Clone> DynamicSegtree<M> {
    pub fn new(monoid: M) -> Self {
        Self {
            root: Some(Box::new(Node::new(0, 1, monoid.id()))),
            monoid,
        }
    }

    fn assign_dfs(cur: &mut Node<T>, i: usize, value: T, m: &M) {
        if cur.to - cur.from == 1 {
            cur.value = value;
        } else {
            let mid = (cur.from + cur.to) / 2;
            if (cur.from..mid).contains(&i) {
                if cur.left.is_none() {
                    cur.left = Some(Box::new(Node::new(cur.from, mid, value.clone())));
                }
                Self::assign_dfs(cur.left.as_mut().unwrap().as_mut(), i, value, m);
            } else {
                if cur.right.is_none() {
                    cur.right = Some(Box::new(Node::new(mid, cur.to, value.clone())));
                }
                Self::assign_dfs(cur.right.as_mut().unwrap().as_mut(), i, value, m);
            }
            cur.value = m.op(
                cur.left
                    .as_ref()
                    .map_or(m.id(), |a| a.as_ref().value.clone()),
                cur.right
                    .as_ref()
                    .map_or(m.id(), |a| a.as_ref().value.clone()),
            );
        }
    }

    pub fn assign(&mut self, i: usize, value: T) {
        loop {
            let cur = self.root.take().unwrap();

            if (cur.from..cur.to).contains(&i) {
                self.root = Some(cur);
                break;
            }

            let mut new_root = Box::new(Node::new(cur.from, cur.to * 2, cur.value.clone()));
            new_root.left = Some(cur);
            self.root = Some(new_root);
        }

        Self::assign_dfs(
            self.root.as_mut().unwrap().as_mut(),
            i,
            value,
            &self.monoid.clone(),
        );
    }

    fn fold_dfs(cur: &Node<T>, from: usize, to: usize, m: &M) -> T {
        if cur.to <= from || to <= cur.from {
            m.id()
        } else if from <= cur.from && cur.to <= to {
            cur.value.clone()
        } else {
            let lv = cur
                .left
                .as_ref()
                .map_or(m.id(), |a| Self::fold_dfs(a.as_ref(), from, to, m));

            let rv = cur
                .right
                .as_ref()
                .map_or(m.id(), |a| Self::fold_dfs(a.as_ref(), from, to, m));

            m.op(lv, rv)
        }
    }

    pub fn fold(&self, Range { start, end }: Range<usize>) -> T {
        Self::fold_dfs(
            self.root.as_ref().unwrap().as_ref(),
            start,
            end,
            &self.monoid,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::algebra::sum::*;
    use crate::testtools::rand_range;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let m = Sum::<u64>::new();
        let mut seg = DynamicSegtree::new(m.clone());
        let mut map = BTreeMap::new();

        let t = 100;

        for _ in 0..t {
            let i = rng.gen_range::<usize, _>(0..usize::MAX / 2);
            let x = rng.gen::<u64>() % 1000000;

            seg.assign(i, x);
            *map.entry(i).or_insert(0) += x;

            let lr = rand_range(&mut rng, 0..usize::MAX / 2);

            let res = seg.fold(lr.clone());
            let ans = map
                .range(lr)
                .map(|(_, v)| v)
                .fold(m.id(), |x, y| m.op(x, *y));

            assert_eq!(res, ans);
        }
    }
}
