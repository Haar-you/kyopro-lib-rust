#![allow(clippy::needless_range_loop)]

use crate::math::ff::traits::FF;

pub fn bell_number_table<T: FF + From<usize>>(n: usize) -> Vec<Vec<T>> {
    let mut ret = vec![vec![T::from(0); n + 1]; n + 1];
    ret[0][0] = T::from(1);

    for i in 1..=n {
        ret[i][1] = T::from(1);
        ret[i][i] = T::from(1);
    }

    for i in 3..=n {
        for j in 2..i {
            ret[i][j] = ret[i - 1][j - 1] + T::from(j) * ret[i - 1][j];
        }
    }

    for i in 0..=n {
        for j in 1..=n {
            let t = ret[i][j - 1];
            ret[i][j] += t;
        }
    }

    ret
}
