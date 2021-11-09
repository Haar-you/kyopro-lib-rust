use crate::{chmin, graph::*};

struct SccRecParams<'a> {
    ret: &'a mut Vec<usize>,
    low: &'a mut Vec<isize>,
    ord: &'a mut Vec<isize>,
    s: &'a mut Vec<usize>,
    check: &'a mut Vec<bool>,
    t: &'a mut usize,
    k: &'a mut usize,
}

pub fn scc<T>(g: &Graph<T>) -> (Vec<usize>, usize) {
    let n = g.len();

    let mut ret = vec![0; n];
    let mut low = vec![-1; n];
    let mut ord = vec![-1; n];
    let mut s = vec![];
    let mut check = vec![false; n];
    let mut k = 0;

    for i in 0..n {
        if ord[i] == -1 {
            let mut t = 0;

            let mut params = SccRecParams {
                ret: &mut ret,
                low: &mut low,
                ord: &mut ord,
                s: &mut s,
                check: &mut check,
                t: &mut t,
                k: &mut k,
            };

            scc_(g, i, &mut params);
        }
    }

    ret.iter_mut().for_each(|x| *x = k - 1 - *x);

    (ret, k)
}

fn scc_<T>(g: &Graph<T>, cur: usize, params: &mut SccRecParams) {
    *params.t += 1;
    params.low[cur] = *params.t as isize;
    params.ord[cur] = *params.t as isize;

    params.s.push(cur);
    params.check[cur] = true;

    for &Edge { to, .. } in &g.edges[cur] {
        if params.ord[to] == -1 {
            scc_(&g, to, params);
            chmin!(params.low[cur], params.low[to]);
        } else if params.check[to] {
            chmin!(params.low[cur], params.low[to]);
        }
    }

    if params.low[cur] == params.ord[cur] {
        loop {
            let u = params.s.pop().unwrap();
            params.check[u] = false;
            params.ret[u] = *params.k;
            if cur == u {
                break;
            }
        }
        *params.k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/all/GRL_3_C
        let g = Graph::<i32>::from_tuples(
            5,
            &[
                (0, 1, 1),
                (1, 0, 1),
                (1, 2, 1),
                (2, 4, 1),
                (4, 3, 1),
                (3, 2, 1),
            ],
        );
        let scc = scc(&g).0;

        assert_eq!(scc[0], scc[1]);
        assert_ne!(scc[0], scc[3]);
        assert_eq!(scc[2], scc[3]);
        assert_eq!(scc[3], scc[4]);
    }
}
