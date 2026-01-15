//! 添字積$\pmod {2^N}$畳み込み
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/mul_mod2n_convolution>
use crate::math::convolution::ntt::*;
use crate::math::prime_mod::*;
use crate::num::const_modint::*;

/// $c_k = \sum_{i \times j = k \pmod {2^N}} a_i b_j$を満たす$c$を求める。
///
/// # Requirements
/// `a.len()` = `b.len()` = $2^N$
pub fn convolution_mul_mod2n<P: PrimeMod>(
    a: Vec<ConstModInt<P>>,
    b: Vec<ConstModInt<P>>,
) -> Vec<ConstModInt<P>> {
    let len = a.len();
    assert_eq!(a.len(), b.len());
    assert!(len.is_power_of_two());

    let n = len.trailing_zeros() as usize;

    if n <= 1 {
        let mut ret = vec![0.into(); len];

        for i in 0..len {
            for j in 0..len {
                ret[i * j % len] += a[i] * b[j];
            }
        }

        return ret;
    }

    let ntt = NTT::<P>::new();
    let mask = (1 << n) - 1;
    let cycle = std::iter::successors(Some(1), |n| Some((n * 5) & mask))
        .take(len / 4)
        .collect::<Vec<_>>();

    let mut s: Vec<Vec<Vec<ConstModInt<P>>>> = vec![vec![vec![]; n - 1]; 2];
    let mut t: Vec<Vec<Vec<ConstModInt<P>>>> = vec![vec![vec![]; n - 1]; 2];

    for i in 0..n - 1 {
        let k = n - i;

        s[0][i].resize(1 << (k - 2), 0.into());
        s[1][i].resize(1 << (k - 2), 0.into());
        t[0][i].resize(1 << (k - 2), 0.into());
        t[1][i].resize(1 << (k - 2), 0.into());

        let mask2 = (1 << k) - 1;

        for (j, c) in cycle.iter().enumerate().take(1 << (k - 2)) {
            let r = c & mask2;
            s[0][i][j] = a[r << i];
            t[0][i][j] = b[r << i];
            s[1][i][j] = a[((len - r) & mask2) << i];
            t[1][i][j] = b[((len - r) & mask2) << i];
        }
    }

    let mut ret = vec![0.into(); 1 << n];

    let mut tt0 = vec![vec![]; n];
    let mut tt1 = vec![vec![]; n];

    for i in (0..n - 1).rev() {
        let mut s0 = s[0][i].clone();
        let mut s1 = s[1][i].clone();
        let slen = s[0][i].len();
        ntt.ntt(&mut s0);
        ntt.ntt(&mut s1);

        for j in (0..n - 1).rev() {
            let tlen = t[0][j].len();

            if slen <= 1 || tlen <= 1 {
                for x in 0..slen {
                    for y in 0..tlen {
                        let g = cycle[(x + y) % cycle.len()] << (i + j);
                        ret[g & mask] += s[0][i][x] * t[0][j][y] + s[1][i][x] * t[1][j][y];
                        let mg = if g == 0 { 0 } else { len - g };
                        ret[mg & mask] += s[1][i][x] * t[0][j][y] + s[0][i][x] * t[1][j][y];
                    }
                }

                continue;
            }

            let w = std::cmp::max(slen, tlen);

            if w > slen {
                s0 = s[0][i].clone();
                s1 = s[1][i].clone();
                s0.resize(w, 0.into());
                s1.resize(w, 0.into());
                ntt.ntt(&mut s0);
                ntt.ntt(&mut s1);
            }

            if w > tt0[j].len() {
                tt0[j] = t[0][j].clone();
                tt0[j].resize(w, 0.into());
                tt1[j] = t[1][j].clone();
                tt1[j].resize(w, 0.into());

                ntt.ntt(&mut tt0[j]);
                ntt.ntt(&mut tt1[j]);
            }

            let t0 = &tt0[j];
            let t1 = &tt1[j];

            let mut c = (0..w)
                .map(|k| s0[k] * t0[k] + s1[k] * t1[k])
                .collect::<Vec<_>>();
            ntt.intt(&mut c);

            c.into_iter().zip(cycle.iter()).for_each(|(x, r)| {
                let index = r << (i + j);
                ret[index & mask] += x;
            });

            let mut c = (0..w)
                .map(|k| s1[k] * t0[k] + s0[k] * t1[k])
                .collect::<Vec<_>>();
            ntt.intt(&mut c);

            c.into_iter().zip(cycle.iter()).for_each(|(x, r)| {
                let index = r << (i + j);
                let index = if index == 0 { 0 } else { len - index };
                ret[index & mask] += x;
            });
        }
    }

    ret[0] += a[0] * b[0];
    for i in 1..len {
        ret[0] += a[i] * b[0] + a[0] * b[i];
        ret[len / 2 * (i % 2)] += a[i] * b[len / 2];
        ret[len / 2 * (i % 2)] += a[len / 2] * b[i];
    }
    ret[0] -= a[len / 2] * b[len / 2];

    ret
}
