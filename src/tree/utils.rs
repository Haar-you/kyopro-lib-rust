use crate::tree::*;
use std::ops::Add;

/// rootを根としたときの根から各頂点への距離を列挙する。
/// # Complexity
/// Time complexity $O(n)$
pub fn tree_distance<T, E: TreeEdgeTrait<Weight = T>>(tr: &Tree<E>, root: usize) -> Vec<T>
where
    T: Add<Output = T> + Copy + Default,
{
    let n = tr.len();
    let mut ret = vec![T::default(); n];
    let mut check = vec![false; n];
    let mut stack = vec![root];

    while let Some(cur) = stack.pop() {
        check[cur] = true;

        for e in tr.nodes[cur].neighbors() {
            if !check[e.to()] {
                ret[e.to()] = ret[cur] + e.weight();
                stack.push(e.to());
            }
        }
    }

    ret
}

/// 木の任意の2頂点の距離の最大値を求める。
/// # Complexity
/// Time complexity $O(n)$
pub fn tree_diameter<T, E: TreeEdgeTrait<Weight = T>>(tr: &Tree<E>) -> (T, usize, usize)
where
    T: Add<Output = T> + Copy + Default + Ord,
{
    let a = tree_distance(tr, 0);
    let (u, _) = a
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();

    let b = tree_distance(tr, u);
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
pub fn tree_height<T, E: TreeEdgeTrait<Weight = T>>(tr: &Tree<E>) -> Vec<(T, usize)>
where
    T: Add<Output = T> + Copy + Default + Ord,
{
    let d = tree_distance(tr, 0);
    let (u, _) = d
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();
    let d1 = tree_distance(tr, u);
    let (v, _) = d1
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();
    let d2 = tree_distance(tr, v);

    d1.into_iter()
        .zip(d2.into_iter())
        .map(|(x, y)| if x > y { (x, u) } else { (y, v) })
        .collect()
}

/// 木上の2頂点を結ぶパス上の頂点列を求める。
/// # Complexity
/// Time complexity $O(n)$
pub fn tree_path<T, E: TreeEdgeTrait<Weight = T>>(tr: &Tree<E>, u: usize, v: usize) -> Vec<usize> {
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

            for e in tr.nodes[i].neighbors() {
                if !check[e.to()] {
                    stack.push((e.to(), 0));
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
    fn test_diameter() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_5_A
        let mut builder = TreeBuilder::new(4);
        builder.extend(
            vec![(0, 1, 2), (1, 2, 1), (1, 3, 3)]
                .into_iter()
                .map(|(u, v, w)| TreeEdge::new(u, v, w, ())),
        );
        let tree = builder.build();
        assert_eq!(tree_diameter(&tree).0, 5);

        let mut builder = TreeBuilder::new(4);
        builder.extend(
            vec![(0, 1, 1), (1, 2, 2), (2, 3, 4)]
                .into_iter()
                .map(|(u, v, w)| TreeEdge::new(u, v, w, ())),
        );
        let tree = builder.build();
        assert_eq!(tree_diameter(&tree).0, 7);
    }

    #[test]
    fn test_height() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_5_B
        let mut builder = TreeBuilder::new(4);
        builder.extend(
            vec![(0, 1, 2), (1, 2, 1), (1, 3, 3)]
                .into_iter()
                .map(|(u, v, w)| TreeEdge::new(u, v, w, ())),
        );
        let tree = builder.build();
        assert_eq!(
            tree_height(&tree)
                .into_iter()
                .map(|(x, _)| x)
                .collect::<Vec<_>>(),
            [5, 3, 4, 5]
        );
    }
}
