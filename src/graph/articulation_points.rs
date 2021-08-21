use crate::graph::template::*;
use std::cmp::min;

impl<T> Graph<T> {
    pub fn articulation_points(&self) -> Vec<usize> {
        let n = self.len();
        let mut visit = vec![None; n];
        let mut low = vec![0; n];
        let mut ret = vec![];
        let mut v = 0;

        for i in 0..n {
            if visit[i].is_none() {
                self.articulation_points_(i, i, &mut visit, &mut low, &mut ret, &mut v);
            }
        }

        ret
    }

    fn articulation_points_(
        &self,
        root: usize,
        cur: usize,
        visit: &mut Vec<Option<usize>>,
        low: &mut Vec<usize>,
        ret: &mut Vec<usize>,
        v: &mut usize,
    ) -> usize {
        if let Some(x) = visit[cur] {
            return x;
        }

        visit[cur] = Some(*v);

        let mut temp = *v;
        let mut children = vec![];
        *v += 1;

        for &Edge {
            from: _,
            to,
            cost: _,
        } in &self.edges[cur]
        {
            if visit[to].is_none() {
                children.push(to);
            }
            let t = self.articulation_points_(root, to, visit, low, ret, v);
            temp = min(temp, t);
        }

        low[cur] = temp;

        if cur != root || children.len() >= 2 {
            for x in children {
                if low[x] >= visit[cur].unwrap() {
                    ret.push(cur);
                    break;
                }
            }
        }

        low[cur]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_3_A
        let g =
            Graph::<u32>::from_tuples_undirected(4, &[(0, 1, 1), (0, 2, 1), (1, 2, 1), (2, 3, 1)]);
        let mut ans = g.articulation_points();
        ans.sort();
        assert_eq!(ans, [2]);

        let g =
            Graph::<u32>::from_tuples_undirected(5, &[(0, 1, 1), (1, 2, 1), (2, 3, 1), (3, 4, 1)]);
        let mut ans = g.articulation_points();
        ans.sort();
        assert_eq!(ans, [1, 2, 3]);
    }
}
