pub fn zalgo<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut ret = vec![0; n];
    let mut j = 0;

    for i in 1..n {
        if i + ret[i - j] < j + ret[j] {
            ret[i] = ret[i - j];
        } else {
            let mut k = if j + ret[j] >= i { j + ret[j] - i } else { 0 };
            while i + k < n && s[k] == s[i + k] {
                k += 1;
            }
            ret[i] = k;
            j = i;
        }
    }

    ret[0] = n;

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(zalgo("abcbcba".as_bytes()), [7, 0, 0, 0, 0, 0, 1]);
        assert_eq!(
            zalgo("mississippi".as_bytes()),
            [11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(zalgo("ababacaca".as_bytes()), [9, 0, 3, 0, 1, 0, 1, 0, 1]);
        assert_eq!(zalgo("aaaaa".as_bytes()), [5, 4, 3, 2, 1]);
    }
}
