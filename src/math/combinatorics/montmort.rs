//! 完全順列の個数を列挙する。
//!
//! # References
//! - <https://ja.wikipedia.org/wiki/%E5%AE%8C%E5%85%A8%E9%A0%86%E5%88%97>
//! - <https://oeis.org/A000166>

/// 長さ`0`から`n`までの完全順列の個数を列挙する。
///
/// **Time complexity** $O(n)$
pub fn montmort(n: usize, m: u64) -> Vec<u64> {
    let mut ret = vec![0; n + 1];

    if n >= 2 {
        ret[2] = 1;

        for i in 3..=n {
            ret[i] = (i - 1) as u64 * (ret[i - 1] + ret[i - 2]) % m;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(montmort(10, 100)[1..], [0, 1, 2, 9, 44, 65, 54, 33, 96, 61]);
        assert_eq!(
            montmort(20, 998244353)[1..],
            [
                0, 1, 2, 9, 44, 265, 1854, 14833, 133496, 1334961, 14684570, 176214841, 294304226,
                127281753, 910981941, 600290115, 222488424, 11814221, 224470198, 496426549
            ]
        );
    }
}
