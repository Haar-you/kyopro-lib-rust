//! 全方位木DP

use crate::tree::*;

/// 全方位木DP
///
/// # References
/// - <https://null-mn.hatenablog.com/entry/2020/04/14/124151>
///
/// # Problems
/// - [EDPC V - Subtree](https://atcoder.jp/contests/dp/submissions/57560435)
/// - <https://atcoder.jp/contests/abc160/tasks/abc160_f>
/// - <https://judge.yosupo.jp/problem/tree_path_composite_sum>
pub struct RerootingDP<'a, T, U, E> {
    init: U,
    up: Box<dyn 'a + Fn(T, &'a E) -> U>,
    merge: Box<dyn 'a + Fn(U, U) -> U>,
    apply: Box<dyn 'a + Fn(U, usize) -> T>,
}

impl<'a, T, U, E> RerootingDP<'a, T, U, E>
where
    E: TreeEdgeTrait,
    T: Clone,
    U: Clone,
{
    /// `RerootingDP`を構築する。
    pub fn new(
        init: U,
        up: Box<impl 'a + Fn(T, &'a E) -> U>,
        merge: Box<impl 'a + Fn(U, U) -> U>,
        apply: Box<impl 'a + Fn(U, usize) -> T>,
    ) -> Self {
        Self {
            init,
            up,
            merge,
            apply,
        }
    }

    /// `tree`上で、全方位DPを実行する。
    pub fn run(&self, tree: &'a Tree<E>) -> Vec<T> {
        let size = tree.len();
        let mut dp = (0..size)
            .map(|i| vec![None; tree.nodes[i].neighbors_size()])
            .collect::<Vec<_>>();

        self.rec1(tree, &mut dp, 0, None);
        self.rec2(tree, &mut dp, 0, None, None);

        tree.nodes
            .iter()
            .enumerate()
            .map(|(i, nodes)| {
                let acc = nodes
                    .neighbors()
                    .enumerate()
                    .filter_map(|(j, e)| dp[i][j].as_ref().map(|res| (self.up)(res.clone(), e)))
                    .fold(self.init.clone(), |x, y| (self.merge)(x, y));
                (self.apply)(acc, i)
            })
            .collect()
    }

    fn rec1(
        &self,
        tree: &'a Tree<E>,
        dp: &mut Vec<Vec<Option<T>>>,
        cur: usize,
        par: Option<usize>,
    ) -> T {
        let acc = tree.nodes[cur]
            .neighbors()
            .enumerate()
            .filter(|(_, e)| par.is_none_or(|u| u != e.to()))
            .map(|(i, e)| {
                let res = self.rec1(tree, dp, e.to(), Some(cur));
                dp[cur][i] = Some(res.clone());
                (self.up)(res, e)
            })
            .fold(self.init.clone(), |x, y| (self.merge)(x, y));

        (self.apply)(acc, cur)
    }

    fn rec2(
        &self,
        tree: &'a Tree<E>,
        dp: &mut Vec<Vec<Option<T>>>,
        cur: usize,
        par: Option<usize>,
        value: Option<T>,
    ) {
        let len = tree.nodes[cur].neighbors_size();

        for (i, e) in tree.nodes[cur].neighbors().enumerate() {
            if par.is_some_and(|u| u == e.to()) {
                dp[cur][i] = value.clone();
            }
        }

        let mut left = vec![self.init.clone(); len + 1];
        let mut right = vec![self.init.clone(); len + 1];

        if len > 1 {
            for (i, e) in tree.nodes[cur].neighbors().take(len - 1).enumerate() {
                left[i + 1] = if let Some(res) = dp[cur][i].clone() {
                    (self.merge)(left[i].clone(), (self.up)(res, e))
                } else {
                    left[i].clone()
                };
            }

            for (i, e) in tree.nodes[cur].neighbors().rev().take(len - 1).enumerate() {
                let i = len - i - 1;
                right[i - 1] = if let Some(res) = dp[cur][i].clone() {
                    (self.merge)(right[i].clone(), (self.up)(res, e))
                } else {
                    right[i].clone()
                };
            }
        }

        for (i, e) in tree.nodes[cur].neighbors().enumerate() {
            if par.is_some_and(|u| u == e.to()) {
                continue;
            }

            self.rec2(
                tree,
                dp,
                e.to(),
                Some(cur),
                Some((self.apply)(
                    (self.merge)(left[i].clone(), right[i].clone()),
                    cur,
                )),
            );
        }
    }
}
