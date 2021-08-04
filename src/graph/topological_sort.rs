use std::collections::VecDeque;

use crate::graph::template::*;

impl<T> Graph<T> {
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.len();
        let mut indeg = vec![0; n];

        for i in 0 .. n {
            for &Edge { from: _, to, cost: _ } in &self.edges[i] {
                indeg[to] += 1;
            }
        }

        let mut q = VecDeque::new();

        for i in 0 .. n {
            if indeg[i] == 0 {
                q.push_back(i);
            }
        }

        let mut ret = vec![];

        while let Some(cur) = q.pop_front() {
            ret.push(cur);
            for &Edge { from: _, to, cost: _ } in &self.edges[cur] {
                indeg[to] -= 1;
                if indeg[to] == 0 {
                    q.push_back(to);
                }
            }
        }

        if ret.len() == n {
            Some(ret)
        }
        else {
            None
        }
    }
}
