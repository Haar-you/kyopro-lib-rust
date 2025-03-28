//! トーシェント関数
//!
//! # References
//! - <https://ja.wikipedia.org/wiki/%E3%82%AA%E3%82%A4%E3%83%A9%E3%83%BC%E3%81%AE%CF%86%E9%96%A2%E6%95%B0>

/// `n`と互いに素である自然数の個数を求める。
pub fn totient(mut n: u64) -> u64 {
    let mut ret = n;

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            ret -= ret / i;
            while n % i == 0 {
                n /= i;
            }
        }

        i += 1;
    }

    if n != 1 {
        ret -= ret / n;
    }

    ret
}

/// `n`までのトーシェント関数のテーブル$\varphi$を構築する。
///
/// nとmが互いに素のとき、$\varphi(nm) = \varphi(n) \cdot \varphi(m)$
pub fn totient_table(n: usize) -> Vec<u64> {
    let mut ret = (0..=n as u64).collect::<Vec<_>>();

    for i in 2..=n {
        if ret[i] == i as u64 {
            for j in (i..=n).step_by(i) {
                ret[j] = ret[j] / (i as u64) * (i as u64 - 1);
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://oeis.org/A000010/list
        assert_eq!(
            &totient_table(69)[1..],
            vec![
                1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20,
                12, 18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22,
                46, 16, 42, 20, 32, 24, 52, 18, 40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66,
                32, 44
            ]
            .as_slice()
        );
    }
}
