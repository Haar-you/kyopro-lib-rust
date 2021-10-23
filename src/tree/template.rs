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
    pub fn neighbors(&self) -> impl Iterator<Item = &TreeEdge<T>> {
        self.children.iter().chain(self.parent.iter())
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

    pub fn from_tuples(size: usize, es: &[(usize, usize, T)]) -> Self {
        let mut ret = Self::new(size);
        for &(u, v, w) in es {
            ret.add(u, v, w);
        }
        ret
    }

    pub fn add(&mut self, u: usize, v: usize, w: T) {
        self.nodes[u].children.push(TreeEdge { to: v, weight: w });
        self.nodes[v].children.push(TreeEdge { to: u, weight: w });
    }

    pub fn add_directed(&mut self, p: usize, c: usize, w: T) {
        assert!(self.nodes[c].parent.is_none());

        self.nodes[p].children.push(TreeEdge { to: c, weight: w });
        self.nodes[c].parent = Some(TreeEdge { to: p, weight: w });
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
