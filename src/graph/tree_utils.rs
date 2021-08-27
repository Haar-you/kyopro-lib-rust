use crate::graph::template::*;
use std::ops::Add;

impl<T> Graph<T>
where
    T: Add<Output = T> + Copy + Default + Ord,
{
    /// rootを根としたときの根から各頂点への距離を列挙する。
    ///
    /// Time complexity O(n)
    pub fn tree_distance(&self, root: usize) -> Vec<T> {
        let n = self.len();
        let mut ret = vec![T::default(); n];
        let mut check = vec![false; n];

        let mut stack = vec![root];
        ret[root] = T::default();

        while let Some(cur) = stack.pop() {
            check[cur] = true;

            for &Edge { to, cost, .. } in &self.edges[cur] {
                if !check[to] {
                    ret[to] = ret[cur] + cost;
                    stack.push(to);
                }
            }
        }

        ret
    }

    /// 木の任意の2頂点の距離の最大値を求める。
    ///
    /// Time complexity O(n)
    pub fn tree_diameter(&self) -> (T, usize, usize) {
        let a = self.tree_distance(0);
        let (u, _) = a
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();

        let b = self.tree_distance(u);
        let (v, &d) = b
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();

        (d, u, v)
    }

    /// 木の各頂点について、そこからの距離の最大値を列挙する。
    ///
    /// Time complexity O(n)
    pub fn tree_height(&self) -> Vec<T> {
        let d = self.tree_distance(0);
        let (u, _) = d
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();
        let d1 = self.tree_distance(u);
        let (v, _) = d1
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();
        let d2 = self.tree_distance(v);

        d1.into_iter()
            .zip(d2.into_iter())
            .map(|(x, y)| std::cmp::max(x, y))
            .collect()
    }

    /// 木上の2頂点を結ぶパス上の頂点列を求める。
    ///
    /// Time complexity O(n)
    pub fn tree_path(&self, u: usize, v: usize) -> Vec<usize> {
        let n = self.len();
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

                for &Edge { to, .. } in &self.edges[i] {
                    if !check[to] {
                        stack.push((to, 0));
                    }
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_5_A
        let g = Graph::<u32>::from_tuples_undirected(4, &[(0, 1, 2), (1, 2, 1), (1, 3, 3)]);
        assert_eq!(g.tree_diameter().0, 5);

        let g = Graph::<u32>::from_tuples_undirected(4, &[(0, 1, 1), (1, 2, 2), (2, 3, 4)]);
        assert_eq!(g.tree_diameter().0, 7);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_5_B
        let g = Graph::<u32>::from_tuples_undirected(4, &[(0, 1, 2), (1, 2, 1), (1, 3, 3)]);
        assert_eq!(g.tree_height(), [5, 3, 4, 5]);
    }
}
