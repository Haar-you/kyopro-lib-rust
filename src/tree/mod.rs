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

#[derive(Clone, Debug)]
pub struct TreeEdge<T> {
    pub to: usize,
    pub weight: T,
}

#[derive(Clone, Debug)]
pub struct TreeNode<T> {
    pub index: usize,
    pub parent: Option<TreeEdge<T>>,
    pub children: Vec<TreeEdge<T>>,
}

#[derive(Clone, Debug)]
pub struct Tree<T> {
    pub nodes: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn neighbors(&self) -> impl DoubleEndedIterator<Item = &TreeEdge<T>> {
        self.children.iter().chain(self.parent.iter())
    }

    pub fn neighbors_size(&self) -> usize {
        self.children.len() + self.parent.as_ref().map_or(0, |_| 1)
    }
}

impl<T: Copy> Tree<T> {
    pub fn new(size: usize) -> Self {
        Self {
            nodes: (0..size)
                .map(|i| TreeNode {
                    index: i,
                    parent: None,
                    children: vec![],
                })
                .collect(),
        }
    }

    pub fn add_undirected(&mut self, edges: impl IntoIterator<Item = (usize, usize, T)>) {
        for (u, v, w) in edges {
            self.nodes[u].children.push(TreeEdge { to: v, weight: w });
            self.nodes[v].children.push(TreeEdge { to: u, weight: w });
        }
    }

    pub fn add_directed(&mut self, edges: impl IntoIterator<Item = (usize, usize, T)>) {
        for (p, c, w) in edges {
            assert!(self.nodes[c].parent.is_none());
            self.nodes[p].children.push(TreeEdge { to: c, weight: w });
            self.nodes[c].parent = Some(TreeEdge { to: p, weight: w });
        }
    }
}

impl<T> Tree<T> {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
