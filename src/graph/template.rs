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
        self.add_directed(from, to, cost.clone());
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

impl<T> Graph<T> {
    pub fn len(&self) -> usize {
        self.edges.len()
    }
}
