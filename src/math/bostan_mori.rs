//! 線形漸化式で表される数列の第`k`項目を求める。
use crate::math::ntt::NTT;
use crate::num::const_modint::ConstModInt;

/// $a_i = \sum_{j = 1}^d c_j a_{i-j}$を満たす数列$a$の初め$d$項と係数$c$から、数列の第`k`項$a_k$を求める。
pub fn bostan_mori<const P: u32, const PR: u32>(
    a: Vec<ConstModInt<P>>,
    c: Vec<ConstModInt<P>>,
    mut k: u64,
) -> ConstModInt<P> {
    assert_eq!(a.len(), c.len());

    let ntt = NTT::<P, PR>::new();
    let d = a.len();

    let mut q: Vec<ConstModInt<P>> = vec![0.into(); d + 1];
    q[0] = 1.into();
    for i in 0..d {
        q[i + 1] = -c[i];
    }

    let mut p = ntt.convolve(a, q.clone());
    p.truncate(d);

    while k > 0 {
        let mut q1 = q.clone();
        for i in (1..q1.len()).step_by(2) {
            q1[i] = -q1[i];
        }

        let size = (2 * d + 1).next_power_of_two();
        let mut u = p.clone();
        u.resize(size, 0.into());
        ntt.ntt(&mut u);

        q1.resize(size, 0.into());
        ntt.ntt(&mut q1);

        u.iter_mut().zip(q1.iter()).for_each(|(x, y)| *x *= *y);
        ntt.intt(&mut u);

        let mut a = q.clone();
        a.resize(size, 0.into());
        ntt.ntt(&mut a);

        a.iter_mut().zip(q1).for_each(|(x, y)| *x *= y);
        ntt.intt(&mut a);

        if k % 2 == 0 {
            for i in 0..d {
                p[i] = u[i * 2];
            }
        } else {
            for i in 0..d {
                p[i] = u[i * 2 + 1];
            }
        }

        for i in 0..=d {
            q[i] = a[i * 2];
        }

        k >>= 1;
    }

    p[0]
}
