//! Cartesian tree
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/cartesian_tree>

/// CartesianTree
pub struct CartesianTree {
    /// Cartesian treeの根
    pub root: usize,
    /// 親の頂点
    pub parent: Vec<Option<usize>>,
    /// 右の子の頂点
    pub left: Vec<Option<usize>>,
    /// 左の子の頂点
    pub right: Vec<Option<usize>>,
}

impl CartesianTree {
    /// distinctな配列`a`から[`CartesianTree`]を構築する。
    pub fn new<T>(a: &[T]) -> Self
    where
        T: PartialOrd,
    {
        let n = a.len();
        let mut p = vec![None; n];
        let mut l = vec![None; n];
        let mut r = vec![None; n];
        let mut root = 0;

        for i in 1..n {
            let mut j = i - 1;

            loop {
                if a[i] < a[j] {
                    if let Some(p) = p[j] {
                        j = p;
                        continue;
                    } else {
                        p[j] = Some(i);
                        l[i] = Some(j);
                        root = i;
                        break;
                    }
                }

                let t = r[j];
                r[j] = Some(i);
                p[i] = Some(j);
                l[i] = t;

                if let Some(t) = t {
                    p[t] = Some(i);
                }

                break;
            }
        }

        CartesianTree {
            root,
            parent: p,
            left: l,
            right: r,
        }
    }
}
