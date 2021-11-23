pub mod articulation_points;
pub mod bellman_ford;
pub mod bipartite;
pub mod bridges;
pub mod chinese_postman;
pub mod detect_cycle;
pub mod dijkstra;
pub mod kruskal;
pub mod lowlink;
pub mod max_independent_set;
pub mod prim;
pub mod scc;
pub mod tsort;
pub mod tsp;
pub mod two_edge;
pub mod warshall_floyd;

pub mod dinic;
pub mod flow;
pub mod ford_fulkerson;
pub mod hopcroft_karp;

pub mod bi_match;

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

#[derive(Debug, Clone)]
pub struct Graph<E> {
    pub edges: Vec<Vec<E>>,
}

impl<E: EdgeTrait + Clone> Graph<E> {
    pub fn new(size: usize) -> Self {
        Graph {
            edges: vec![vec![]; size],
        }
    }

    pub fn add_directed(&mut self, edges: impl IntoIterator<Item = E>) {
        for e in edges.into_iter() {
            self.edges[e.from()].push(e);
        }
    }

    pub fn add_undirected(&mut self, edges: impl IntoIterator<Item = E>) {
        for e in edges.into_iter() {
            self.edges[e.from()].push(e.clone());
            self.edges[e.to()].push(e.rev());
        }
    }
}

impl<E> Graph<E> {
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}
