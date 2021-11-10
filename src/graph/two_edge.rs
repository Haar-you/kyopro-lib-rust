use crate::graph::{*, lowlink::*};

pub fn two_edge_connected_components<T>(g: &Graph<T>) -> Vec<Vec<usize>> {
    let ll = Lowlink::new(&g);
    let mut ret = vec![];

    for i in 0..ll.size {
        if ll.par[i].is_none() {
            let index = ret.len();
            ret.push(vec![]);
            rec(&ll, i, index, &mut ret);
        }
    }

    ret
}

fn rec(ll: &Lowlink, cur: usize, index: usize, ret: &mut Vec<Vec<usize>>) {
    ret[index].push(cur);

    for &to in &ll.ch[cur] {
        if ll.ord[cur] < ll.low[to] {
            let index = ret.len();
            ret.push(vec![]);
            rec(ll, to, index, ret);
        } else {
            rec(ll, to, index, ret);
        }
    }
}

