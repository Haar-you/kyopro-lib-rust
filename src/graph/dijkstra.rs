use std::collections::BinaryHeap;
use std::cmp::Reverse;

use crate::graph::template::*;

pub fn dijkstra<T>(g: &Graph<T>, src: Vec<usize>, zero: T) -> Vec<Option<T>>
where
    T: std::clone::Clone + std::ops::Add<Output=T> + Ord
{
    let n = g.len();
    let mut ret = vec![None; n];
    let mut heap: BinaryHeap<Reverse<(T, usize)>> = BinaryHeap::new();
    let mut check = vec![false; n];

    for u in src {
        ret[u] = Some(zero.clone());
        heap.push(Reverse((zero.clone(), u)));
    }

    while let Some(Reverse((d, u))) = heap.pop() {
        if check[u] {
            continue;
        }
        check[u] = true;

        for &Edge {from: _, to, ref cost} in &g.edges[u] {
            if let Some(ref d2) = ret[to] {
                if *d2 > d.clone() + cost.clone() {
                    let d = d.clone() + cost.clone();
                    ret[to] = Some(d.clone());
                    if !check[to] {
                        heap.push(Reverse((d, to)));
                    }
                }
            }
            else {
                let d = d.clone() + cost.clone();
                ret[to] = Some(d.clone());
                heap.push(Reverse((d, to)));
            }
        }
    }

    ret
}
