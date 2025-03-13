//! 重心分解
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc291/tasks/abc291_h>
use crate::tree::*;

/// [`CentroidDecomposition`]の頂点ノード
#[derive(Clone)]
pub struct Node {
    /// 親の頂点
    pub par: Option<usize>,
    /// 子の頂点列
    pub children: Vec<usize>,
    /// 深さ
    pub depth: usize,
    /// 部分木の大きさ
    pub subsize: usize,
}

/// 重心分解
pub struct CentroidDecomposition {
    nodes: Vec<Node>,
}

impl CentroidDecomposition {
    /// 木`tree`を重心分解する。
    pub fn new<E: TreeEdgeTrait>(tree: &Tree<E>) -> Self {
        let n = tree.len();
        let mut this = Self {
            nodes: vec![
                Node {
                    par: None,
                    children: vec![],
                    depth: !0,
                    subsize: 0,
                };
                n
            ],
        };

        let mut subsize = vec![0; n];
        this.decompose(tree, 0, None, 0, &mut subsize);
        for (a, s) in this.nodes.iter_mut().zip(subsize) {
            a.subsize = s;
        }
        this
    }

    /// 重心分解後の頂点列への参照を返す。
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    fn decompose<E: TreeEdgeTrait>(
        &mut self,
        tree: &Tree<E>,
        cur: usize,
        par: Option<usize>,
        d: usize,
        subsize: &mut [usize],
    ) {
        self.dfs_subsize(tree, cur, None, subsize);
        let c = self.get_centroid(tree, cur, None, subsize[cur], subsize);

        self.nodes[c].par = par;
        self.nodes[c].depth = d;

        if let Some(par) = par {
            self.nodes[par].children.push(c);
        }

        for e in tree.nodes[c].neighbors() {
            if self.nodes[e.to()].depth != !0 {
                continue;
            }
            self.decompose(tree, e.to(), Some(c), d + 1, subsize);
        }
    }

    fn get_centroid<E: TreeEdgeTrait>(
        &self,
        tree: &Tree<E>,
        cur: usize,
        par: Option<usize>,
        total_size: usize,
        subsize: &[usize],
    ) -> usize {
        tree.nodes[cur]
            .neighbors()
            .filter(|e| par != Some(e.to()) && self.nodes[e.to()].depth == !0)
            .find(|e| 2 * subsize[e.to()] > total_size)
            .map_or(cur, |e| {
                self.get_centroid(tree, e.to(), Some(cur), total_size, subsize)
            })
    }

    fn dfs_subsize<E: TreeEdgeTrait>(
        &self,
        tree: &Tree<E>,
        cur: usize,
        par: Option<usize>,
        subsize: &mut [usize],
    ) {
        subsize[cur] = 1 + tree.nodes[cur]
            .neighbors()
            .filter(|e| par != Some(e.to()) && self.nodes[e.to()].depth == !0)
            .map(|e| {
                self.dfs_subsize(tree, e.to(), Some(cur), subsize);
                subsize[e.to()]
            })
            .sum::<usize>();
    }
}
