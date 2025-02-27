//! Manacher's algorithm

/// `s`の各要素を中心とした奇数長の最長回文の片側の長さを求める。
///
/// **Time complexity** $O(|s|)$
pub fn manacher<T: PartialEq>(s: &[T]) -> Vec<usize> {
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

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            manacher("abaaababa".as_bytes()),
            [1, 2, 1, 4, 1, 2, 3, 2, 1]
        );
    }
}
