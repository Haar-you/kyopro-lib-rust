
#[derive(Debug, Clone)]
pub struct Edge<T> {
    pub from: usize,
    pub to: usize,
    pub cost: T
}

impl<T> Edge<T> {
    fn new(from: usize, to: usize, cost: T) -> Self {
        Edge {
            from: from, to: to, cost: cost
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph<T> {
    pub edges: Vec<Vec<Edge<T>>>
}

impl<T: std::clone::Clone> Graph<T> {
    pub fn new(size: usize) -> Self {
        Graph {
            edges: vec![vec![]; size]
        }
    }

    pub fn add_directed(&mut self, from: usize, to: usize, cost: T) -> &mut Self {
        self.edges[from].push(Edge::new(from, to, cost));
        self
    }

    pub fn add_undirected(&mut self, from: usize, to: usize, cost: T) -> &mut Self {
        self.add_directed(from, to, cost.clone());
        self.add_directed(to, from, cost);
        self
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }
}
