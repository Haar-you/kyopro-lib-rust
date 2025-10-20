//! 符号付き第一種スターリング数$s(0,0), \dots, s(n,n)$を列挙する。
use crate::num::ff::*;

/// 符号付き第一種スターリング数$s(0,0), \dots, s(n,n)$を列挙する。
///
/// **Time complexity** $O(n^2)$
pub fn stirling_first_table<Modulo: FF>(n: usize, modulo: Modulo) -> Vec<Vec<Modulo::Element>>
where
    Modulo::Element: Copy,
{
    let mut ret = vec![vec![modulo.from_u64(0); n + 1]; n + 1];

    ret[0][0] = modulo.from_u64(1);

    for i in 1..=n {
        for j in 1..=i {
            ret[i][j] = -modulo.from_u64(i as u64 - 1) * ret[i - 1][j] + ret[i - 1][j - 1]
        }
    }

    ret
}
