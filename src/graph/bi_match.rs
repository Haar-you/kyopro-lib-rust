use crate::graph::flow::MaxFlow;

pub struct BipartiteMatching<F> {
    left: usize,
    right: usize,
    flow: F,
    s: usize,
    t: usize,
}

impl<F: MaxFlow<Cap = u64>> BipartiteMatching<F> {
    pub fn new(left: usize, right: usize) -> Self {
        let mut flow = F::new(left + right + 2);
        let s = left + right;
        let t = s + 1;

        for i in 0..left {
            flow.add_edge(s, i, 1);
        }
        for i in 0..right {
            flow.add_edge(left + i, t, 1);
        }

        Self {
            left,
            right,
            flow,
            s,
            t,
        }
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        assert!(i < self.left);
        assert!(j < self.right);
        self.flow.add_edge(i, self.left + j, 1);
    }

    pub fn matching(&mut self) -> u64 {
        self.flow.max_flow(self.s, self.t)
    }
}
