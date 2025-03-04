use std::ops::{Add, AddAssign};
use std::{cmp::Reverse, collections::BinaryHeap};

use crate::trait_alias;
use crate::{graph::*, num::one_zero::Zero};

trait_alias!(
    Elem: Zero + Add<Output = Self> + AddAssign + Ord + Eq + Copy
);

type Path = Vec<usize>;

fn shortest_path<D: Direction, E: EdgeTrait>(
    g: &Graph<D, E>,
    from: usize,
    t: usize,
    usable: &[bool],
    valid: &[Vec<bool>],
) -> Option<(E::Weight, Path)>
where
    E::Weight: Elem,
{
    let n = g.len();
    let mut visited = vec![false; n];
    let mut dist = vec![None; n];
    let mut restore = vec![(0, 0); n];
    let mut pq = BinaryHeap::new();

    dist[from] = Some(E::Weight::zero());
    pq.push(Reverse((E::Weight::zero(), from)));

    while let Some(Reverse((d, i))) = pq.pop() {
        if visited[i] {
            continue;
        }
        visited[i] = true;

        for (k, e) in g.edges[i].iter().enumerate() {
            if !valid[i][k] || !usable[e.to()] {
                continue;
            }

            if dist[e.to()].is_none() || dist[e.to()].unwrap() > d + e.weight() {
                dist[e.to()] = Some(d + e.weight());
                restore[e.to()] = (i, k);
                if !visited[e.to()] {
                    pq.push(Reverse((dist[e.to()].unwrap(), e.to())));
                }
            }
        }
    }

    if let Some(d) = dist[t] {
        let mut p = vec![];

        let mut cur = t;
        while cur != from {
            let (i, j) = restore[cur];
            p.push(j);
            cur = i;
        }

        p.reverse();

        Some((d, p))
    } else {
        None
    }
}

pub fn yen_algorithm<D: Direction, E: EdgeTrait>(
    g: &Graph<D, E>,
    from: usize,
    to: usize,
    k: usize,
) -> Vec<Option<(E::Weight, Path)>>
where
    E::Weight: Elem,
{
    let n = g.len();
    let mut result: Vec<Option<(E::Weight, Path)>> = vec![None; k];
    let mut stock = BinaryHeap::new();
    let mut valid = (0..n)
        .map(|i| vec![true; g.edges[i].len()])
        .collect::<Vec<_>>();

    for i in 0..k {
        if i == 0 {
            let usable = vec![true; n];
            if let Some((c, p)) = shortest_path(g, from, to, &usable, &valid) {
                stock.push(Reverse((c, p)));
            }
        } else {
            let mut prev_path = vec![];

            let mut cur = from;
            for &u in &result[i - 1].as_ref().unwrap().1 {
                prev_path.push(cur);
                cur = g.edges[cur][u].to();
            }
            prev_path.push(to);

            let mut check = vec![true; i];
            let mut usable = vec![true; n];

            for k in 0..prev_path.len() - 1 {
                let u = prev_path[k];

                for j in 0..i {
                    if check[j] {
                        valid[u][result[j].as_ref().unwrap().1[k]] = false;
                    }
                }

                if let Some((mut c, p)) = shortest_path(g, u, to, &usable, &valid) {
                    let mut temp = vec![];

                    for (j, &p) in prev_path.iter().enumerate().take(k) {
                        let v = result[i - 1].as_ref().unwrap().1[j];
                        c += g.edges[p][v].weight();
                        temp.push(v);
                    }
                    // for j in 0..k {
                    //     let v = result[i - 1].as_ref().unwrap().1[j];
                    //     c += g.edges[prev_path[j]][v].weight();
                    //     temp.push(v);
                    // }

                    temp.extend(p.into_iter());
                    stock.push(Reverse((c, temp)));
                }

                usable[u] = false;

                for j in 0..i {
                    if check[j] {
                        valid[u][result[j].as_ref().unwrap().1[k]] = true;
                    }
                }

                for j in 0..i {
                    if check[j]
                        && prev_path[k + 1] != g.edges[u][result[j].as_ref().unwrap().1[k]].to()
                    {
                        check[j] = false;
                    }
                }
            }
        }

        if stock.is_empty() {
            break;
        }

        result[i] = stock.pop().map(|a| a.0);

        while stock.peek().map(|a| &a.0) == result[i].as_ref() {
            stock.pop();
        }
    }

    result
}
