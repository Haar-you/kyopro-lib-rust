//! トーシェント関数の総和
//!
//! # References
//! - <https://yukicoder.me/wiki/sum_totient>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/sum_of_totient_function>

use crate::math::totient::totient_table;
use crate::num::ff::*;
use std::collections::HashMap;

/// トーシェント関数の総和
pub fn totient_sum<Modulo: FF>(n: u64, m: Modulo) -> Modulo::Element
where
    Modulo::Element: FFElem + Copy,
{
    let k = (n as f64).powf(0.66) as u64;

    let mut memo1 = vec![m.from_u64(0); k as usize + 1];
    let table = totient_table(k as usize);
    let mut sum = m.from_u64(0);
    for i in 1..=k as usize {
        sum += m.from_u64(table[i]);
        memo1[i] = sum;
    }

    let mut memo2 = HashMap::new();

    rec(n, &m, &memo1, &mut memo2)
}

fn rec<Modulo: FF>(
    x: u64,
    m: &Modulo,
    memo1: &[Modulo::Element],
    memo2: &mut HashMap<u64, Modulo::Element>,
) -> Modulo::Element
where
    Modulo::Element: FFElem + Copy,
{
    if let Some(y) = memo1.get(x as usize) {
        return *y;
    }
    if let Some(y) = memo2.get(&x) {
        return *y;
    }

    let mut ret = if x % 2 == 0 {
        m.from_u64(x / 2) * m.from_u64(x + 1)
    } else {
        m.from_u64(x) * m.from_u64((x + 1) / 2)
    };

    let s = (x as f64).sqrt() as u64;

    for i in 2..=s {
        if x / i > s {
            ret -= rec(x / i, m, memo1, memo2);
        }
    }

    for i in 1..=s {
        let t = m.from_u64(x / i - x / (i + 1));
        ret -= rec(i, m, memo1, memo2) * t;
    }

    memo2.insert(x, ret);
    ret
}
