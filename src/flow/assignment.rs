//! 割当問題
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/assignment>

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    rev: usize,
    cap: bool,
    cost: i64,
    is_rev: bool,
}

fn add_edge(edges: &mut Vec<Vec<Edge>>, u: usize, v: usize, cost: i64) {
    let rev = edges[v].len();
    edges[u].push(Edge {
        to: v,
        rev,
        cap: true,
        cost,
        is_rev: false,
    });
    let rev = edges[u].len() - 1;
    edges[v].push(Edge {
        to: u,
        rev,
        cap: false,
        cost: -cost,
        is_rev: true,
    });
}

/// 割当問題を解く。
pub fn assignment(a: Vec<Vec<i64>>) -> (i64, Vec<usize>) {
    let n = a.len();
    assert!(a.iter().all(|v| v.len() == n));

    let size = n * 2 + 1;
    let mut edges = vec![vec![]; n * 2 + 1];

    for i in 0..n {
        for j in 0..n {
            add_edge(&mut edges, i, n + j, a[i][j]);
        }
    }

    let sink = 2 * n;

    for i in 0..n {
        add_edge(&mut edges, n + i, sink, 0);
    }

    let mut min_cost = 0;
    let mut h = vec![0; size];

    for src in 0..n {
        let mut prev = vec![(0, 0); size];
        let mut cost = vec![None; size];
        let mut pq = vec![None; size];

        cost[src] = Some(0);
        pq[src] = Some(0);

        loop {
            let Some((c, v)) = pq
                .iter()
                .enumerate()
                .filter_map(|(i, c)| c.map(|x| (x, i)))
                .min()
            else {
                break;
            };

            pq[v] = None;

            let h_v = h[v];

            for (i, e) in edges[v].iter().enumerate() {
                if e.cap
                    && (cost[e.to].is_none() || cost[e.to].unwrap() + h[e.to] > c + h_v + e.cost)
                {
                    cost[e.to] = Some(c + e.cost + h_v - h[e.to]);
                    prev[e.to] = (v, i);
                    pq[e.to] = pq[e.to].map(|x| x.min(cost[e.to].unwrap())).or(cost[e.to]);
                }
            }
        }

        for i in 0..size {
            if let Some(x) = cost[i] {
                h[i] += x;
            }
        }

        min_cost += h[sink];

        let mut cur = sink;
        while cur != src {
            let e = &mut edges[prev[cur].0][prev[cur].1];
            e.cap ^= true;
            let rev = e.rev;
            edges[cur][rev].cap ^= true;
            cur = prev[cur].0;
        }
    }

    let assignment = (0..n)
        .map(|i| {
            let k = edges[i].iter().find(|e| !e.is_rev && !e.cap).unwrap().to;
            k - n
        })
        .collect();

    (min_cost, assignment)
}
