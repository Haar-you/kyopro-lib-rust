use crate::graph::*;

#[derive(Clone)]
pub struct UndirectedEulerianTrail<E: EdgeTrait> {
    size: usize,
    edge_count: usize,
    graph: Vec<Vec<(usize, E)>>,
    deg: Vec<u32>,
}

impl<E: EdgeTrait + Clone> UndirectedEulerianTrail<E> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            edge_count: 0,
            graph: vec![vec![]; size],
            deg: vec![0; size],
        }
    }

    pub fn add_edge(&mut self, e: E) {
        let from = e.from();
        let to = e.to();

        if from == to {
            let rindex = self.graph[from].len();
            self.graph[from].push((rindex, e));
        } else {
            let rindex = self.graph[to].len();
            self.graph[from].push((rindex, e.clone()));

            let rindex = self.graph[from].len() - 1;
            self.graph[to].push((rindex, e.rev()));
        }

        self.deg[from] += 1;
        self.deg[to] += 1;
        self.edge_count += 1;
    }

    fn dfs(&mut self, cur: usize, vs: &mut Vec<usize>, es: &mut Vec<E>) {
        while let Some((rindex, e)) = self.graph[cur].pop() {
            let from = e.from();
            let to = e.to();
            if from != to {
                self.graph[to].swap_remove(rindex);

                if let Some((index, e)) = self.graph[to].get(rindex).cloned() {
                    if e.from() != e.to() {
                        self.graph[e.to()][index].0 = rindex;
                    }
                }
            }

            self.dfs(to, vs, es);
            es.push(e);
        }

        vs.push(cur);
    }

    pub fn solve(mut self) -> Option<(Vec<usize>, Vec<E>)> {
        let mut odd = 0;
        let mut start = None;

        for i in 0..self.size {
            if self.deg[i] % 2 == 1 {
                odd += 1;
                start = Some(i);
            }
            if start.is_none() && !self.graph[i].is_empty() {
                start = Some(i);
            }
        }

        let start = start.unwrap_or(0);

        if odd != 0 && odd != 2 {
            return None;
        }

        let mut vs = vec![];
        let mut es = vec![];
        self.dfs(start, &mut vs, &mut es);
        if vs.len() == self.edge_count + 1 && es.len() == self.edge_count {
            vs.reverse();
            es.reverse();

            Some((vs, es))
        } else {
            None
        }
    }
}
