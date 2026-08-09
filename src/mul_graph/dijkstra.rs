//! 頂点倍加グラフ上でのDijkstra法
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;

pub use crate::mul_graph::MulGraph;
use crate::num::one_zero::Zero;
use crate::num::traits::Unsigned;

/// Dijkstra法
pub fn dijkstra<V, W>(graph: &MulGraph<V, W>, src: &[V]) -> HashMap<V, W>
where
    V: Hash + Eq + Copy + Ord,
    W: Copy + Ord + Zero + Add<Output = W> + Unsigned,
{
    let zero = W::zero();
    let mut ret = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut check = HashSet::new();

    for &u in src {
        ret.insert(u, zero);
        heap.push(Reverse((zero, u)));
    }

    while let Some(Reverse((d, u))) = heap.pop() {
        if check.contains(&u) {
            continue;
        }
        check.insert(u);

        for e in graph.neighbours_of(u) {
            let to = e.to;
            let cost = e.weight;

            match ret.get(&to) {
                Some(&d2) if d2 <= d + cost => {}
                _ => {
                    let d = d + cost;
                    ret.insert(to, d);
                    if !check.contains(&to) {
                        heap.push(Reverse((d, to)));
                    }
                }
            }
        }
    }

    ret
}
