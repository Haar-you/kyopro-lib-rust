//! 木DP

#[allow(unused_imports)]
use crate::misc::is_none_or::IsNoneOr;
use crate::tree::*;

/// 木DP
///
/// # Problems
/// - <https://atcoder.jp/contests/dp/tasks/dp_p>
/// - <https://yukicoder.me/problems/no/763>
pub struct TreeDP<'a, Weight, T, U> {
    init: U,
    up: Box<dyn 'a + Fn(T, (usize, Weight)) -> U>,
    merge: Box<dyn 'a + Fn(U, U) -> U>,
    apply: Box<dyn 'a + Fn(U, usize) -> T>,
}

impl<'a, Weight, T, U> TreeDP<'a, Weight, T, U>
where
    Weight: Copy,
    T: Clone,
    U: Clone,
{
    /// `TreeDP`を構築する。
    pub fn new(
        init: U,
        up: Box<impl 'a + Fn(T, (usize, Weight)) -> U>,
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

    /// `root`を根にして、`tree`上でDPを実行する。
    ///
    /// **Time complexity** $O(n)$
    pub fn run<E: TreeEdgeTrait<Weight = Weight>>(&self, tree: &Tree<E>, root: usize) -> Vec<T> {
        let size = tree.len();
        let mut ret = vec![None; size];

        self.__dfs(tree, root, None, &mut ret);

        ret.into_iter().flatten().collect()
    }

    fn __dfs<E: TreeEdgeTrait<Weight = Weight>>(
        &self,
        tree: &Tree<E>,
        cur: usize,
        par: Option<usize>,
        ret: &mut Vec<Option<T>>,
    ) -> T {
        let acc = tree.nodes[cur]
            .neighbors()
            .filter(|e| par.is_none_or(|p| p != e.to()))
            .map(|e| {
                let a = self.__dfs(tree, e.to(), Some(cur), ret);
                (self.up)(a, (e.to(), e.weight()))
            })
            .fold(self.init.clone(), |a, b| (self.merge)(a, b));
        ret[cur] = Some((self.apply)(acc, cur));
        ret[cur].clone().unwrap()
    }
}
