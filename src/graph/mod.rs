pub mod articulation_points;
pub mod bellman_ford;
pub mod bipartite;
pub mod bridges;
pub mod chinese_postman;
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
pub mod hopcroft_karp;

#[derive(Debug, Clone)]
pub struct Edge<T> {
    pub from: usize,
    pub to: usize,
    pub cost: T,
}

impl<T> Edge<T> {
    fn new(from: usize, to: usize, cost: T) -> Self {
        Edge { from, to, cost }
    }
}

#[derive(Debug, Clone)]
pub struct Graph<T> {
    pub edges: Vec<Vec<Edge<T>>>,
}

impl<T: Copy> Graph<T> {
    pub fn new(size: usize) -> Self {
        Graph {
            edges: vec![vec![]; size],
        }
    }

    pub fn add_directed(&mut self, from: usize, to: usize, cost: T) {
        self.edges[from].push(Edge::new(from, to, cost));
    }

    pub fn add_undirected(&mut self, from: usize, to: usize, cost: T) {
        self.add_directed(from, to, cost);
        self.add_directed(to, from, cost);
    }

    pub fn from_tuples(n: usize, edges: &[(usize, usize, T)]) -> Self {
        let mut ret = Graph::<T>::new(n);
        for &(u, v, c) in edges {
            ret.add_directed(u, v, c);
        }
        ret
    }

    pub fn from_tuples_undirected(n: usize, edges: &[(usize, usize, T)]) -> Self {
        let mut ret = Graph::<T>::new(n);
        for &(u, v, c) in edges {
            ret.add_undirected(u, v, c);
        }
        ret
    }
}

impl<T: Copy + From<u32>> Graph<T> {
    pub fn from_tuples_unweighted(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut ret = Graph::<T>::new(n);
        for &(u, v) in edges {
            ret.add_directed(u, v, T::from(1));
        }
        ret
    }

    pub fn from_tuples_undirected_unweighted(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut ret = Graph::<T>::new(n);
        for &(u, v) in edges {
            ret.add_undirected(u, v, T::from(1));
        }
        ret
    }
}

impl<T> Graph<T> {
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}
