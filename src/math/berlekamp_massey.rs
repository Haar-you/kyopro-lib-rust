//! 線形漸化式を求める。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/find_linear_recurrence>
use crate::num::ff::*;
use std::iter::zip;
use std::ops::Add;

/// $N$項の数列$a_0, a_1, \ldots, a_{N-1}$から、
/// 最短の線形漸化式$a_i = c_1 a_{i-1} + c_2 a_{i-2} + \dots + c_d a_{i-d}$の係数$c_i$を求める。
pub fn berlekamp_massey<Modulo: FF>(
    mut s: Vec<Modulo::Element>,
    modulo: Modulo,
) -> Vec<Modulo::Element>
where
    Modulo::Element: FFElem + Copy,
{
    let zero = modulo.from_u64(0);
    let one = modulo.from_u64(1);
    let len = s.len();
    let mut c = vec![one];
    let mut p = vec![one];
    let mut l = 0;
    let mut m = 1;
    let mut b = one;

    c.reserve(len);
    s.reverse();

    for n in 0..len {
        let d = s[len - n - 1]
            + zip(c.iter().skip(1), s.iter().skip(len - n))
                .map(|(&c, &s)| c * s)
                .fold(zero, Add::add);

        if d == zero {
            m += 1;
        } else if 2 * l <= n {
            let temp = c.clone();
            if c.len() < p.len() + m {
                c.resize(p.len() + m, zero);
            }
            let t = d / b;

            for (c, p) in c.iter_mut().skip(m).zip(p.iter()) {
                *c -= t * *p;
            }

            l = n + 1 - l;
            p = temp;
            b = d;
            m = 1;
        } else {
            if c.len() < p.len() + m {
                c.resize(p.len() + m, zero);
            }
            let t = d / b;

            for (c, p) in c.iter_mut().skip(m).zip(p.iter()) {
                *c -= t * *p;
            }

            m += 1;
        }
    }

    c.into_iter().skip(1).take(l).map(|x| -x).collect()
}

#[cfg(test)]
mod tests {
    use std::ops::{Add, Mul};

    use crate::{
        iter::collect::CollectVec,
        math::prime_mod::Prime,
        num::{const_modint::ConstModIntBuilder, one_zero::Zero},
    };

    use super::*;

    fn generate<T>(prefix: &[T], coeffs: &[T]) -> Vec<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Zero,
    {
        assert_eq!(prefix.len(), coeffs.len());
        let n = prefix.len();

        let mut ret = prefix.to_vec();

        for _ in 0..n {
            let a = ret
                .iter()
                .rev()
                .zip(coeffs.iter())
                .map(|(&a, &c)| a * c)
                .fold(T::zero(), std::ops::Add::add);

            ret.push(a);
        }

        ret
    }

    #[test]
    fn test() {
        let ff = ConstModIntBuilder::<Prime<998244353>>::new();

        let a = vec![1, 2, 3, 4, 5];
        let a = a.into_iter().map(|x| ff.from_u64(x)).collect_vec();
        let c = vec![2, 3, 2, 8, 5];
        let c = c.into_iter().map(|x| ff.from_u64(x)).collect_vec();

        let a = generate(&a, &c);
        let res = berlekamp_massey(a, ff);

        assert_eq!(res, c);
    }
}
