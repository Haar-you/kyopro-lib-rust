use crate::graph::template::*;

impl<T> Graph<T> {
    pub fn scc(&self) -> (Vec<usize>, usize) {
        let n = self.len();

        let mut ret = vec![0; n];
        let mut low = vec![-1; n];
        let mut ord = vec![-1; n];
        let mut s = vec![];
        let mut check = vec![false; n];
        let mut k = 0;

        for i in 0 .. n {
            if ord[i] == -1 {
                let mut t = 0;
                self.scc_(i, &mut ret, &mut low, &mut ord, &mut s, &mut check, &mut t, &mut k);
            }
        }

        for x in &mut ret {
            *x = k - 1 - *x;
        }

        (ret, k)
    }

    fn scc_(&self, cur: usize, ret: &mut Vec<usize>, low: &mut Vec<isize>, ord: &mut Vec<isize>,
            s: &mut Vec<usize>, check: &mut Vec<bool>, t: &mut usize, k: &mut usize
    ) {
        *t += 1;
        low[cur] = *t as isize;
        ord[cur] = *t as isize;

        s.push(cur);
        check[cur] = true;

        for &Edge { from: _, to, cost: _ } in &self.edges[cur] {
            if ord[to] == -1 {
                self.scc_(to, ret, low, ord, s, check, t, k);
                low[cur] = std::cmp::min(low[cur], low[to]);
            }
            else if check[to] {
                low[cur] = std::cmp::min(low[cur], low[to]);
            }
        }

        if low[cur] == ord[cur] {
            loop {
                let u = s.pop().unwrap();
                check[u] = false;
                ret[u] = *k;
                if cur == u {
                    break;
                }
            }
            *k += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_3_C
        let mut g = Graph::<i32>::new(5);
        g.from_tuples(&[(0, 1, 1), (1, 0, 1), (1, 2, 1), (2, 4, 1), (4, 3, 1), (3, 2, 1)]);
        let scc = g.scc().0;

        assert_eq!(scc[0], scc[1]);
        assert_ne!(scc[0], scc[3]);
        assert_eq!(scc[2], scc[3]);
        assert_eq!(scc[3], scc[4]);
    }
}
