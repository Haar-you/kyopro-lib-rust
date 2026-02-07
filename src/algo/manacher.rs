//! Manacher's algorithm

/// `s`の各要素を中心とした奇数長の最長回文の長さを求める。
///
/// **Time complexity** $O(|s|)$
pub fn manacher_odd<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut ret = vec![0; n];
    let mut center = 0;

    for cur in 0..n {
        let left: i32 = center as i32 * 2 - cur as i32;

        if left >= 0 && cur + ret[left as usize] < center + ret[center] {
            ret[cur] = ret[left as usize];
        } else {
            let mut len = center + ret[center] - cur;
            while cur >= len && cur + len < n && s[cur - len] == s[cur + len] {
                len += 1;
            }

            ret[cur] = len;
            center = cur;
        }
    }

    ret.into_iter().map(|l| l * 2 - 1).collect()
}

/// `s`の各要素の間を中心とした偶数長の最長回文の長さを求める。
///
/// **Time complexity** $O(|s|)$
pub fn manacher_even<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut t = vec![None; n * 2 - 1];

    for (i, a) in s.iter().enumerate() {
        t[i * 2] = Some(a);
    }

    manacher_odd(&t)
        .into_iter()
        .skip(1)
        .step_by(2)
        .map(|l| (l / 2).div_ceil(2) * 2)
        .collect()
}

/// [`manacher_odd`]と[`manacher_even`]の結果を回文の中心位置の順に並べたものを返す。
///
/// **Time complexity** $O(|s|)$
pub fn manacher<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut odd = manacher_odd(s).into_iter();
    let mut even = manacher_even(s).into_iter();

    let mut ret = vec![];

    for _ in 0..n - 1 {
        ret.push(odd.next().unwrap());
        ret.push(even.next().unwrap());
    }
    ret.push(odd.next().unwrap());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            manacher_odd("abaaababa".as_bytes()),
            [1, 3, 1, 7, 1, 3, 5, 3, 1]
        );
        assert_eq!(
            manacher_even("abaaababa".as_bytes()),
            [0, 0, 2, 2, 0, 0, 0, 0]
        );

        assert_eq!(manacher("aaaaa".as_bytes()), [1, 2, 3, 4, 5, 4, 3, 2, 1]);
    }
}
