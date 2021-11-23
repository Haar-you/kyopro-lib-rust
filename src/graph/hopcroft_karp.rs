use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Edge {
    to: usize,
    rev: Option<usize>,
    used: bool,
}

impl Edge {
    fn new(to: usize, rev: Option<usize>, used: bool) -> Self {
        Self { to, rev, used }
    }
}

pub struct HopcroftKarp {
    left: usize,
    right: usize,
    size: usize,
    graph: Vec<Vec<Edge>>,
    dist: Vec<Option<usize>>,
}

impl HopcroftKarp {
    pub fn new(left: usize, right: usize) -> Self {
        let size = left + right + 2;
        let dist = vec![None; size];

        let mut graph = vec![vec![]; size];
        for i in 0..left {
            graph[0].push(Edge::new(i + 1, None, false));
        }
        for i in 0..right {
            graph[i + left + 1].push(Edge::new(size - 1, None, false));
        }

        Self {
            left,
            right,
            size,
            graph,
            dist,
        }
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        assert!(i < self.left);
        assert!(j < self.right);

        let i = i + 1;
        let j = j + self.left + 1;

        let e = Edge::new(j, Some(self.graph[j].len()), false);
        self.graph[i].push(e);

        let e = Edge::new(i, Some(self.graph[i].len() - 1), true);
        self.graph[j].push(e);
    }

    fn bfs(&mut self) -> bool {
        self.dist.iter_mut().for_each(|x| *x = None);

        let mut q = VecDeque::new();
        q.push_back(0);
        self.dist[0] = Some(0);

        while let Some(i) = q.pop_front() {
            for &Edge { to, used, .. } in &self.graph[i] {
                if !used && self.dist[to].is_none() {
                    self.dist[to] = Some(self.dist[i].unwrap() + 1);
                    q.push_back(to);
                }
            }
        }

        self.dist.last().is_some()
    }

    fn dfs(&mut self, cur: usize) -> bool {
        if cur == self.size - 1 {
            true
        } else {
            for i in 0..self.graph[cur].len() {
                let Edge { to, used, .. } = self.graph[cur][i];

                if !used && self.dist[cur].unwrap() + 1 == self.dist[to].unwrap() && self.dfs(to) {
                    let e = &mut self.graph[cur][i];

                    e.used = true;
                    if let Some(rev) = e.rev {
                        self.graph[to][rev].used = false;
                    }

                    return true;
                }
            }

            false
        }
    }

    pub fn matching(&mut self) -> u32 {
        let mut ret = 0;

        while self.bfs() {
            let mut flow = 0;
            for i in 0..self.left {
                let e = &mut self.graph[0][i];
                let to = e.to;
                if !e.used && self.dfs(to) {
                    let e = &mut self.graph[0][i];
                    e.used = true;
                    flow += 1;
                }
            }

            if flow == 0 {
                break;
            }

            ret += flow;
        }

        ret
    }

    pub fn get_matching(&self) -> Vec<(usize, usize)> {
        let mut ret = vec![];

        for i in 0..self.left {
            for &Edge { to, used, .. } in &self.graph[i + 1] {
                if used {
                    ret.push((i, to - self.left - 1));
                }
            }
        }

        ret
    }
}
