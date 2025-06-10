//! 第二種スターリング数$S(0,0), \dots, S(n,n)$を列挙する。
use crate::num::ff::*;

/// 第二種スターリング数$S(0,0), \dots, S(n,n)$を列挙する。
///
/// **Time complexity** $O(n^2)$
pub fn stirling_second_table<Modulo: FF>(n: usize, modulo: Modulo) -> Vec<Vec<Modulo::Element>>
where
    Modulo::Element: Copy,
{
    let mut ret = vec![vec![modulo.from_u64(0); n + 1]; n + 1];

    ret[0][0] = modulo.from_u64(1);

    for i in 1..=n {
        ret[i][1] = modulo.from_u64(1);
        ret[i][i] = modulo.from_u64(1);
    }

    for i in 3..=n {
        for j in 2..i {
            ret[i][j] = ret[i - 1][j - 1] + modulo.from_u64(j as u64) * ret[i - 1][j];
        }
    }

    ret
}
