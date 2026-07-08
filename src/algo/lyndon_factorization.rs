//! Lyndon分解
//!
//! # References
//! - <https://qiita.com/nakashi18/items/66882bd6e0127174267a>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/lyndon_factorization>

/// 列の最長Lyndon接頭辞を得る。
///
/// 最長Lyndon接頭辞を$p$として、$|p|$と$p$が先頭から連続して繰り返される回数を返す。
pub fn longest_lyndon_prefix<T>(s: &[T]) -> (usize, usize)
where
    T: Ord,
{
    let mut i = 0;
    let mut j = 1;

    while j < s.len() && s[i] <= s[j] {
        if s[i] == s[j] {
            i += 1;
            j += 1;
        } else {
            i = 0;
            j += 1;
        }
    }

    let len = j - i;
    let count = j / len;
    (len, count)
}

/// 列をLyndon分解する。
///
/// Lyndon分解を$s=s_1 s_2 \dots s_n$として、各$i$について$s_i$の開始位置と$|s_i|$の組を返す。
pub fn lyndon_factorize<T>(mut s: &[T]) -> Vec<(usize, usize)>
where
    T: Ord,
{
    let mut ret = vec![];
    let mut pos = 0;

    while !s.is_empty() {
        let (len, count) = longest_lyndon_prefix(s);

        for _ in 0..count {
            ret.push((pos, len));
            s = &s[len..];
            pos += len;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use std::iter::once;

    use super::*;

    #[test]
    fn test() {
        let check = |s: &str, ans: Vec<usize>| {
            let res = lyndon_factorize(s.as_bytes());
            let res: Vec<_> = res
                .into_iter()
                .map(|(p, _)| p)
                .chain(once(s.len()))
                .collect();

            assert_eq!(res, ans);
        };

        check("babaabaab", vec![0, 1, 3, 6, 9]);
        check("ababacaca", vec![0, 8, 9]);
    }
}
