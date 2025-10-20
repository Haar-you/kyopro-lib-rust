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
pub mod detect_cycle;
pub mod eulerian;

pub mod bipartite;

pub mod enumerate_triangles;
pub mod max_independent_set;

pub mod chu_liu_edmonds;
pub mod kruskal;
pub mod prim;

pub mod tsort;

pub mod chinese_postman;
pub mod tsp;

pub mod chromatic_number;

use std::marker::PhantomData;

/// [`Graph`]にもたせる辺の満たすトレイト。
pub trait EdgeTrait {
    /// 辺の重みの型
    type Weight;
    /// 辺の始点
    fn from(&self) -> usize;
    /// 辺の終点
    fn to(&self) -> usize;
    /// 辺の重み
    fn weight(&self) -> Self::Weight;
    /// 逆辺
    fn rev(self) -> Self;
}

/// グラフの辺
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Edge<T, I> {
    /// 辺の始点
    pub from: usize,
    /// 辺の終点
    pub to: usize,
    /// 辺の重み
    pub weight: T,
    /// 辺の番号など
    pub index: I,
}

impl<T, I> Edge<T, I> {
    /// `from`から`to`への重さ`weight`、辺番号`index`をもつ有向辺を作る。
    pub fn new(from: usize, to: usize, weight: T, index: I) -> Self {
        Self {
            from,
            to,
            weight,
            index,
        }
    }
}

impl<T: Clone, I> EdgeTrait for Edge<T, I> {
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

/// グラフの辺の有向・無向の情報をもたせるためのトレイト。
pub trait Direction {}
/// 有向辺をもつ。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Directed;
/// 無向辺をもつ。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Undirected;
impl Direction for Directed {}
impl Direction for Undirected {}

/// グラフのノード
#[derive(Clone, Debug)]
pub struct GraphNode<E> {
    /// 接続する辺
    pub edges: Vec<E>,
}

impl<E: EdgeTrait> GraphNode<E> {
    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.edges.iter()
    }
}

impl<E: EdgeTrait> IntoIterator for GraphNode<E> {
    type Item = E;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.edges.into_iter()
    }
}

impl<'a, E: EdgeTrait> IntoIterator for &'a GraphNode<E> {
    type Item = &'a E;
    type IntoIter = std::slice::Iter<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.edges.iter()
    }
}

/// グラフ
#[derive(Debug, Clone)]
pub struct Graph<D, E> {
    nodes: Vec<GraphNode<E>>,
    __phantom: PhantomData<D>,
}

impl<D: Direction, E: EdgeTrait + Clone> Graph<D, E> {
    /// 頂点数が`size`の空の`Graph`を構築する。
    pub fn new(size: usize) -> Self {
        Self {
            nodes: vec![GraphNode { edges: vec![] }; size],
            __phantom: PhantomData,
        }
    }
}

impl<E: EdgeTrait + Clone> Graph<Directed, E> {
    /// 有向グラフに辺を追加する。
    pub fn add(&mut self, e: E) {
        self.nodes[e.from()].edges.push(e);
    }
}

impl<E: EdgeTrait + Clone> Extend<E> for Graph<Directed, E> {
    fn extend<T: IntoIterator<Item = E>>(&mut self, iter: T) {
        iter.into_iter().for_each(|e| self.add(e));
    }
}

impl<E: EdgeTrait + Clone> Graph<Undirected, E> {
    /// 無向グラフに辺を追加する。
    pub fn add(&mut self, e: E) {
        self.nodes[e.from()].edges.push(e.clone());
        self.nodes[e.to()].edges.push(e.rev());
    }
}

impl<E: EdgeTrait + Clone> Extend<E> for Graph<Undirected, E> {
    fn extend<T: IntoIterator<Item = E>>(&mut self, iter: T) {
        iter.into_iter().for_each(|e| self.add(e));
    }
}

impl<D, E> Graph<D, E> {
    /// 各頂点の[`GraphNode`]への参照のイテレータを返す。
    pub fn nodes_iter(&self) -> impl Iterator<Item = &GraphNode<E>> {
        self.nodes.iter()
    }

    /// `i`番目の頂点の[`GraphNode`]への参照を返す。
    pub fn node_of(&self, i: usize) -> &GraphNode<E> {
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
