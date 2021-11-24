use std::collections::BTreeMap;

#[derive(Clone)]
pub struct UndirectedEulerianTrail {
    size: usize,
    edge_count: usize,
    graph: Vec<BTreeMap<usize, u32>>,
    deg: Vec<u32>,
}

impl UndirectedEulerianTrail {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            edge_count: 0,
            graph: vec![BTreeMap::new(); size],
            deg: vec![0; size],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        *self.graph[from].entry(to).or_insert(0) += 1;
        *self.graph[to].entry(from).or_insert(0) += 1;
        self.deg[from] += 1;
        self.deg[to] += 1;
        self.edge_count += 1;
    }

    fn del(&mut self, i: usize, j: usize) {
        if self.graph[i].get(&j) == Some(&1) {
            self.graph[i].remove(&j);
        } else {
            *self.graph[i].get_mut(&j).unwrap() -= 1;
        }

        if self.graph[j].get(&i) == Some(&1) {
            self.graph[j].remove(&i);
        } else {
            *self.graph[j].get_mut(&i).unwrap() -= 1;
        }
    }

    fn dfs(&mut self, cur: usize, trail: &mut Vec<usize>) {
        if let Some((&next, _)) = self.graph[cur].iter().next() {
            self.del(cur, next);
            self.dfs(next, trail);
        }

        while let Some((&next, _)) = self.graph[cur].iter().next() {
            self.del(cur, next);
            let mut temp = vec![];
            self.dfs(next, &mut temp);
            trail.extend(temp);
        }

        trail.push(cur);
    }

    pub fn solve(mut self) -> Option<Vec<usize>> {
        let mut odd = 0;
        let mut start = 0;

        for i in 0..self.size {
            if self.deg[i] % 2 == 1 {
                odd += 1;
                start = i;
            }
        }

        if odd != 0 && odd != 2 {
            return None;
        }

        let mut ret = vec![];
        self.dfs(start, &mut ret);
        if ret.len() == self.edge_count + 1 {
            Some(ret)
        } else {
            None
        }
    }
}
