//! グラフの彩色数
//!
//! # References
//! - <https://www.slideshare.net/slideshow/ss-12131479/12131479>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/chromatic_number>
use crate::{graph::*, math::mod_ops::pow::mod_pow};

const M: u64 = 1000000007;

/// グラフの彩色数を求める。
pub fn chromatic_number<E: EdgeTrait>(graph: &Graph<Undirected, E>) -> usize {
    let n = graph.len();

    let mut g = vec![0; n];
    for e in graph.nodes_iter().flatten() {
        g[e.from()] |= 1 << e.to();
    }

    let mut f = vec![0_u64; 1 << n];
    f[0] = 1;
    for i in 1_usize..(1 << n) {
        let c = i.trailing_zeros();
        f[i] = f[i - (1 << c)] + f[(i - (1 << c)) & !g[c as usize]];
        if f[i] >= M {
            f[i] -= M;
        }
    }

    let check = |k| {
        let mut t = 0;

        for (i, &f) in f.iter().enumerate() {
            let s = mod_pow(f, k as u64, M);
            if s == 0 {
                continue;
            }

            if i.count_ones() % 2 == 1 {
                t += M - s;
            } else {
                t += s;
            }
            if t >= M {
                t -= M;
            }
        }

        t % M != 0
    };

    for i in 1..n {
        if check(i) {
            return i;
        }
    }
    n
}
