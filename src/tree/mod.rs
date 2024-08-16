//! 木に関するもの

pub mod centroid;
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
    type Weight;
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    fn weight(&self) -> Self::Weight;
    fn rev(self) -> Self;
}

#[derive(Clone, Debug)]
pub struct TreeEdge<T, I> {
    pub from: usize,
    pub to: usize,
    pub weight: T,
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

#[derive(Clone, Debug, Default)]
pub struct TreeNode<E> {
    pub parent: Option<E>,
    pub children: Vec<E>,
}

impl<E: TreeEdgeTrait> TreeNode<E> {
    pub fn neighbors(&self) -> impl DoubleEndedIterator<Item = &E> {
        self.children.iter().chain(self.parent.iter())
    }

    pub fn neighbors_size(&self) -> usize {
        self.children.len() + self.parent.as_ref().map_or(0, |_| 1)
    }
}

pub struct TreeBuilder<E> {
    nodes: Vec<TreeNode<E>>,
}

impl<E: TreeEdgeTrait + Clone> TreeBuilder<E> {
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

    pub fn extend(&mut self, edges: impl IntoIterator<Item = E>) {
        for e in edges {
            self.nodes[e.from()].children.push(e.clone());
            self.nodes[e.to()].children.push(e.rev());
        }
    }

    pub fn build(self) -> Tree<E> {
        Tree {
            nodes: self.nodes,
            root: None,
        }
    }
}

pub struct RootedTreeBuilder<E> {
    nodes: Vec<TreeNode<E>>,
    root: usize,
}

impl<E: TreeEdgeTrait + Clone> RootedTreeBuilder<E> {
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

    pub fn extend(&mut self, edges: impl IntoIterator<Item = E>) {
        for e in edges {
            assert!(self.nodes[e.to()].parent.is_none());
            self.nodes[e.from()].children.push(e.clone());
            self.nodes[e.to()].parent.replace(e.rev());
        }
    }

    pub fn build(self) -> Tree<E> {
        Tree {
            nodes: self.nodes,
            root: Some(self.root),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tree<E> {
    nodes: Vec<TreeNode<E>>,
    root: Option<usize>,
}

impl<E> Tree<E> {
    pub fn nodes_iter(&self) -> impl Iterator<Item = &TreeNode<E>> {
        self.nodes.iter()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn root(&self) -> Option<usize> {
        self.root
    }
}
