#![allow(clippy::many_single_char_names)]

use crate::tree::*;
use std::{cmp::max, ops::Add};

/// rootを根としたときの根から各頂点への距離を列挙する。
/// # Complexity
/// Time complexity $O(n)$
pub fn tree_distance<T>(tr: &Tree<T>, root: usize) -> Vec<T>
where
    T: Add<Output = T> + Copy + Default,
{
    let n = tr.len();
    let mut ret = vec![T::default(); n];
    let mut check = vec![false; n];
    let mut stack = vec![root];

    while let Some(cur) = stack.pop() {
        check[cur] = true;

        for &TreeEdge { to, weight } in tr.nodes[cur].neighbors() {
            if !check[to] {
                ret[to] = ret[cur] + weight;
                stack.push(to);
            }
        }
    }

    ret
}

/// 木の任意の2頂点の距離の最大値を求める。
/// # Complexity
/// Time complexity $O(n)$
pub fn tree_diameter<T>(tr: &Tree<T>) -> (T, usize, usize)
where
    T: Add<Output = T> + Copy + Default + Ord,
{
    let a = tree_distance(&tr, 0);
    let (u, _) = a
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();

    let b = tree_distance(&tr, u);
    let (v, &d) = b
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();

    (d, u, v)
}

/// 木の各頂点について、そこからの距離の最大値を列挙する。
/// # Complexity
/// Time complexity $O(n)$
pub fn tree_height<T>(tr: &Tree<T>) -> Vec<T>
where
    T: Add<Output = T> + Copy + Default + Ord,
{
    let d = tree_distance(&tr, 0);
    let (u, _) = d
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();
    let d1 = tree_distance(&tr, u);
    let (v, _) = d1
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();
    let d2 = tree_distance(&tr, v);

    d1.into_iter()
        .zip(d2.into_iter())
        .map(|(x, y)| max(x, y))
        .collect()
}

/// 木上の2頂点を結ぶパス上の頂点列を求める。
/// # Complexity
/// Time complexity $O(n)$
pub fn tree_path<T>(tr: &Tree<T>, u: usize, v: usize) -> Vec<usize> {
    let n = tr.len();
    let mut ret = vec![];
    let mut stack = vec![];
    let mut check = vec![false; n];

    stack.push((u, 0));

    while let Some((i, st)) = stack.pop() {
        if st == 1 {
            ret.pop();
        } else {
            stack.push((i, 1));
            ret.push(i);

            if i == v {
                break;
            }

            check[i] = true;

            for &TreeEdge { to, .. } in tr.nodes[i].neighbors() {
                if !check[to] {
                    stack.push((to, 0));
                }
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_5_A
        let mut tree = Tree::new(4);
        tree.add_undirected(vec![(0, 1, 2), (1, 2, 1), (1, 3, 3)]);
        assert_eq!(tree_diameter(&tree).0, 5);

        let mut tree = Tree::new(4);
        tree.add_undirected(vec![(0, 1, 1), (1, 2, 2), (2, 3, 4)]);
        assert_eq!(tree_diameter(&tree).0, 7);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_5_B
        let mut tree = Tree::new(4);
        tree.add_undirected(vec![(0, 1, 2), (1, 2, 1), (1, 3, 3)]);
        assert_eq!(tree_height(&tree), [5, 3, 4, 5]);
    }
}
