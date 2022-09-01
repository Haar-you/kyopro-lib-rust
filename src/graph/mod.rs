//! グラフ一般に関するもの

pub mod articulation_points;
pub mod biconnected;
pub mod bridges;
pub mod lowlink;
pub mod scc;
pub mod two_edge;

pub mod pseudo_tree;

pub mod bellman_ford;
pub mod bfs;
pub mod dijkstra;
pub mod warshall_floyd;

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

pub mod psp;

pub mod dinic;
pub mod flow;
pub mod ford_fulkerson;
pub mod hopcroft_karp;
pub mod min_cost_flow;

pub mod bi_match;

use std::marker::PhantomData;

pub trait EdgeTrait {
    type Weight;
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    fn weight(&self) -> Self::Weight;
    fn rev(self) -> Self;
}

#[derive(Debug, Clone)]
pub struct Edge<T, I> {
    pub from: usize,
    pub to: usize,
    pub weight: T,
    pub index: I,
}

impl<T, I> Edge<T, I> {
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

pub trait Direction {}
pub struct Directed;
pub struct Undirected;
impl Direction for Directed {}
impl Direction for Undirected {}

#[derive(Debug, Clone)]
pub struct Graph<D, E> {
    pub edges: Vec<Vec<E>>,
    pub __phantom: PhantomData<D>,
}

impl<D: Direction, E: EdgeTrait + Clone> Graph<D, E> {
    pub fn new(size: usize) -> Self {
        Graph {
            edges: vec![vec![]; size],
            __phantom: PhantomData,
        }
    }
}

impl<E: EdgeTrait + Clone> Graph<Directed, E> {
    pub fn add(&mut self, edges: impl IntoIterator<Item = E>) {
        for e in edges.into_iter() {
            self.edges[e.from()].push(e);
        }
    }
}

impl<E: EdgeTrait + Clone> Graph<Undirected, E> {
    pub fn add(&mut self, edges: impl IntoIterator<Item = E>) {
        for e in edges.into_iter() {
            self.edges[e.from()].push(e.clone());
            self.edges[e.to()].push(e.rev());
        }
    }
}

impl<D, E> Graph<D, E> {
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}
