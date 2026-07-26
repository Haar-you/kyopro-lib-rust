//! グラフ一般に関するもの

pub mod articulation_points;
pub mod biconnected;
pub mod bridges;
pub mod lowlink;
pub mod scc;
pub mod two_edge;

pub mod functional_graph;
pub mod pseudo_tree;

pub mod bellman_ford;
pub mod bfs;
pub mod dijkstra;
pub mod warshall_floyd;
pub mod yen;

pub mod cycle;
pub mod eulerian;

pub mod bipartite;

pub mod enumerate_triangles;
pub mod max_independent_set;

pub mod directed_mst;
pub mod kruskal;
pub mod min_steiner_tree;
pub mod prim;

pub mod count_tsort;
pub mod tsort;

pub mod chinese_postman;
pub mod tsp;

pub mod chromatic_number;

pub mod count_eulerian_circuits;
pub mod matrix_tree;

use std::marker::PhantomData;

/// グラフの辺
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Edge<W, I> {
    from: usize,
    to: usize,
    weight: W,
    index: usize,
    /// 補助的な情報
    pub option: I,
}

impl<W, I> Edge<W, I> {
    fn new(from: usize, to: usize, weight: W, index: usize, option: I) -> Self {
        Self {
            from,
            to,
            weight,
            index,
            option,
        }
    }
}

impl<W, I> Edge<W, I> {
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

impl<W: Copy, I> Edge<W, I> {
    /// 辺の重みを返す。
    #[inline]
    pub fn weight(&self) -> W {
        self.weight
    }
}

/// グラフの辺の有向・無向の情報をもたせるためのトレイト。
pub trait Direction {}
/// 有向辺をもつことを示す。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Directed;
/// 無向辺をもつことを示す。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Undirected;
impl Direction for Directed {}
impl Direction for Undirected {}

/// グラフのノード
#[derive(Clone, Debug)]
pub struct GraphNode<W, I> {
    edges: Vec<Edge<W, I>>,
}

impl<W, I> GraphNode<W, I> {
    /// 隣接辺を列挙するイテレータを返す。
    pub fn neighbors(&self) -> impl DoubleEndedIterator<Item = &Edge<W, I>> {
        self.edges.iter()
    }

    /// 隣接辺の個数を返す。
    pub fn neighbors_size(&self) -> usize {
        self.edges.len()
    }
}

impl<W, I> IntoIterator for GraphNode<W, I> {
    type Item = Edge<W, I>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.edges.into_iter()
    }
}

impl<'a, W, I> IntoIterator for &'a GraphNode<W, I> {
    type Item = &'a Edge<W, I>;
    type IntoIter = std::slice::Iter<'a, Edge<W, I>>;

    fn into_iter(self) -> Self::IntoIter {
        self.edges.iter()
    }
}

/// グラフ
#[derive(Debug, Clone)]
pub struct Graph<D, W, I> {
    nodes: Vec<GraphNode<W, I>>,
    edge_num: usize,
    __phantom: PhantomData<D>,
}

/// 有向グラフ
pub type DirectedGraph<W, I> = Graph<Directed, W, I>;
/// 無向グラフ
pub type UndirectedGraph<W, I> = Graph<Undirected, W, I>;

impl<D: Direction, W: Copy, I: Clone> Graph<D, W, I> {
    /// 頂点数が`size`の空の`Graph`を構築する。
    pub fn new(size: usize) -> Self {
        Self {
            nodes: vec![GraphNode { edges: vec![] }; size],
            edge_num: 0,
            __phantom: PhantomData,
        }
    }
}

impl<W, I> Graph<Directed, W, I> {
    /// 有向グラフに辺を追加する。
    pub fn add(&mut self, from: usize, to: usize, weight: W, option: I) {
        self.nodes[from]
            .edges
            .push(Edge::new(from, to, weight, self.edge_num, option));
        self.edge_num += 1;
    }
}

impl<W, I> Extend<(usize, usize, W, I)> for Graph<Directed, W, I> {
    fn extend<T: IntoIterator<Item = (usize, usize, W, I)>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|(from, to, weight, option)| self.add(from, to, weight, option));
    }
}

impl<W: Copy, I: Clone> Graph<Undirected, W, I> {
    /// 無向グラフに辺を追加する。
    pub fn add(&mut self, u: usize, v: usize, weight: W, option: I) {
        self.nodes[u]
            .edges
            .push(Edge::new(u, v, weight, self.edge_num, option.clone()));
        self.nodes[v]
            .edges
            .push(Edge::new(v, u, weight, self.edge_num, option));
        self.edge_num += 1;
    }
}

impl<W: Copy, I: Clone> Extend<(usize, usize, W, I)> for Graph<Undirected, W, I> {
    fn extend<T: IntoIterator<Item = (usize, usize, W, I)>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|(u, v, weight, option)| self.add(u, v, weight, option));
    }
}

impl<D, W, I> Graph<D, W, I> {
    /// 各頂点の[`GraphNode`]への参照のイテレータを返す。
    pub fn nodes_iter(&self) -> impl Iterator<Item = &GraphNode<W, I>> {
        self.nodes.iter()
    }

    /// `i`番目の頂点の[`GraphNode`]への参照を返す。
    pub fn node_of(&self, i: usize) -> &GraphNode<W, I> {
        &self.nodes[i]
    }

    /// グラフの頂点数を返す。
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// グラフの頂点数が`0`ならば`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
