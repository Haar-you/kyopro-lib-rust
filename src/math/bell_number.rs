#![allow(clippy::needless_range_loop)]

use crate::math::ff::traits::*;

pub fn bell_number_table<Modulo: FF>(n: usize, modulo: Modulo) -> Vec<Vec<Modulo::Output>>
where
    Modulo::Output: FFElem,
{
    let mut ret = vec![vec![modulo.from_u64(0); n + 1]; n + 1];
    ret[0][0] = modulo.from_u64(1);

    for i in 1..=n {
        ret[i][1] = modulo.from_u64(1);
        ret[i][i] = modulo.from_u64(1);
    }

    for i in 3..=n {
        for j in 2..i {
            ret[i][j] =
                ret[i - 1][j - 1].clone() + modulo.from_u64(j as u64) * ret[i - 1][j].clone();
        }
    }

    for i in 0..=n {
        for j in 1..=n {
            let t = ret[i][j - 1].clone();
            ret[i][j] += t;
        }
    }

    ret
}
