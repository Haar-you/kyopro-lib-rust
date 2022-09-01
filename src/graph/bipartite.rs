//! 二部グラフ判定

use crate::graph::*;

pub fn check_bipartite<E: EdgeTrait>(
    g: &Graph<Undirected, E>,
) -> Vec<Option<(Vec<usize>, Vec<usize>)>> {
    let n = g.len();
    let mut ret = vec![];
    let mut check = vec![-1; n];
    let mut visit = vec![false; n];

    for i in 0..n {
        if visit[i] {
            continue;
        }

        let mut a = vec![];
        let mut b = vec![];

        let res = (|| {
            let mut stack = vec![i];
            check[i] = 0;
            a.push(i);

            while let Some(cur) = stack.pop() {
                if visit[cur] {
                    continue;
                }
                visit[cur] = true;

                for e in &g.edges[cur] {
                    let to = e.to();
                    if check[to] == check[cur] {
                        return false;
                    }
                    if check[to] == -1 {
                        if check[cur] == 0 {
                            check[to] = 1;
                            b.push(to);
                        } else {
                            check[to] = 0;
                            a.push(to);
                        }

                        stack.push(to);
                    }
                }
            }

            true
        })();

        ret.push(if res { Some((a, b)) } else { None });
    }

    ret
}
