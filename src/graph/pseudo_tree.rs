use std::collections::VecDeque;

pub struct PseudoTreeBuilder {
    g: Vec<Vec<usize>>,
    edge_num: usize,
}

pub struct PseudoTree {
    group: Vec<usize>,
    in_loop: Vec<bool>,
}

impl PseudoTreeBuilder {
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![vec![]; n],
            edge_num: 0,
        }
    }

    pub fn add(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
        self.edge_num += 1;
    }

    pub fn build(self) -> PseudoTree {
        assert_eq!(self.edge_num, self.g.len());
        let n = self.g.len();
        let mut indeg = vec![0; n];
        let mut visit = vec![false; n];
        let mut in_loop = vec![true; n];

        for &to in self.g.iter().flatten() {
            indeg[to] += 1;
        }

        let mut queue: VecDeque<_> = indeg
            .iter()
            .enumerate()
            .filter_map(|(i, &deg)| (deg == 1).then_some(i))
            .collect();

        while let Some(cur) = queue.pop_front() {
            in_loop[cur] = false;

            if visit[cur] {
                continue;
            }
            visit[cur] = true;

            for &to in &self.g[cur] {
                if !visit[to] {
                    indeg[to] -= 1;

                    if indeg[to] == 1 {
                        queue.push_back(to);
                    }
                }
            }
        }

        let mut group = vec![0; n];

        for i in 0..n {
            if in_loop[i] {
                group[i] = i;
                for &to in &self.g[i] {
                    if !in_loop[to] {
                        self.__dfs(to, i, &mut group);
                    }
                }
            }
        }

        PseudoTree { group, in_loop }
    }

    fn __dfs(&self, cur: usize, par: usize, group: &mut [usize]) {
        group[cur] = group[par];

        for &to in &self.g[cur] {
            if to != par {
                self.__dfs(to, cur, group);
            }
        }
    }
}

impl PseudoTree {
    pub fn group_of(&self, i: usize) -> usize {
        self.group[i]
    }

    pub fn is_in_loop(&self, i: usize) -> bool {
        self.in_loop[i]
    }
}
