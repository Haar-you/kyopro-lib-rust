//! 二重頂点連結分解

pub use crate::graph::lowlink::Lowlink;

pub fn biconnected(ll: &Lowlink) -> Vec<(Vec<usize>, Vec<(usize, usize)>)> {
    let n = ll.size;

    let mut check = vec![false; n];
    let mut ret = vec![];
    let mut temp = vec![];
    let mut vcheck = vec![false; n];

    for i in 0..n {
        if !check[i] {
            if ll.ch[i].is_empty() && ll.par[i].is_none() {
                ret.push((vec![i], vec![]));
            } else {
                dfs(i, ll, &mut check, &mut vcheck, &mut temp, &mut ret);
            }
        }
    }

    ret
}

fn dfs(
    cur: usize,
    ll: &Lowlink,
    check: &mut [bool],
    vcheck: &mut [bool],
    stack: &mut Vec<(usize, usize)>,
    ret: &mut Vec<(Vec<usize>, Vec<(usize, usize)>)>,
) {
    check[cur] = true;

    for &to in ll.ch[cur].iter().chain(ll.back[cur].iter()) {
        if !check[to] || ll.ord[to] < ll.ord[cur] {
            stack.push((cur, to));
        }
        if !check[to] {
            dfs(to, ll, check, vcheck, stack, ret);

            if ll.low[to] >= ll.ord[cur] {
                let mut es = vec![];
                let mut vs = vec![];

                while let Some(e) = stack.pop() {
                    let (u, v) = e;
                    es.push(e);

                    if !vcheck[u] {
                        vs.push(u);
                        vcheck[u] = true;
                    }
                    if !vcheck[v] {
                        vs.push(v);
                        vcheck[v] = true;
                    }

                    if u == cur && v == to {
                        break;
                    }
                }

                for &i in &vs {
                    vcheck[i] = false;
                }

                ret.push((vs, es));
            }
        }
    }
}
