//! 木に関する構造体やトレイトを提供する。

/// 木の辺
#[derive(Clone, Debug)]
pub struct TreeEdge<W, I> {
    pub(crate) from: usize,
    pub(crate) to: usize,
    pub(crate) weight: W,
    pub(crate) index: usize,
    /// 補助的な情報
    pub option: I,
}

impl<W, I> TreeEdge<W, I> {
    /// `from`から`to`への重さ`weight`、辺番号`index`をもつ有向辺を作る。
    pub fn new(from: usize, to: usize, weight: W, index: usize, option: I) -> Self {
        Self {
            from,
            to,
            weight,
            index,
            option,
        }
    }
}

impl<W, I> TreeEdge<W, I> {
    /// 辺の始点を返す。
    #[inline]
    pub fn from(&self) -> usize {
        self.from
    }
    /// 辺の終点を返す。
    #[inline]
    pub fn to(&self) -> usize {
        self.to
    }
    /// 辺の番号を返す。
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }
    /// 辺の逆辺を作って返す。
    pub fn rev(mut self) -> Self {
        std::mem::swap(&mut self.from, &mut self.to);
        self
    }
}

impl<W: Copy, I> TreeEdge<W, I> {
    /// 辺の重みを返す。
    #[inline]
    pub fn weight(&self) -> W {
        self.weight
    }
}

/// 木のノード
#[derive(Clone, Debug, Default)]
pub struct TreeNode<W, I> {
    pub(crate) parent: Option<TreeEdge<W, I>>,
    pub(crate) children: Vec<TreeEdge<W, I>>,
}

impl<W, I> TreeNode<W, I> {
    /// 親ノードへの辺を返す。
    pub fn parent(&self) -> Option<&TreeEdge<W, I>> {
        self.parent.as_ref()
    }
    /// 子ノードへの辺へのイテレータを返す。
    pub fn children(&self) -> impl DoubleEndedIterator<Item = &TreeEdge<W, I>> {
        self.children.iter()
    }

    /// 隣接辺を列挙するイテレータを返す。
    pub fn neighbors(&self) -> impl DoubleEndedIterator<Item = &TreeEdge<W, I>> {
        self.children.iter().chain(self.parent.iter())
    }

    /// 隣接辺の個数を返す。
    pub fn neighbors_size(&self) -> usize {
        self.children.len() + self.parent.as_ref().map_or(0, |_| 1)
    }
}

/// 非根付き木を構築する
pub struct TreeBuilder<W, I> {
    nodes: Vec<TreeNode<W, I>>,
    edge_num: usize,
}

impl<W: Copy, I: Clone> TreeBuilder<W, I> {
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
            edge_num: 0,
        }
    }

    /// [`Tree`]を作る。
    pub fn build(self) -> Tree<W, I> {
        Tree {
            nodes: self.nodes,
            root: None,
        }
    }

    /// 木に辺を追加する。
    pub fn add(&mut self, u: usize, v: usize, weight: W, option: I) {
        self.nodes[u]
            .children
            .push(TreeEdge::new(u, v, weight, self.edge_num, option.clone()));
        self.nodes[v]
            .children
            .push(TreeEdge::new(v, u, weight, self.edge_num, option));
        self.edge_num += 1;
    }
}

impl<W: Copy, I: Clone> Extend<(usize, usize, W, I)> for TreeBuilder<W, I> {
    fn extend<T: IntoIterator<Item = (usize, usize, W, I)>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|(u, v, weight, option)| self.add(u, v, weight, option));
    }
}

/// 根付き木を構築する
pub struct RootedTreeBuilder<W, I> {
    nodes: Vec<TreeNode<W, I>>,
    root: usize,
    edge_num: usize,
}

impl<W: Copy, I: Clone> RootedTreeBuilder<W, I> {
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
            edge_num: 0,
        }
    }

    /// 根付きの[`Tree`]を作る。
    pub fn build(self) -> Tree<W, I> {
        Tree {
            nodes: self.nodes,
            root: Some(self.root),
        }
    }

    /// 木に辺を追加する。
    pub fn add(&mut self, from: usize, to: usize, weight: W, option: I) {
        assert!(self.nodes[to].parent.is_none());
        self.nodes[from].children.push(TreeEdge::new(
            from,
            to,
            weight,
            self.edge_num,
            option.clone(),
        ));
        self.nodes[to]
            .parent
            .replace(TreeEdge::new(to, from, weight, self.edge_num, option));
        self.edge_num += 1;
    }
}

impl<W: Copy, I: Clone> Extend<(usize, usize, W, I)> for RootedTreeBuilder<W, I> {
    fn extend<T: IntoIterator<Item = (usize, usize, W, I)>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|(from, to, weight, option)| self.add(from, to, weight, option));
    }
}

/// 木
#[derive(Clone, Debug)]
pub struct Tree<W, I> {
    pub(crate) nodes: Vec<TreeNode<W, I>>,
    pub(crate) root: Option<usize>,
}

impl<W, I> Tree<W, I> {
    /// 各頂点の[`TreeNode`]への参照のイテレータを返す。
    pub fn nodes_iter(&self) -> impl Iterator<Item = &TreeNode<W, I>> {
        self.nodes.iter()
    }

    /// `i`番目の頂点の[`TreeNode`]への参照を返す。
    pub fn node_of(&self, i: usize) -> &TreeNode<W, I> {
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
