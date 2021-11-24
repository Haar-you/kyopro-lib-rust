#[derive(Clone)]
pub struct DirectedEulerianTrail {
    size: usize,
    edge_count: usize,
    graph: Vec<Vec<u32>>,
    indeg: Vec<i32>,
    outdeg: Vec<i32>,
}

impl DirectedEulerianTrail {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            edge_count: 0,
            graph: vec![vec![]; size],
            indeg: vec![0; size],
            outdeg: vec![0; size],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(to as u32);
        self.indeg[to] += 1;
        self.outdeg[from] += 1;
        self.edge_count += 1;
    }

    fn dfs(&mut self, cur: usize, trail: &mut Vec<usize>) {
        if let Some(next) = self.graph[cur].pop() {
            self.dfs(next as usize, trail);
        }

        while let Some(next) = self.graph[cur].pop() {
            let mut temp = vec![];
            self.dfs(next as usize, &mut temp);
            trail.extend(temp);
        }

        trail.push(cur);
    }

    pub fn solve(mut self) -> Option<Vec<usize>> {
        let mut in_count = 0;
        let mut out_count = 0;
        let mut start = 0;

        for i in 0..self.size {
            match self.outdeg[i] - self.indeg[i] {
                0 => {}
                1 => {
                    out_count += 1;
                    start = i;
                }
                -1 => in_count += 1,
                _ => return None,
            }
        }

        if !(in_count == 0 && out_count == 0 || in_count == 1 && out_count == 1) {
            return None;
        }

        let mut ret = vec![];
        self.dfs(start, &mut ret);
        if ret.len() == self.edge_count + 1 {
            ret.reverse();
            Some(ret)
        } else {
            None
        }
    }
}
