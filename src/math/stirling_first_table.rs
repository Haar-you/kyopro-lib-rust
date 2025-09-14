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

#[cfg(test)]
mod tests {
    use crate::{math::stirling_first::stirling_first, num::const_modint::ConstModIntBuilder};

    use super::*;

    #[test]
    fn test() {
        let modulo = ConstModIntBuilder::<998244353>;

        let n = 100;
        let table = stirling_first_table(n, modulo);

        for i in 0..=n {
            let ans = stirling_first::<998244353, 3>(i);

            assert_eq!(&table[i][..=i], ans);
        }
    }
}
