//! Auxiliary Tree
//!
//! # References
//! - <https://noshi91.github.io/algorithm-encyclopedia/auxiliary-tree>

use crate::{misc::is_none_or::IsNoneOr, tree::*};

/// Auxiliary Tree
///
/// 与えられた頂点集合とそのLCAを保って木を圧縮した木を生成する。
pub struct AuxiliaryTree {
    preorder: Vec<usize>,
}

impl AuxiliaryTree {
    /// `AuxiliaryTree`を生成する。
    pub fn new<E: TreeEdgeTrait>(tree: &Tree<E>, root: usize) -> Self {
        let n = tree.len();
        let mut this = Self {
            preorder: vec![0; n],
        };
        this.dfs(tree, root, None, &mut 0);
        this
    }

    fn dfs<E: TreeEdgeTrait>(
        &mut self,
        tree: &Tree<E>,
        cur: usize,
        par: Option<usize>,
        i: &mut usize,
    ) {
        self.preorder[cur] = *i;
        *i += 1;

        for e in tree.nodes[cur].neighbors() {
            if par.is_none_or(|p| p != e.to()) {
                self.dfs(tree, e.to(), Some(cur), i);
            }
        }
    }

    /// 頂点集合`vs`からAuxiliaryTreeを構築する。
    pub fn build<F>(&self, mut vs: Vec<usize>, lca: F) -> Vec<usize>
    where
        F: Fn(usize, usize) -> usize,
    {
        vs.sort_by(|&a, &b| self.preorder[a].cmp(&self.preorder[b]));

        let n = vs.len();
        for i in 0..n - 1 {
            let x = lca(vs[i], vs[i + 1]);
            vs.push(x);
        }

        vs.sort_by(|&a, &b| self.preorder[a].cmp(&self.preorder[b]));
        vs.dedup();
        vs
    }
}
