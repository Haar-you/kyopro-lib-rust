use crate::graph::template::*;

impl<T> Graph<T> {
    pub fn check_bipartite(&self) -> Vec<Option<(Vec<usize>, Vec<usize>)>> {
        let mut ret = Vec::new();
        let n = self.len();
        let mut check = vec![-1; n];
        let mut visit = vec![false; n];

        for i in 0 .. n {
            if visit[i] {
                continue;
            }

            let mut a = Vec::new();
            let mut b = Vec::new();

            let res = (|| {
                let mut stack = Vec::new();

                stack.push(i);
                check[i] = 0;
                a.push(i);

                while let Some(cur) = stack.pop() {
                    if visit[cur] {
                        continue;
                    }
                    visit[cur] = true;

                    for &Edge { from: _, to, cost: _ } in &self.edges[cur] {
                        if check[to] == check[cur] {
                            return false;
                        }
                        if check[to] == -1 {
                            if check[cur] == 0 {
                                check[to] = 1;
                                b.push(to);
                            }
                            else {
                                check[to] = 0;
                                a.push(to);
                            }

                            stack.push(to);
                        }
                    }
                }

                return true;
            })();

            ret.push(if res { Some((a, b)) } else { None });
        }

        ret
    }
}
