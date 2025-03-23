//! 木に関するもの

pub mod auxiliary_tree;
pub mod centroid;
pub mod centroid_decomposition;
pub mod depth_query;
pub mod euler_tour;
pub mod hld;
pub mod lca;
pub mod rerooting;
pub mod rooted_isomorphism;
pub mod rooting;
pub mod tree_dp;
pub mod utils;

pub trait TreeEdgeTrait {
    /// 辺の重みの型
    type Weight;
    /// 辺の始点を返す。
    fn from(&self) -> usize;
    /// 辺の終点を返す。
    fn to(&self) -> usize;
    /// 辺の重みを返す。
    fn weight(&self) -> Self::Weight;
    /// 逆辺を返す。
    fn rev(self) -> Self;
}

/// 始点、終点、重み、番号をもつ木の辺
#[derive(Clone, Debug)]
pub struct TreeEdge<T, I> {
    /// 始点
    pub from: usize,
    /// 終点
    pub to: usize,
    /// 重み
    pub weight: T,
    /// 辺の番号
    pub index: I,
}

impl<T, I> TreeEdge<T, I> {
    pub fn new(from: usize, to: usize, weight: T, index: I) -> Self {
        Self {
            from,
            to,
            weight,
            index,
        }
    }
}

impl<T: Clone, I> TreeEdgeTrait for TreeEdge<T, I> {
    type Weight = T;
    #[inline]
    fn from(&self) -> usize {
        self.from
    }
    #[inline]
    fn to(&self) -> usize {
        self.to
    }
    #[inline]
    fn weight(&self) -> Self::Weight {
        self.weight.clone()
    }
    fn rev(mut self) -> Self {
        std::mem::swap(&mut self.from, &mut self.to);
        self
    }
}

/// 木のノード
#[derive(Clone, Debug, Default)]
pub struct TreeNode<E> {
    /// 親ノードへの辺
    pub parent: Option<E>,
    /// 子ノードへの辺
    pub children: Vec<E>,
}

impl<E: TreeEdgeTrait> TreeNode<E> {
    /// 隣接辺を列挙するイテレータを返す。
    pub fn neighbors(&self) -> impl DoubleEndedIterator<Item = &E> {
        self.children.iter().chain(self.parent.iter())
    }

    /// 隣接辺の個数を返す。
    pub fn neighbors_size(&self) -> usize {
        self.children.len() + self.parent.as_ref().map_or(0, |_| 1)
    }
}

/// 非根付き木を構築する
pub struct TreeBuilder<E> {
    nodes: Vec<TreeNode<E>>,
}

impl<E: TreeEdgeTrait + Clone> TreeBuilder<E> {
    /// 頂点数`size`の[`TreeBuilder`]を生成する。
    pub fn new(size: usize) -> Self {
        Self {
            nodes: vec![
                TreeNode {
                    parent: None,
                    children: vec![],
                };
                size
            ],
        }
    }

    pub fn build(self) -> Tree<E> {
        Tree {
            nodes: self.nodes,
            root: None,
        }
    }
}

impl<E: TreeEdgeTrait + Clone> Extend<E> for TreeBuilder<E> {
    fn extend<T: IntoIterator<Item = E>>(&mut self, iter: T) {
        for e in iter {
            self.nodes[e.from()].children.push(e.clone());
            self.nodes[e.to()].children.push(e.rev());
        }
    }
}

/// 根付き木を構築する
pub struct RootedTreeBuilder<E> {
    nodes: Vec<TreeNode<E>>,
    root: usize,
}

impl<E: TreeEdgeTrait + Clone> RootedTreeBuilder<E> {
    /// 頂点数`size`の[`TreeBuilder`]を生成する。
    pub fn new(size: usize, root: usize) -> Self {
        Self {
            nodes: vec![
                TreeNode {
                    parent: None,
                    children: vec![],
                };
                size
            ],
            root,
        }
    }

    pub fn build(self) -> Tree<E> {
        Tree {
            nodes: self.nodes,
            root: Some(self.root),
        }
    }
}

impl<E: TreeEdgeTrait + Clone> Extend<E> for RootedTreeBuilder<E> {
    fn extend<T: IntoIterator<Item = E>>(&mut self, iter: T) {
        for e in iter {
            assert!(self.nodes[e.to()].parent.is_none());
            self.nodes[e.from()].children.push(e.clone());
            self.nodes[e.to()].parent.replace(e.rev());
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tree<E> {
    nodes: Vec<TreeNode<E>>,
    root: Option<usize>,
}

impl<E> Tree<E> {
    /// 各頂点の[`TreeNode`]への参照のイテレータを返す。
    pub fn nodes_iter(&self) -> impl Iterator<Item = &TreeNode<E>> {
        self.nodes.iter()
    }

    /// `i`番目の頂点の[`TreeNode`]への参照を返す。
    pub fn nodes(&self, i: usize) -> &TreeNode<E> {
        &self.nodes[i]
    }

    /// 木の頂点数を返す。
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// 木の頂点数が`0`ならば`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// 木に根があれば根を返す。
    pub fn root(&self) -> Option<usize> {
        self.root
    }
}
