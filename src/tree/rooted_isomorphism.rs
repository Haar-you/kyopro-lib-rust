//! 根付き木の(根付き)部分木を同型性によって分類する。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/rooted_tree_isomorphism_classification>
#[allow(unused_imports)]
use crate::misc::is_none_or::IsNoneOr;
use crate::tree::*;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

/// 根付き木の(根付き)部分木を同型性によって分類する。
///
/// # Returns
/// - `(k, a)` : `k`は部分木の種類数、`a`は頂点`i`と頂点`j`を根とする部分木が同型のときに限り`a[i] = a[j]`を満たす。
pub fn rooted_isomorphism<E: TreeEdgeTrait>(tree: &Tree<E>, root: usize) -> (usize, Vec<usize>) {
    let n = tree.len();
    let mut ret = vec![0; n];
    let mut map = HashMap::new();
    let mut stack = Vec::with_capacity(2 * n);

    stack.push((false, root, None));

    while let Some((back, cur, par)) = stack.pop() {
        if back {
            let mut children = vec![];

            for e in tree.nodes[cur].neighbors() {
                if par.is_none_or(|p| p != e.to()) {
                    children.push(ret[e.to()]);
                }
            }

            children.sort_unstable();

            let mut hasher = DefaultHasher::new();
            children.hash(&mut hasher);
            let h = hasher.finish();

            let k = map.len();
            ret[cur] = *map.entry(h).or_insert(k);
        } else {
            stack.push((true, cur, par));
            for e in tree.nodes[cur].neighbors() {
                if par.is_none_or(|p| p != e.to()) {
                    stack.push((false, e.to(), Some(cur)));
                }
            }
        }
    }

    (map.len(), ret)
}
