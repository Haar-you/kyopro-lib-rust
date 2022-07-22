//! 全方位木DP

use crate::tree::*;

/// 全方位木DP
///
/// # References
/// - [https://null-mn.hatenablog.com/entry/2020/04/14/124151](https://null-mn.hatenablog.com/entry/2020/04/14/124151x)
///
/// # Verification
/// - [EDPC V - Subtree #26944740](https://atcoder.jp/contests/dp/submissions/26944740)

pub struct RerootingDP<'a, Weight, T> {
    id: T,
    merge: Box<dyn 'a + Fn(T, T) -> T>,
    up: Box<dyn 'a + Fn(T, (usize, Weight)) -> T>,
    apply: Box<dyn 'a + Fn(T, usize) -> T>,
}

impl<'a, Weight, T> RerootingDP<'a, Weight, T>
where
    Weight: Copy,
    T: Clone,
{
    pub fn new(
        id: T,
        merge: Box<impl 'a + Fn(T, T) -> T>,
        up: Box<impl 'a + Fn(T, (usize, Weight)) -> T>,
        apply: Box<impl 'a + Fn(T, usize) -> T>,
    ) -> Self {
        Self {
            id,
            merge,
            up,
            apply,
        }
    }

    pub fn run(&self, tree: &Tree<Weight>) -> Vec<T> {
        let size = tree.len();

        let mut dp = vec![vec![]; size];
        let mut ret = vec![self.id.clone(); size];

        for (i, x) in dp.iter_mut().enumerate() {
            *x = vec![self.id.clone(); tree.nodes[i].neighbors_size()];
        }
        self.rec1(tree, &mut dp, 0, None);
        self.rec2(tree, &mut dp, 0, None, self.id.clone());
        for i in 0..size {
            for (j, e) in tree.nodes[i].neighbors().enumerate() {
                ret[i] = (self.merge)(
                    ret[i].clone(),
                    (self.up)(dp[i][j].clone(), (e.to, e.weight)),
                );
            }

            ret[i] = (self.apply)(ret[i].clone(), i);
        }

        ret
    }

    fn rec1(&self, tree: &Tree<Weight>, dp: &mut Vec<Vec<T>>, cur: usize, par: Option<usize>) -> T {
        let mut acc = self.id.clone();

        for (i, e) in tree.nodes[cur].neighbors().enumerate() {
            if par.map_or(false, |u| u == e.to) {
                continue;
            }

            dp[cur][i] = self.rec1(tree, dp, e.to, Some(cur));
            acc = (self.merge)(acc.clone(), (self.up)(dp[cur][i].clone(), (e.to, e.weight)));
        }

        (self.apply)(acc, cur)
    }

    fn rec2(
        &self,
        tree: &Tree<Weight>,
        dp: &mut Vec<Vec<T>>,
        cur: usize,
        par: Option<usize>,
        value: T,
    ) {
        let len = tree.nodes[cur].neighbors_size();

        for (i, e) in tree.nodes[cur].neighbors().enumerate() {
            if par.map_or(false, |u| u == e.to) {
                dp[cur][i] = value.clone();
            }
        }

        let mut left = vec![self.id.clone(); len + 1];
        let mut right = vec![self.id.clone(); len + 1];

        if len > 1 {
            for (i, e) in tree.nodes[cur].neighbors().take(len - 1).enumerate() {
                left[i + 1] = (self.merge)(
                    left[i].clone(),
                    (self.up)(dp[cur][i].clone(), (e.to, e.weight)),
                );
            }

            for (i, e) in tree.nodes[cur].neighbors().rev().take(len - 1).enumerate() {
                let i = len - i - 1;
                right[i - 1] = (self.merge)(
                    right[i].clone(),
                    (self.up)(dp[cur][i].clone(), (e.to, e.weight)),
                );
            }
        }

        for (i, e) in tree.nodes[cur].neighbors().enumerate() {
            if par.map_or(false, |u| u == e.to) {
                continue;
            }

            self.rec2(
                tree,
                dp,
                e.to,
                Some(cur),
                (self.apply)((self.merge)(left[i].clone(), right[i].clone()), cur),
            );
        }
    }
}
